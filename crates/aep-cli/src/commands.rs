use crate::{
    cli::{Cli, Command, RecordAction},
    guidance, migration, spec, workflow,
};
use aep_core::{Error, Kind, Record, Result, digest, now, unique_id, validate};
use aep_store::{Snapshot, Store, contained, git, parse_yaml, read_optional};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Read,
    path::Path,
};

pub struct Outcome {
    pub data: Value,
    pub text: Option<String>,
    pub changed: bool,
    pub exit_code: i32,
    pub diagnostics: Vec<Value>,
}
impl Outcome {
    pub fn ok(data: Value) -> Self {
        Self {
            data,
            text: None,
            changed: false,
            exit_code: 0,
            diagnostics: vec![],
        }
    }
    pub fn text(text: String, data: Value) -> Self {
        Self {
            text: Some(text),
            ..Self::ok(data)
        }
    }
}
pub fn input<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    let text = if path == Path::new("-") {
        let mut text = String::new();
        std::io::stdin()
            .take(16 * 1024 * 1024 + 1)
            .read_to_string(&mut text)?;
        text
    } else {
        std::fs::read_to_string(path)?
    };
    if text.len() > 16 * 1024 * 1024 {
        return Err(Error::input("Input exceeds 16 MiB"));
    }
    parse_yaml(&text)
}
pub fn record_json(r: &Record) -> Value {
    json!({"record":r,"revision":r.revision()})
}
pub fn store(args: &Cli) -> Result<Store> {
    Store::open(&args.root)
}
pub fn run(args: &Cli) -> Result<Outcome> {
    match &args.command {
        Command::Skills { command } => return guidance::run(args, command.as_ref()),
        Command::Init { claude } => return init(args, *claude),
        Command::Doctor => return doctor(args),
        Command::Migrate { command } => return migration::run(args, command),
        Command::Recover { apply, rollback } => {
            let store = match Store::open(&args.root) {
                Ok(s) => s,
                Err(e) if e.code == "prerequisite" => Store::uninitialized(&args.root)?,
                Err(e) => return Err(e),
            };
            if (*apply || *rollback) && !args.dry_run {
                let id = store.recover(*rollback)?;
                let mut out = Outcome::ok(json!({"recovered":id}));
                out.changed = id.is_some();
                return Ok(out);
            }
            return Ok(Outcome::ok(json!({"transaction":store.recovery()?})));
        }
        _ => {}
    }
    let mut s = store(args)?;
    let snap = s.snapshot()?;
    s.config.context_revision = digest(serde_json::to_vec(
        &snap
            .files
            .iter()
            .filter(|(p, _)| {
                p.starts_with(&format!("{}/", s.config.stores.rules))
                    || (p.starts_with(&format!("{}/changes/", s.config.stores.ledger))
                        && !p.ends_with("/change.yaml"))
                    || ["AGENTS.md", "CLAUDE.md", ".aep/config.toml"].contains(&p.as_str())
            })
            .collect::<BTreeMap<_, _>>(),
    )?);
    match &args.command {
        Command::Config { command } => match command {
            crate::cli::ConfigAction::Show => Ok(Outcome::ok(
                json!({"config":s.config,"revision":digest(snap.files.get(".aep/config.toml").ok_or_else(|| Error::blocked("Missing config"))?)}),
            )),
            crate::cli::ConfigAction::Update { file, expect } => {
                let before = snap
                    .files
                    .get(".aep/config.toml")
                    .ok_or_else(|| Error::blocked("Initialize the project first"))?;
                if digest(before) != *expect {
                    return Err(Error::conflict("Configuration revision changed"));
                }
                let config: aep_core::Config =
                    if file.extension().and_then(|s| s.to_str()) == Some("toml") {
                        toml::from_str(&std::fs::read_to_string(file)?)
                            .map_err(|e| Error::input(e.to_string()))?
                    } else {
                        input(file)?
                    };
                let proposed = Store::with_config(s.root.clone(), config)?;
                if proposed.config.cli_version != aep_core::VERSION
                    || proposed.config.schema_version != aep_core::SCHEMA
                    || proposed.config.openspec_profile != "1.12.0-common"
                {
                    return Err(Error::unsupported(
                        "Configuration requires an unsupported release/schema/profile",
                    ));
                }
                if snap.files.keys().any(|path| {
                    s.roots()
                        .iter()
                        .any(|root| path.starts_with(&format!("{root}/")))
                }) && serde_json::to_value(&proposed.config.stores)?
                    != serde_json::to_value(&s.config.stores)?
                {
                    return Err(Error::blocked(
                        "Moving populated stores requires an explicit migration",
                    ));
                }
                let diagnostics = validate(&snap.records, &proposed.config);
                if !diagnostics.is_empty() {
                    return Err(Error::input(serde_json::to_string(&diagnostics)?));
                }
                let files = BTreeMap::from([(
                    ".aep/config.toml".into(),
                    Some(
                        toml::to_string_pretty(&proposed.config)
                            .map_err(|e| Error::input(e.to_string()))?,
                    ),
                )]);
                let plan = s.plan(&snap, files)?;
                if !args.dry_run {
                    s.apply(&plan)?;
                }
                let mut out = Outcome::ok(
                    json!({"operation_id":plan.id,"source_revision":plan.source_revision,"files":plan.changes.iter().map(|c| &c.path).collect::<Vec<_>>(),"dry_run":args.dry_run}),
                );
                out.changed = !args.dry_run && !plan.changes.is_empty();
                Ok(out)
            }
        },
        Command::Status => Ok(Outcome::ok(
            json!({"revision":snap.revision,"records":snap.records.len(),"stories":snap.records.iter().filter(|r|r.kind==Kind::Story).map(|r|json!({"id":r.id,"status":r.status,"readiness":aep_core::readiness(r,&snap.records,&s.config)})).collect::<Vec<_>>(),"diagnostics":validate(&snap.records,&s.config)}),
        )),
        Command::Query { kind, status } => {
            let kind = kind.as_deref().map(Kind::parse).transpose()?;
            Ok(Outcome::ok(
                json!({"revision":snap.revision,"records":snap.records.iter().filter(|r|kind.is_none_or(|k|k==r.kind)&&status.as_ref().is_none_or(|st|st==&r.status)).map(record_json).collect::<Vec<_>>()}),
            ))
        }
        Command::Context { id, source } => context(&s, &snap, id, source),
        Command::Timeline { id } => {
            let mut records: Vec<_> = snap
                .records
                .iter()
                .filter(|r| {
                    r.kind == Kind::Event && id.as_ref().is_none_or(|id| r.refs.contains(id))
                })
                .collect();
            records.sort_by(|a, b| a.created_at.cmp(&b.created_at).then(a.id.cmp(&b.id)));
            Ok(Outcome::ok(
                json!({"revision":snap.revision,"events":records,"ordering":"recorded_at; occurrence time is unknown unless explicitly recorded"}),
            ))
        }
        Command::Check => check(&s, &snap),
        Command::Story(a) => records(args, &s, &snap, Kind::Story, &a.action),
        Command::Layer(a) => records(args, &s, &snap, Kind::Layer, &a.action),
        Command::Wave(a) => records(args, &s, &snap, Kind::Wave, &a.action),
        Command::Release(a) => records(args, &s, &snap, Kind::Release, &a.action),
        Command::Change(a) => records(args, &s, &snap, Kind::Change, &a.action),
        Command::Decision(a) => records(args, &s, &snap, Kind::Decision, &a.action),
        Command::Roadmap(a) => records(args, &s, &snap, Kind::Roadmap, &a.action),
        Command::Lesson(a) => records(args, &s, &snap, Kind::Lesson, &a.action),
        Command::Rule(a) => records(args, &s, &snap, Kind::Rule, &a.action),
        Command::Gate(a) => records(args, &s, &snap, Kind::Gate, &a.action),
        Command::Reflect {
            command: crate::cli::Reflect::Propose(i),
        } => records(
            args,
            &s,
            &snap,
            Kind::Rule,
            &RecordAction::New(crate::cli::Input {
                file: i.file.clone(),
            }),
        ),
        Command::Verify { command } => workflow::verify(args, &s, &snap, command),
        Command::Dispatch { command } => workflow::dispatch(args, &s, &snap, command),
        Command::Worktree { command } => workflow::worktree(&s, &snap, command),
        Command::Attempt { command } => workflow::attempt(args, &s, &snap, command),
        Command::Review { command } => workflow::review(args, &s, &snap, command),
        Command::Deliver { command } => workflow::deliver(args, &s, &snap, command),
        Command::Spec { command } => spec::run(args, &s, &snap, command),
        Command::Openspec { command } => spec::openspec(args, &s, &snap, command),
        Command::Dashboard => dashboard(&s, &snap),
        _ => Err(Error::input("Unsupported command combination")),
    }
}
pub fn save(
    s: &Store,
    snap: &Snapshot,
    records: Vec<Record>,
    extra: BTreeMap<String, Option<String>>,
    dry_run: bool,
) -> Result<Outcome> {
    s.check_version()?;
    let mut candidate = snap.records.clone();
    let mut writes = extra;
    let mut ids = vec![];
    let mut transitions = vec![];
    for mut r in records {
        let previous = candidate.iter().find(|old| old.id == r.id).cloned();
        if let Some(old) = candidate.iter_mut().find(|old| old.id == r.id) {
            if old.kind != r.kind {
                return Err(Error::input("Record kind cannot change"));
            }
            if old.revision() == r.revision() {
                continue;
            }
            r.created_at = old.created_at.clone();
            r.updated_at = Some(now());
            *old = r.clone();
        } else {
            candidate.push(r.clone());
        }
        transitions.push(json!({"id":r.id,"kind":r.kind,"from":previous.as_ref().map(|r| &r.status),"to":r.status,"before_revision":previous.as_ref().map(Record::revision),"after_revision":r.revision()}));
        ids.push(r.id.clone());
        writes.insert(s.record_path(r.kind, &r.id)?, Some(s.record_bytes(&r)?));
    }
    let diagnostics = validate(&candidate, &s.config);
    if !diagnostics.is_empty() {
        return Err(Error::new(
            "validation",
            serde_json::to_string(&diagnostics)?,
            1,
        ));
    }
    if !ids.is_empty() {
        let mut event = Record::new(Kind::Event, &unique_id("event"), "Records updated");
        event.status = "recorded".into();
        event.refs = ids.clone();
        event.set("transitions", json!(transitions));
        event.set("recorded_at", now());
        event.set(
            "revisions",
            json!(
                candidate
                    .iter()
                    .filter(|r| ids.contains(&r.id))
                    .map(|r| (&r.id, r.revision()))
                    .collect::<BTreeMap<_, _>>()
            ),
        );
        writes.insert(
            s.record_path(event.kind, &event.id)?,
            Some(s.record_bytes(&event)?),
        );
    }
    let plan = s.plan(snap, writes)?;
    let changed = !plan.changes.is_empty();
    if !dry_run {
        s.apply(&plan)?;
    }
    let mut out = Outcome::ok(
        json!({"operation_id":plan.id,"source_revision":plan.source_revision,"records":ids,"files":plan.changes.iter().map(|c|&c.path).collect::<Vec<_>>(),"dry_run":dry_run}),
    );
    out.changed = changed && !dry_run;
    Ok(out)
}
fn init(args: &Cli, claude: bool) -> Result<Outcome> {
    let s = match Store::open(&args.root) {
        Ok(s) => s,
        Err(e) if e.code == "prerequisite" => Store::uninitialized(&args.root)?,
        Err(e) => return Err(e),
    };
    let snap = s.snapshot()?;
    let files = initial_files(&s, &snap, claude)?;
    save(&s, &snap, vec![], files, args.dry_run)
}
pub fn initial_files(
    s: &Store,
    snap: &Snapshot,
    claude: bool,
) -> Result<BTreeMap<String, Option<String>>> {
    let mut files = BTreeMap::new();
    if !snap.files.contains_key(".aep/config.toml") {
        files.insert(
            ".aep/config.toml".into(),
            Some(toml::to_string_pretty(&s.config).map_err(|e| Error::input(e.to_string()))?),
        );
    }
    let index = format!("{}/README.md", s.config.stores.rules);
    if !snap.files.contains_key(&index) {
        files.insert(index,Some("# Project rules\n\nProject-specific maintenance rules belong here. Index each rule by task or path, purpose, and file. Add code, testing, package, DevOps, and release rules when the project needs them. Link executable settings instead of duplicating command flags.\n".into()));
    }
    let entry = guidance::asset("templates/AGENTS.md")?
        .replace("project-rules/", &format!("{}/", s.config.stores.rules));
    let agents = read_optional(&contained(&s.root, "AGENTS.md")?)?;
    let merged = match agents {
        None => entry.clone(),
        Some(old) if old.contains("<!-- aep-cli-entrypoint: 5.0 -->") => old,
        Some(old) => format!(
            "{}\n\n{}",
            old.trim_end(),
            guidance::asset("templates/entrypoint.md")?
                .replace("project-rules/", &format!("{}/", s.config.stores.rules))
        ),
    };
    files.insert("AGENTS.md".into(), Some(merged));
    if claude {
        let previous = read_optional(&contained(&s.root, "CLAUDE.md")?)?.unwrap_or_default();
        if !previous.lines().any(|l| l.trim() == "@AGENTS.md") {
            files.insert(
                "CLAUDE.md".into(),
                Some(format!(
                    "{}{}@AGENTS.md\n",
                    previous,
                    if previous.is_empty() || previous.ends_with('\n') {
                        ""
                    } else {
                        "\n"
                    }
                )),
            );
        }
    }
    let mut ignore = read_optional(&contained(&s.root, ".gitignore")?)?.unwrap_or_default();
    for pattern in [".aep/*", "!.aep/config.toml"] {
        if !ignore.lines().any(|line| line == pattern) {
            if !ignore.is_empty() && !ignore.ends_with('\n') {
                ignore.push('\n');
            }
            ignore.push_str(pattern);
            ignore.push('\n');
        }
    }
    files.insert(".gitignore".into(), Some(ignore));
    Ok(files)
}
fn doctor(args: &Cli) -> Result<Outcome> {
    fn available(program: &str) -> bool {
        std::env::var_os("PATH").is_some_and(|paths| {
            std::env::split_paths(&paths).any(|path| {
                let path = path.join(program);
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    path.metadata()
                        .is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
                }
                #[cfg(not(unix))]
                {
                    path.is_file()
                }
            })
        })
    }
    let git_version = workflow::execute(&["git".into(), "--version".into()], &args.root, 5)
        .ok()
        .filter(|r| r.code == Some(0))
        .map(|r| r.stdout.trim().to_string());
    let project = Store::open(&args.root);
    let detail = match &project {
        Ok(s) => {
            json!({"root":s.root,"config":s.config,"compatible":s.check_version().is_ok(),"recovery_required":s.recovery()?.is_some()})
        }
        Err(e) => json!({"diagnostic":e}),
    };
    let configured = project.as_ref().is_ok_and(|s| s.check_version().is_ok());
    let native_platform = cfg!(any(target_os = "linux", target_os = "macos"));
    Ok(Outcome::ok(
        json!({"cli_version":aep_core::VERSION,"schema_version":aep_core::SCHEMA,"guidance_digest":guidance::bundle_digest(),"git":git_version,"project":detail,"platform":{"os":std::env::consts::OS,"arch":std::env::consts::ARCH,"native_target":native_platform},"capabilities":{
            "skills":{"available":true,"requires":[]},
            "records":{"available":configured,"requires":["compatible initialized Git project"]},
            "worktrees":{"available":configured && git_version.is_some() && native_platform,"requires":["Git","Linux or macOS","committed design and dependency integrations"]},
            "verify":{"available":configured && native_platform,"requires":["bound candidate worktree","configured project commands and their toolchains"]},
            "github_delivery":{"executable_present":available("gh"),"requires":["gh","repository authentication","current checks and review"],"authentication_checked":false},
            "openspec_reference":{"executable_present":available("openspec"),"requires":["OpenSpec 1.12.0"],"version_checked":false}
        },"openspec_cli_required":false,"llm_required":false}),
    ))
}
pub fn check(s: &Store, snap: &Snapshot) -> Result<Outcome> {
    let mut diagnostics: Vec<Value> = validate(&snap.records, &s.config)
        .iter()
        .map(|d| json!(d))
        .collect();
    if let Err(e) = s.check_version() {
        diagnostics.push(json!(e));
    }
    for r in &snap.records {
        for path in &r.paths {
            if let Err(e) = contained(&s.root, path) {
                diagnostics.push(json!({"record":r.id,"code":e.code,"message":e.message}));
            }
        }
    }
    for d in spec::check_all(s, snap)? {
        diagnostics.push(d);
    }
    let mut out = Outcome::ok(
        json!({"revision":snap.revision,"records":snap.records.len(),"pass":diagnostics.is_empty(),"diagnostics":diagnostics}),
    );
    if !diagnostics.is_empty() {
        out.exit_code = 1;
        out.diagnostics = diagnostics;
    }
    Ok(out)
}
fn context(s: &Store, snap: &Snapshot, id: &str, sources: &[String]) -> Result<Outcome> {
    let mut pending = vec![id.to_string()];
    let mut seen = BTreeSet::new();
    let mut records = vec![];
    let mut unknowns = vec![];
    while let Some(id) = pending.pop() {
        if !seen.insert(id.clone()) {
            continue;
        }
        match snap.get(&id) {
            Ok(r) => {
                pending.extend(r.links().iter().map(|s| s.to_string()));
                records.push(record_json(r));
            }
            Err(_) => unknowns.push(id),
        }
    }
    let mut files = vec![];
    for source in sources {
        let path = contained(&s.root, source)?;
        let text = std::fs::read_to_string(path)?;
        if text.len() > 1024 * 1024 {
            return Err(Error::input(
                "Context source exceeds 1 MiB; narrow the source",
            ));
        }
        files.push(json!({"path":source,"digest":digest(&text),"content":text}));
    }
    Ok(Outcome::ok(
        json!({"repository":s.root,"head":git(&s.root,&["rev-parse","HEAD"]).ok(),"revision":snap.revision,"subject":id,"records":records,"sources":files,"missing_references":unknowns,"scope":"Explicit record links and requested files; agent selects additional context"}),
    ))
}
fn records(
    args: &Cli,
    s: &Store,
    snap: &Snapshot,
    kind: Kind,
    action: &RecordAction,
) -> Result<Outcome> {
    match action {
        RecordAction::List => {
            return Ok(Outcome::ok(json!(
                snap.records
                    .iter()
                    .filter(|r| r.kind == kind)
                    .map(record_json)
                    .collect::<Vec<_>>()
            )));
        }
        RecordAction::Show { id } => {
            let r = snap.get(id)?;
            if r.kind != kind {
                return Err(Error::input("Record kind mismatch"));
            }
            return Ok(Outcome::ok(record_json(r)));
        }
        RecordAction::Find { query } => {
            let needle = query.to_lowercase();
            let mut matches: Vec<Value> = snap
                .records
                .iter()
                .filter(|r| {
                    r.kind == kind
                        && format!("{} {}", r.title, r.description)
                            .to_lowercase()
                            .contains(&needle)
                })
                .map(record_json)
                .collect();
            if kind == Kind::Lesson {
                for (path, content) in snap.files.iter().filter(|(p, text)| {
                    p.starts_with(&format!("{}/", s.config.stores.lessons))
                        && !snap
                            .records
                            .iter()
                            .filter(|r| r.kind == Kind::Lesson)
                            .any(|r| {
                                s.record_path(r.kind, &r.id)
                                    .as_ref()
                                    .is_ok_and(|path| path == *p)
                            })
                        && text.to_lowercase().contains(&needle)
                }) {
                    matches.push(json!({"kind":"imported_lesson_source","source":path,"digest":digest(content),"content":content,"occurred_at":null}));
                }
            }
            return Ok(Outcome::ok(json!(matches)));
        }
        RecordAction::Check => return check(s, snap),
        RecordAction::Promote {
            id,
            environment,
            by,
        } if kind == Kind::Release => {
            return workflow::promote_release(args, s, snap, id, environment, by);
        }
        RecordAction::Reconcile { id, commit, by } if kind == Kind::Story => {
            return workflow::reconcile_import(args, s, snap, id, commit, by);
        }
        RecordAction::Evaluate { id } if kind == Kind::Gate => {
            return workflow::evaluate_gate(args, s, snap, id);
        }
        _ => {}
    }
    let record = match action {
        RecordAction::Reopen { id } if kind == Kind::Story => {
            let mut r = subject(snap, id, kind)?;
            if !["integrated", "released", "imported", "cancelled"].contains(&r.status.as_str()) {
                return Err(Error::blocked(
                    "Reopen requires a completed, imported, or cancelled story",
                ));
            }
            if let Some(Value::String(delivery)) = r.data.remove("delivery") {
                r.refs.push(delivery);
            }
            r.data.remove("pr_url");
            r.status = "pending".into();
            r
        }

        RecordAction::New(i) | RecordAction::Record(i) => {
            let mut r: Record = input(&i.file)?;
            if r.kind != kind || kind.protected() {
                return Err(Error::input("Record kind mismatch or protected record"));
            }
            if snap.records.iter().any(|old| old.id == r.id) {
                return Err(Error::conflict("Record already exists"));
            }
            if !["pending", "draft"].contains(&r.status.as_str()) {
                return Err(Error::blocked(
                    "Create a pending/draft record; use an explicit operation to accept or complete it",
                ));
            }
            r.created_at = Some(now());
            r.updated_at = r.created_at.clone();
            r
        }
        RecordAction::Update { id, file, expect } => {
            let old = snap.get(id)?;
            if old.kind != kind {
                return Err(Error::input("Record kind mismatch"));
            }
            if old.revision() != *expect {
                return Err(Error::conflict("Record revision changed"));
            }
            let r: Record = input(file)?;
            if r.id != *id
                || r.kind != kind
                || r.status != old.status
                || r.created_at != old.created_at
            {
                return Err(Error::input(
                    "Update preserves ID, kind, status, and creation time",
                ));
            }
            if !["pending", "draft"].contains(&old.status.as_str()) {
                return Err(Error::blocked(
                    "Accepted records are immutable; create a superseding record",
                ));
            }
            r
        }
        RecordAction::Accept { id, by }
            if [Kind::Change, Kind::Decision, Kind::Roadmap].contains(&kind) =>
        {
            let mut r = subject(snap, id, kind)?;
            if !["pending", "draft"].contains(&r.status.as_str())
                || r.description.trim().is_empty()
                || by.trim().is_empty()
            {
                return Err(Error::blocked(
                    "Acceptance requires a draft, design content, and attribution",
                ));
            }
            if kind == Kind::Change {
                spec::validate_change(s, snap, &r)?;
            }
            r.status = "accepted".into();
            r.set("accepted_by", by.clone());
            r.set("acceptance_class", "attributed_decision");
            r
        }
        RecordAction::Supersede { id, by } if [Kind::Decision, Kind::Roadmap].contains(&kind) => {
            let mut r = subject(snap, id, kind)?;
            let next = subject(snap, by, kind)?;
            if r.id == next.id || next.status != "accepted" {
                return Err(Error::blocked(
                    "Superseding record must be a different accepted decision",
                ));
            }
            r.status = "superseded".into();
            r.refs.push(by.clone());
            r.set("superseded_by", by.clone());
            r
        }
        RecordAction::Close { id } if kind == Kind::Change => {
            let mut r = subject(snap, id, kind)?;
            let stories: Vec<_> = snap
                .records
                .iter()
                .filter(|x| x.change_ids().contains(&id.as_str()))
                .collect();
            if stories.is_empty()
                || stories
                    .iter()
                    .any(|x| !["integrated", "released"].contains(&x.status.as_str()))
                || r.status != "published"
            {
                return Err(Error::blocked(
                    "Close requires published specifications and integrated linked stories",
                ));
            }
            r.status = "closed".into();
            r
        }
        RecordAction::Adopt { id, by, evidence } if kind == Kind::Rule => {
            let mut r = subject(snap, id, kind)?;
            if !["pending", "draft"].contains(&r.status.as_str())
                || by.trim().is_empty()
                || r.description.trim().is_empty()
            {
                return Err(Error::blocked(
                    "Adoption requires a draft rule and attribution",
                ));
            }
            let path = s.record_path(kind, id)?;
            let current = snap
                .files
                .get(&path)
                .ok_or_else(|| Error::input("Missing proposal source"))?;
            let mut accepted = vec![];
            for evidence_id in evidence {
                let e = subject(snap, evidence_id, Kind::Evidence)?;
                if !s.config.checks.iter().any(|check| {
                    e.text("check_id") == Some(check.id.as_str()) && check.rules.contains(id)
                }) {
                    return Err(Error::blocked(
                        "Check configuration must explicitly name this rule proposal as an evaluation target",
                    ));
                }
                let story = e
                    .refs
                    .iter()
                    .filter_map(|id| snap.get(id).ok())
                    .find(|r| r.kind == Kind::Story)
                    .ok_or_else(|| Error::blocked("Rule validation evidence has no story"))?;
                let (plan, missing, _) = workflow::evidence_requirements(s, snap, story)?;
                if e.status != "pass"
                    || e.text("fingerprint") != Some(plan.fingerprint.as_str())
                    || !missing.is_empty()
                    || !["integrated", "released"].contains(&story.status.as_str())
                {
                    return Err(Error::blocked(
                        "Rule adoption needs current integrated check and review evidence",
                    ));
                }
                if aep_store::git_bytes(&s.root, &["show", &format!("{}:{path}", plan.head)])?
                    .as_slice()
                    != current.as_bytes()
                {
                    return Err(Error::blocked(
                        "Validation candidate does not contain this exact rule proposal",
                    ));
                }
                accepted.push(evidence_id.clone());
            }
            if accepted.is_empty() {
                return Err(Error::blocked(
                    "Supply --evidence for checks that evaluated the committed rule proposal",
                ));
            }
            r.status = "adopted".into();
            r.set("adopted_by", by.clone());
            r.set("proposal_digest", digest(current));
            r.set("adoption_evidence", json!(accepted));
            r.refs.extend(accepted);
            r
        }
        RecordAction::Retire { id } if kind == Kind::Rule => {
            let mut r = subject(snap, id, kind)?;
            r.status = "retired".into();
            r
        }
        _ => {
            return Err(Error::input(format!(
                "Operation is not supported for {}",
                kind.name()
            )));
        }
    };
    save(s, snap, vec![record], BTreeMap::new(), args.dry_run)
}
pub fn subject(snap: &Snapshot, id: &str, kind: Kind) -> Result<Record> {
    let r = snap.get(id)?;
    if r.kind != kind {
        return Err(Error::input(format!("{id} is not {}", kind.name())));
    }
    Ok(r.clone())
}
fn dashboard(s: &Store, snap: &Snapshot) -> Result<Outcome> {
    Ok(Outcome::ok(
        json!({"schema_version":1,"revision":snap.revision,"records":snap.records,"readiness":snap.records.iter().filter(|r|r.kind==Kind::Story).map(|r|aep_core::readiness(r,&snap.records,&s.config)).collect::<Vec<_>>(),"diagnostics":validate(&snap.records,&s.config)}),
    ))
}
