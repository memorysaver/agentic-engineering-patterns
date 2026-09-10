use crate::{
    cli::{self, Cli},
    commands::{Outcome, input, record_json, save, subject},
};
use aep_core::{Error, Kind, Record, Result, digest, now, path_overlap, unique_id};
use aep_store::{Snapshot, Store, contained, git};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Read,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};

static INTERRUPTED: AtomicBool = AtomicBool::new(false);
pub fn install_interrupt_handler() {
    let _ = ctrlc::set_handler(|| INTERRUPTED.store(true, Ordering::SeqCst));
}
#[derive(Debug, Serialize)]
pub struct ProcessResult {
    pub code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub truncated: bool,
    pub timed_out: bool,
    pub cancelled: bool,
}
fn drain(mut reader: impl Read) -> (String, bool) {
    let mut output = vec![];
    let mut chunk = [0; 8192];
    let mut truncated = false;
    loop {
        match reader.read(&mut chunk) {
            Ok(0) | Err(_) => break,
            Ok(n) => {
                let keep = n.min(65536 - output.len());
                output.extend_from_slice(&chunk[..keep]);
                truncated |= keep < n;
            }
        }
    }
    (String::from_utf8_lossy(&output).into_owned(), truncated)
}
fn terminate_group(child: &mut std::process::Child) {
    #[cfg(unix)]
    unsafe {
        libc::kill(-(child.id() as i32), libc::SIGKILL);
    }
    let _ = child.kill();
}
pub fn execute(argv: &[String], cwd: &Path, seconds: u64) -> Result<ProcessResult> {
    if argv.is_empty() || seconds == 0 {
        return Err(Error::input("Command and positive timeout are required"));
    }
    let mut cmd = Command::new(&argv[0]);
    cmd.args(&argv[1..])
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }
    let mut child = cmd.spawn()?;
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();
    let out = std::thread::spawn(move || drain(stdout));
    let err = std::thread::spawn(move || drain(stderr));
    let start = Instant::now();
    let mut timed_out = false;
    let mut cancelled = false;
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        cancelled = INTERRUPTED.load(Ordering::SeqCst);
        timed_out = start.elapsed() >= Duration::from_secs(seconds);
        if cancelled || timed_out {
            terminate_group(&mut child);
            break child.wait()?;
        }
        std::thread::sleep(Duration::from_millis(20));
    };
    // A check owns its subprocess group, including descendants holding output pipes.
    #[cfg(unix)]
    unsafe {
        libc::kill(-(child.id() as i32), libc::SIGKILL);
    }
    let (stdout, ot) = out
        .join()
        .map_err(|_| Error::new("process", "stdout reader failed", 6))?;
    let (stderr, et) = err
        .join()
        .map_err(|_| Error::new("process", "stderr reader failed", 6))?;
    Ok(ProcessResult {
        code: status.code(),
        stdout,
        stderr,
        truncated: ot || et,
        timed_out,
        cancelled,
    })
}
fn run_git(root: &Path, args: &[&str]) -> Result<String> {
    git(root, args)
}
pub fn active_attempt(snap: &Snapshot, story: &str) -> Result<Record> {
    let matches: Vec<_> = snap
        .records
        .iter()
        .filter(|r| {
            r.kind == Kind::Attempt
                && r.refs.contains(&story.into())
                && ["prepared", "running", "review"].contains(&r.status.as_str())
        })
        .collect();
    if matches.len() != 1 {
        return Err(Error::blocked(format!(
            "Expected one active attempt for {story}; found {}",
            matches.len()
        )));
    }
    Ok(matches[0].clone())
}
fn verification_attempt(snap: &Snapshot, story: &Record) -> Result<Record> {
    if ["integrated", "released"].contains(&story.status.as_str()) {
        let receipt = snap.get(
            story
                .text("delivery")
                .ok_or_else(|| Error::blocked("Missing delivery"))?,
        )?;
        subject(
            snap,
            receipt
                .text("attempt")
                .ok_or_else(|| Error::blocked("Delivery has no attempt"))?,
            Kind::Attempt,
        )
    } else {
        active_attempt(snap, &story.id)
    }
}
pub fn reconcile_import(
    args: &Cli,
    s: &Store,
    snap: &Snapshot,
    id: &str,
    commit: &str,
    by: &str,
) -> Result<Outcome> {
    let mut story = subject(snap, id, Kind::Story)?;
    if story.status != "imported"
        || ![Some("completed"), Some("done")].contains(&story.text("legacy_status"))
        || by.trim().is_empty()
    {
        return Err(Error::blocked(
            "Reconciliation requires an imported completion claim and attribution after inspecting the code",
        ));
    }
    let head = git(
        &s.root,
        &["rev-parse", "--verify", &format!("{commit}^{{commit}}")],
    )?;
    git(&s.root, &["merge-base", "--is-ancestor", &head, "HEAD"]).map_err(|_| {
        Error::blocked("Imported integration must be present in the current Git history")
    })?;
    let mut attempt = Record::new(
        Kind::Attempt,
        &unique_id("import-attempt"),
        "Imported implementation inspection",
    );
    attempt.status = "done".into();
    attempt.refs = vec![id.into()];
    attempt.owner = Some(by.into());
    let path = aep_store::destination(&s.root.join(".aep/worktrees").join(&attempt.id))?;
    let branch = format!("aep/import-{}", attempt.id);
    attempt.set("base", head.clone());
    attempt.set("branch", branch.clone());
    attempt.set("worktree", path.to_string_lossy().to_string());
    let mut receipt = Record::new(
        Kind::Delivery,
        &unique_id("import-delivery"),
        "Inspected legacy integration",
    );
    receipt.status = "integrated".into();
    receipt.refs = vec![id.into()];
    receipt.set("attempt", attempt.id.clone());
    receipt.set("head", head.clone());
    receipt.set("integration_head", head.clone());
    receipt.set("evidence_class", "attributed_git_inspection");
    receipt.set("inspected_by", by.to_string());
    story.status = "integrated".into();
    story.set("delivery", receipt.id.clone());
    if args.dry_run {
        return save(
            s,
            snap,
            vec![attempt, receipt, story],
            BTreeMap::new(),
            true,
        );
    }
    // Git worktree creation precedes a record plan; a failed save reports its explicit side effect.
    std::fs::create_dir_all(path.parent().unwrap())?;
    git(
        &s.root,
        &[
            "worktree",
            "add",
            "-b",
            &branch,
            &path.to_string_lossy(),
            &head,
        ],
    )?;
    save(
        s,
        snap,
        vec![attempt, receipt, story],
        BTreeMap::new(),
        false,
    )
    .map_err(Error::changed)
}
pub fn attempt_path(s: &Store, attempt: &Record) -> Result<PathBuf> {
    let path = PathBuf::from(
        attempt
            .text("worktree")
            .ok_or_else(|| Error::input("Attempt has no worktree"))?,
    );
    let canonical = std::fs::canonicalize(&path)
        .map_err(|_| Error::blocked("Attempt worktree is unavailable"))?;
    let actual = aep_store::repository(&canonical)?;
    if actual != canonical || canonical == s.root {
        return Err(Error::blocked(
            "Attempt is not isolated at the recorded worktree root",
        ));
    }
    let common = git(
        &canonical,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )?;
    if common
        != git(
            &s.root,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?
    {
        return Err(Error::blocked("Attempt belongs to another repository"));
    }
    let branch = git(&canonical, &["branch", "--show-current"])?;
    if Some(branch.as_str()) != attempt.text("branch") {
        return Err(Error::blocked("Attempt branch changed"));
    }
    let base = attempt
        .text("base")
        .ok_or_else(|| Error::input("Attempt has no base"))?;
    git(&canonical, &["merge-base", "--is-ancestor", base, "HEAD"])
        .map_err(|_| Error::blocked("Attempt no longer contains its base"))?;
    Ok(canonical)
}
fn changed_paths(path: &Path, base: &str, head: &str) -> Result<Vec<String>> {
    Ok(git(
        path,
        &[
            "-c",
            "core.quotePath=false",
            "diff",
            "--name-only",
            base,
            head,
            "--",
        ],
    )?
    .lines()
    .map(String::from)
    .collect())
}
fn ensure_clean_code(path: &Path) -> Result<()> {
    if !git(path, &["status", "--porcelain"])?.is_empty() {
        return Err(Error::blocked(
            "Commit the implementation before recording revision-bound evidence",
        ));
    }
    Ok(())
}
#[derive(Debug, Serialize)]
pub struct VerificationPlan {
    pub story: String,
    pub attempt: String,
    pub head: String,
    pub fingerprint: String,
    pub risk: String,
    pub independent_review: bool,
    pub checks: Vec<String>,
    pub paths: Vec<String>,
}
fn linked_context(s: &Store, snap: &Snapshot, story: &Record) -> Result<BTreeMap<String, String>> {
    let mut pending = story.refs.clone();
    pending.extend(story.change_ids().into_iter().map(str::to_string));
    let mut seen = BTreeSet::new();
    let mut files = BTreeMap::new();
    while let Some(id) = pending.pop() {
        if !seen.insert(id.clone()) {
            continue;
        }
        let record = snap.get(&id)?;
        if !matches!(
            record.kind,
            Kind::Change | Kind::Decision | Kind::Roadmap | Kind::Rule | Kind::Lesson
        ) {
            continue;
        }
        pending.extend(record.refs.iter().cloned());
        if record.kind != Kind::Change {
            let path = s.record_path(record.kind, &record.id)?;
            let content = snap
                .files
                .get(&path)
                .ok_or_else(|| Error::input("Linked context file is missing"))?;
            files.insert(path, digest(content));
        }
    }
    Ok(files)
}
pub fn verification_plan(s: &Store, snap: &Snapshot, story: &Record) -> Result<VerificationPlan> {
    let integrated = ["integrated", "released"].contains(&story.status.as_str());
    let (attempt, head) = if integrated {
        let receipt = subject(
            snap,
            story
                .text("delivery")
                .ok_or_else(|| Error::blocked("Missing integration receipt"))?,
            Kind::Delivery,
        )?;
        let attempt = subject(
            snap,
            receipt
                .text("attempt")
                .ok_or_else(|| Error::blocked("Delivery has no attempt"))?,
            Kind::Attempt,
        )?;
        if receipt.status != "integrated"
            || attempt.status != "done"
            || !receipt.refs.contains(&story.id)
        {
            return Err(Error::blocked(
                "Integration receipt does not complete this attempt",
            ));
        }
        if receipt.data.get("tree_verified") == Some(&Value::Bool(false)) {
            return Err(Error::blocked(
                "Integrated tree differs from or cannot be compared to the verified candidate; inspect and reopen the story for current validation",
            ));
        }
        let head = receipt
            .text("head")
            .ok_or_else(|| Error::blocked("Delivery has no verified head"))?
            .to_string();
        git(&s.root, &["cat-file", "-e", &format!("{head}^{{commit}}")])?;
        (attempt, head)
    } else {
        let attempt = active_attempt(snap, &story.id)?;
        let path = attempt_path(s, &attempt)?;
        ensure_clean_code(&path)?;
        let head = git(&path, &["rev-parse", "HEAD"])?;
        (attempt, head)
    };
    let paths = changed_paths(
        &s.root,
        attempt
            .text("base")
            .ok_or_else(|| Error::input("Attempt has no base"))?,
        &head,
    )?;
    for path in &paths {
        if !story.paths.iter().any(|scope| path_overlap(scope, path)) {
            return Err(Error::blocked(format!(
                "Changed path is outside story scope: {path}"
            )));
        }
    }
    let changes: Vec<&Record> = story
        .change_ids()
        .into_iter()
        .map(|id| snap.get(id))
        .collect::<Result<_>>()?;
    let docs_only = !paths.is_empty()
        && paths
            .iter()
            .all(|p| p.ends_with(".md") || p.ends_with(".txt"));
    let sensitive = paths.iter().any(|p| {
        s.config
            .policy
            .protected_paths
            .iter()
            .any(|prefix| path_overlap(p, prefix))
    });
    let contract = changes.iter().any(|c| {
        c.data
            .get("specs")
            .and_then(Value::as_array)
            .is_some_and(|v| !v.is_empty())
    });
    let risk = if sensitive || story.risk == "deep" {
        "deep"
    } else if story.risk == "light" && docs_only && !contract {
        "light"
    } else {
        "standard"
    };
    let mut checks: BTreeSet<String> = story
        .required_checks
        .iter()
        .chain(&s.config.policy.required_checks)
        .cloned()
        .collect();
    for c in &s.config.checks {
        if c.required
            && (c.paths.is_empty()
                || c.paths
                    .iter()
                    .any(|prefix| paths.iter().any(|p| path_overlap(prefix, p))))
        {
            checks.insert(c.id.clone());
        }
    }
    for id in &checks {
        if !s.config.checks.iter().any(|c| &c.id == id) {
            return Err(Error::blocked(format!("Unknown check {id}")));
        }
    }
    let rules: BTreeMap<_, _> = snap
        .files
        .iter()
        .filter(|(p, _)| {
            p.starts_with(&format!("{}/", s.config.stores.rules))
                || [".aep/config.toml", "AGENTS.md", "CLAUDE.md"].contains(&p.as_str())
        })
        .collect();
    // Operational status, timestamps and delivery URLs do not change accepted intent.
    // Referenced contract bytes do: header-only hashes miss edited BDD scenarios.
    let mut story_contract = story.clone();
    story_contract.status.clear();
    story_contract.created_at = None;
    story_contract.updated_at = None;
    for key in ["delivery", "pr_url", "release_record"] {
        story_contract.data.remove(key);
    }
    let change_contracts: Vec<Record> = changes
        .iter()
        .map(|change| {
            let mut c = (*change).clone();
            c.status.clear();
            c.created_at = None;
            c.updated_at = None;
            for key in ["publication", "published_at", "publication_evidence"] {
                c.data.remove(key);
            }
            c
        })
        .collect();
    let contract_files: BTreeMap<_, _> = snap
        .files
        .iter()
        .filter(|(p, _)| {
            changes.iter().any(|c| {
                p.starts_with(&format!("{}/changes/{}/", s.config.stores.ledger, c.id))
                    && !p.ends_with("/change.yaml")
            })
        })
        .collect();
    let mut declared_contracts = BTreeMap::new();
    for change in &changes {
        for c in crate::spec::contracts(change)? {
            let bytes = snap
                .files
                .get(&c.path)
                .ok_or_else(|| Error::input("Delta must be in a configured context store"))?;
            declared_contracts.insert(c.path, digest(bytes));
        }
    }
    let fingerprint = digest(serde_json::to_vec(
        &json!({"head":head,"story":story_contract,"changes":change_contracts,"contracts":contract_files,"declared_contracts":declared_contracts,"linked_context":linked_context(s,snap,story)?,"rules":rules,"checks":s.config.checks,"risk":risk,"base":attempt.text("base")}),
    )?);
    Ok(VerificationPlan {
        story: story.id.clone(),
        attempt: attempt.id,
        head,
        fingerprint,
        risk: risk.into(),
        independent_review: s.config.policy.independent_review,
        checks: checks.into_iter().collect(),
        paths,
    })
}
pub fn verify(args: &Cli, s: &Store, snap: &Snapshot, command: &cli::Verify) -> Result<Outcome> {
    let (id, selected) = match command {
        cli::Verify::Plan { story } => (story, None),
        cli::Verify::Run { story, check } => (story, Some(check)),
    };
    let story = subject(snap, id, Kind::Story)?;
    let plan = verification_plan(s, snap, &story)?;
    if selected.is_none() || args.dry_run {
        return Ok(Outcome::ok(json!(plan)));
    }
    s.check_version()?;
    let selected = selected.unwrap();
    let ids = if selected.is_empty() {
        plan.checks.clone()
    } else {
        selected.clone()
    };
    if ids.is_empty() {
        return Err(Error::blocked(
            "No checks configured; define project checks before verification",
        ));
    }
    let attempt = verification_attempt(snap, &story)?;
    let path = attempt_path(s, &attempt)?;
    ensure_clean_code(&path)?;
    if git(&path, &["rev-parse", "HEAD"])? != plan.head {
        return Err(Error::blocked(
            "Validation worktree is not at the recorded verification head",
        ));
    }
    let mut receipts = vec![];
    let mut failed = false;
    for id in ids {
        let c = s
            .config
            .checks
            .iter()
            .find(|c| c.id == id)
            .ok_or_else(|| Error::input(format!("Unknown check {id}")))?;
        let cwd = contained(&path, &c.cwd)?;
        let mut r = Record::new(Kind::Evidence, &unique_id("check"), &format!("Check {id}"));
        r.refs = vec![story.id.clone(), attempt.id.clone()];
        r.set("check_id", id);
        r.set("head", plan.head.clone());
        r.set("fingerprint", plan.fingerprint.clone());
        r.set("environment", c.environment.clone());
        r.set("started_at", now());
        r.set("command", json!(c.command));
        let missing: Vec<_> = c
            .env
            .iter()
            .filter(|key| std::env::var_os(key).is_none())
            .collect();
        if !missing.is_empty() {
            r.status = "blocked".into();
            r.set("missing_environment", json!(missing));
            failed = true;
        } else {
            match execute(&c.command, &cwd, c.timeout_seconds) {
                Ok(result) => {
                    r.status =
                        if result.code == Some(0) && !result.cancelled && !result.timed_out {
                            "pass"
                        } else {
                            "fail"
                        }
                        .into();
                    failed |= r.status != "pass";
                    r.set("result", json!(result));
                }
                Err(e) => {
                    r.status = "blocked".into();
                    r.set("execution_error", json!(e));
                    failed = true;
                }
            }
        }
        r.set("finished_at", now());
        receipts.push(r);
        if INTERRUPTED.load(Ordering::SeqCst) {
            break;
        }
    }
    // Check processes may have changed inputs. Preserve the evidence, but never certify drift.
    let fresh = s.snapshot().map_err(Error::changed)?;
    let current = verification_plan(s, &fresh, &subject(&fresh, &story.id, Kind::Story)?);
    let stale = current
        .as_ref()
        .map_or(true, |p| p.fingerprint != plan.fingerprint)
        || ensure_clean_code(&path).is_err()
        || git(&path, &["rev-parse", "HEAD"]).as_ref().ok() != Some(&plan.head);
    for r in &mut receipts {
        if stale {
            r.status = "stale".into();
            r.set("stale_reason", "Inputs changed during verification");
        }
    }
    let check_results = json!(receipts);
    let mut out = save(s, &fresh, receipts, BTreeMap::new(), false)?;
    out.data["check_results"] = check_results;
    if failed || stale {
        out.exit_code = 1;
    }
    Ok(out)
}
pub fn dispatch(
    args: &Cli,
    s: &Store,
    snap: &Snapshot,
    command: &cli::Dispatch,
) -> Result<Outcome> {
    match command {
        cli::Dispatch::Plan { story } => {
            let records: Vec<_> = snap
                .records
                .iter()
                .filter(|r| r.kind == Kind::Story && story.as_ref().is_none_or(|id| *id == r.id))
                .map(|r| aep_core::readiness(r, &snap.records, &s.config))
                .collect();
            if story.is_some() && records.is_empty() {
                return Err(Error::input("Unknown story"));
            }
            Ok(Outcome::ok(
                json!({"revision":snap.revision,"stories":records}),
            ))
        }
        cli::Dispatch::Start {
            story,
            base,
            owner,
            worktree,
        } => {
            let mut story = subject(snap, story, Kind::Story)?;
            let ready = aep_core::readiness(&story, &snap.records, &s.config);
            if !ready.ready {
                return Err(Error::blocked(ready.reasons.join("; ")));
            }
            if owner.trim().is_empty() || story.paths.is_empty() {
                return Err(Error::blocked(
                    "Dispatch needs an owner and explicit write scope",
                ));
            }
            let base = git(
                &s.root,
                &["rev-parse", "--verify", &format!("{base}^{{commit}}")],
            )?;
            let mut required = vec![
                s.record_path(Kind::Story, &story.id)?,
                s.record_path(Kind::Change, story.change_ids()[0])?,
                ".aep/config.toml".into(),
            ];
            required.extend(
                snap.files
                    .keys()
                    .filter(|p| p.starts_with(&format!("{}/", s.config.stores.rules)))
                    .cloned(),
            );
            required.extend(linked_context(s, snap, &story)?.into_keys());
            required.extend(
                ["AGENTS.md", "CLAUDE.md"]
                    .into_iter()
                    .filter(|p| snap.files.contains_key(*p))
                    .map(str::to_string),
            );
            for id in story.change_ids() {
                required.extend(
                    snap.files
                        .keys()
                        .filter(|p| {
                            p.starts_with(&format!("{}/changes/{id}/", s.config.stores.ledger))
                        })
                        .cloned(),
                );
                required.extend(
                    crate::spec::contracts(snap.get(id)?)?
                        .into_iter()
                        .map(|c| c.path),
                );
                crate::spec::validate_change(s, snap, snap.get(id)?)?;
            }
            for dependency in aep_core::dependency_records(&story, &snap.records)
                .into_iter()
                .filter(|r| r.kind == Kind::Story)
            {
                let dep = &dependency.id;
                let receipt = snap.get(dependency.text("delivery").ok_or_else(|| {
                    Error::blocked(format!("Dependency {dep} has no integration receipt"))
                })?)?;
                let head = receipt
                    .text("integration_head")
                    .ok_or_else(|| Error::blocked("Dependency integration revision is unknown"))?;
                git(&s.root, &["merge-base", "--is-ancestor", head, &base]).map_err(|_| {
                    Error::blocked(format!(
                        "Base does not contain dependency {dep} integration {head}"
                    ))
                })?;
            }
            for path in required {
                let committed = aep_store::git_bytes(&s.root, &["show", &format!("{base}:{path}")])
                    .map_err(|_| {
                        Error::blocked(format!(
                            "Required design/config is absent from base: {path}"
                        ))
                    })?;
                let current = std::fs::read_to_string(contained(&s.root, &path)?)?;
                if committed != current.as_bytes() {
                    return Err(Error::blocked(format!(
                        "Required design/config differs from committed base: {path}"
                    )));
                }
            }
            let id = unique_id("attempt");
            let branch = format!("aep/{}-{id}", story.id);
            let path = worktree
                .clone()
                .unwrap_or_else(|| s.root.join(".aep/worktrees").join(&id));
            let path = if path.is_absolute() {
                path
            } else {
                s.root.join(path)
            };
            let path = aep_store::destination(&path)?;
            if path.exists() {
                return Err(Error::conflict("Worktree destination already exists"));
            }
            if args.dry_run {
                return Ok(Outcome::ok(
                    json!({"story":story.id,"base":base,"branch":branch,"worktree":path,"owner":owner}),
                ));
            }
            let mut attempt = Record::new(Kind::Attempt, &id, &format!("Implement {}", story.id));
            attempt.status = "prepared".into();
            attempt.refs = vec![story.id.clone()];
            attempt.owner = Some(owner.clone());
            attempt.set("base", base.clone());
            attempt.set("branch", branch.clone());
            attempt.set("worktree", path.to_string_lossy().to_string());
            attempt.set("store", s.root.to_string_lossy().to_string());
            attempt.set("guidance_digest", crate::guidance::bundle_digest());
            attempt.set("cli_version", aep_core::VERSION);
            story.status = "in_progress".into();
            save(
                s,
                snap,
                vec![attempt.clone(), story],
                BTreeMap::new(),
                false,
            )?;
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| Error::from(e).changed())?;
            }
            if let Err(e) = run_git(
                &s.root,
                &[
                    "worktree",
                    "add",
                    "-b",
                    &branch,
                    &path.to_string_lossy(),
                    &base,
                ],
            ) {
                return Err(e.changed());
            }
            let actual = attempt_path(s, &attempt)?;
            let mut out = Outcome::ok(
                json!({"attempt":record_json(&attempt),"worktree":actual,"launch_request":{"cwd":actual,"store":s.root,"prompt":format!("Read AGENTS.md and aep --skill implement. Implement story {} in this worktree. Use aep --root {} for shared records. Record actual checks and results.",attempt.refs[0],s.root.display())},"worker_started":false}),
            );
            out.changed = true;
            Ok(out)
        }
    }
}
pub fn worktree(s: &Store, snap: &Snapshot, command: &cli::Worktree) -> Result<Outcome> {
    let cli::Worktree::Inspect { attempt } = command;
    let r = subject(snap, attempt, Kind::Attempt)?;
    let path = attempt_path(s, &r)?;
    Ok(Outcome::ok(
        json!({"attempt":r,"worktree":path,"head":git(&path,&["rev-parse","HEAD"])?,"changes":git(&path,&["status","--porcelain"])?}),
    ))
}
pub fn attempt(args: &Cli, s: &Store, snap: &Snapshot, command: &cli::Attempt) -> Result<Outcome> {
    let id = match command {
        cli::Attempt::Status { id }
        | cli::Attempt::Record { id, .. }
        | cli::Attempt::Recover { id, .. } => id,
    };
    let mut r = subject(snap, id, Kind::Attempt)?;
    match command {
        cli::Attempt::Status { .. } => Ok(Outcome::ok(record_json(&r))),
        cli::Attempt::Record { status, .. } => {
            if !["prepared", "running", "review"].contains(&r.status.as_str()) {
                return Err(Error::blocked("Attempt is terminal"));
            }
            attempt_path(s, &r)?;
            r.status = match status {
                cli::AttemptStatus::Running => "running",
                cli::AttemptStatus::Review => "review",
                cli::AttemptStatus::Failed => "failed",
                cli::AttemptStatus::Cancelled => "cancelled",
            }
            .into();
            let mut updates = vec![r.clone()];
            if matches!(
                status,
                cli::AttemptStatus::Failed | cli::AttemptStatus::Cancelled
            ) {
                let mut story = subject(snap, &r.refs[0], Kind::Story)?;
                story.status = "pending".into();
                updates.push(story);
            }
            save(s, snap, updates, BTreeMap::new(), args.dry_run)
        }
        cli::Attempt::Recover { cancel, .. } => {
            if *cancel {
                if !["prepared", "running", "review"].contains(&r.status.as_str()) {
                    return Err(Error::blocked("Only an active attempt can be cancelled"));
                }
                r.status = "cancelled".into();
                let mut story = subject(snap, &r.refs[0], Kind::Story)?;
                story.status = "pending".into();
                save(s, snap, vec![r, story], BTreeMap::new(), args.dry_run)
            } else {
                let path = attempt_path(s, &r)?;
                Ok(Outcome::ok(
                    json!({"attempt":r,"worktree":path,"worker_liveness":"unknown; reconcile through the host before relaunching"}),
                ))
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Finding {
    severity: String,
    description: String,
    #[serde(default)]
    resolved: bool,
    #[serde(default)]
    evidence: Vec<String>,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReviewInput {
    request: String,
    reviewer: String,
    head: String,
    fingerprint: String,
    findings: Vec<Finding>,
    #[serde(default)]
    attestation: bool,
}
fn latest_completed_review<'a>(snap: &'a Snapshot, story: &str) -> Option<&'a Record> {
    snap.records
        .iter()
        .filter(|r| {
            r.kind == Kind::Review
                && r.refs.iter().any(|id| id == story)
                && ["blocking", "material", "pass", "pass_with_notes"].contains(&r.status.as_str())
        })
        .max_by_key(|r| (&r.created_at, &r.id))
}
fn unresolved_findings(review: &Record) -> Vec<Value> {
    review
        .data
        .get("findings")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter(|finding| {
            matches!(finding["severity"].as_str(), Some("blocking" | "material"))
                && finding["resolved"] != true
        })
        .cloned()
        .collect()
}
pub fn review(args: &Cli, s: &Store, snap: &Snapshot, command: &cli::Review) -> Result<Outcome> {
    match command {
        cli::Review::Request { story } => {
            let story = subject(snap, story, Kind::Story)?;
            let plan = verification_plan(s, snap, &story)?;
            let attempt = verification_attempt(snap, &story)?;
            let prior: Vec<_> = snap
                .records
                .iter()
                .filter(|r| r.kind == Kind::Review && r.refs.contains(&attempt.id))
                .collect();
            if prior.iter().any(|r| {
                r.status == "requested" && r.text("fingerprint") == Some(plan.fingerprint.as_str())
            }) {
                return Err(Error::blocked(
                    "A review is already requested for this candidate",
                ));
            }
            let mut r = Record::new(
                Kind::Review,
                &unique_id("review"),
                &format!("Review {}", story.id),
            );
            r.status = "requested".into();
            r.refs = vec![story.id.clone(), attempt.id.clone()];
            r.set("head", plan.head.clone());
            r.set("fingerprint", plan.fingerprint.clone());
            r.set("builder", attempt.owner.clone().unwrap_or_default());
            r.set("round", json!(prior.len() + 1));
            let carried = latest_completed_review(snap, &story.id)
                .map(unresolved_findings)
                .unwrap_or_default();
            r.set("required_findings", json!(carried));
            let mut updates: Vec<Record> = prior
                .into_iter()
                .filter(|review| review.status == "requested")
                .map(|review| {
                    let mut review = review.clone();
                    review.status = "superseded".into();
                    review.set("superseded_by", r.id.clone());
                    review
                })
                .collect();
            updates.push(r.clone());
            let mut out = save(s, snap, updates, BTreeMap::new(), args.dry_run)?;
            out.data["request"] = json!({"record":r,"contracts":story.change_ids(),"scope":story.paths,"checks":plan,"response":{"request":r.id,"reviewer":"independent-reviewer","head":r.text("head"),"fingerprint":r.text("fingerprint"),"findings":[]},"instructions":"Review accepted intent, actual diff, applicable rules, and check evidence. Classify concrete findings as blocking, material, or polish. Attribute the result to the reviewer; do not infer pass from the builder's summary."});
            Ok(out)
        }
        cli::Review::Record(file) => {
            let response: ReviewInput = input(&file.file)?;
            let mut r = subject(snap, &response.request, Kind::Review)?;
            let story = r
                .refs
                .iter()
                .filter_map(|id| snap.get(id).ok())
                .find(|r| r.kind == Kind::Story)
                .ok_or_else(|| Error::input("Review has no story"))?;
            let plan = verification_plan(s, snap, story)?;
            if response.reviewer.trim().is_empty()
                || response.head != plan.head
                || response.fingerprint != plan.fingerprint
            {
                return Err(Error::blocked(
                    "Review attribution or current revision/fingerprint does not match",
                ));
            }
            if response.attestation {
                let old: Vec<Finding> =
                    serde_json::from_value(r.data.get("findings").cloned().unwrap_or(json!([])))?;
                if !["material", "blocking"].contains(&r.status.as_str())
                    || (r.status == "blocking" && plan.independent_review)
                    || !r.refs.contains(&plan.attempt)
                    || latest_completed_review(snap, &story.id).map(|latest| &latest.id)
                        != Some(&r.id)
                    || r.text("builder") != Some(response.reviewer.as_str())
                    || old.len() != response.findings.len()
                {
                    return Err(Error::blocked(
                        "Fix attestation must preserve the latest review findings and come from the builder; required independent blocking review cannot be self-attested",
                    ));
                }
                for (before, after) in old.iter().zip(&response.findings) {
                    if before.severity != after.severity || before.description != after.description
                    {
                        return Err(Error::input("Attestation cannot rewrite the review"));
                    }
                }
                r.set("reviewed_head", r.text("head").unwrap_or("").to_string());
                r.set("fix_attestation_by", response.reviewer.clone());
            } else {
                if r.status != "requested"
                    || !r.refs.contains(&plan.attempt)
                    || r.text("builder") == Some(response.reviewer.as_str())
                    || r.text("head") != Some(response.head.as_str())
                    || r.text("fingerprint") != Some(response.fingerprint.as_str())
                {
                    return Err(Error::blocked(
                        "Expected an independent response to the current requested review",
                    ));
                }
                r.set("reviewer", response.reviewer.clone());
                r.set("attribution_class", "host_reported");
            }
            let mut required: Vec<Finding> = serde_json::from_value(
                r.data
                    .get("required_findings")
                    .cloned()
                    .unwrap_or(json!([])),
            )?;
            // A candidate can return to an earlier revision while another request
            // is open. Findings recorded since this request are still obligations.
            if let Some(latest) = latest_completed_review(snap, &story.id) {
                required.extend(serde_json::from_value::<Vec<Finding>>(json!(
                    unresolved_findings(latest)
                ))?);
            }
            for prior in required {
                if !response
                    .findings
                    .iter()
                    .any(|f| f.severity == prior.severity && f.description == prior.description)
                {
                    return Err(Error::blocked(
                        "Review must explicitly retain each unresolved prior blocking or material finding",
                    ));
                }
            }
            for finding in &response.findings {
                if !["blocking", "material", "polish"].contains(&finding.severity.as_str())
                    || finding.description.trim().is_empty()
                {
                    return Err(Error::input("Invalid review finding"));
                }
                if finding.resolved
                    && finding.severity != "polish"
                    && (finding.evidence.is_empty()
                        || !finding.evidence.iter().all(|id| {
                            snap.get(id).is_ok_and(|e| {
                                e.kind == Kind::Evidence
                                    && e.status == "pass"
                                    && e.text("fingerprint") == Some(plan.fingerprint.as_str())
                            })
                        }))
                {
                    return Err(Error::blocked(
                        "Resolved findings require current passing evidence",
                    ));
                }
            }
            r.status = if response
                .findings
                .iter()
                .any(|f| f.severity == "blocking" && !f.resolved)
            {
                "blocking"
            } else if response
                .findings
                .iter()
                .any(|f| f.severity == "material" && !f.resolved)
            {
                "material"
            } else if response.attestation {
                "pass_with_notes"
            } else {
                "pass"
            }
            .into();
            r.set("head", response.head);
            r.set("fingerprint", response.fingerprint);
            r.set("findings", json!(response.findings));
            save(s, snap, vec![r], BTreeMap::new(), args.dry_run)
        }
    }
}
pub fn evidence_requirements(
    s: &Store,
    snap: &Snapshot,
    story: &Record,
) -> Result<(VerificationPlan, Vec<String>, Vec<String>)> {
    let plan = verification_plan(s, snap, story)?;
    let mut missing = vec![];
    let mut evidence = vec![];
    if plan.checks.is_empty() {
        missing.push("No required verification checks are configured".into());
    }
    for check in &plan.checks {
        let latest = snap
            .records
            .iter()
            .filter(|r| {
                r.kind == Kind::Evidence
                    && r.refs.contains(&story.id)
                    && r.text("check_id") == Some(check.as_str())
                    && r.text("fingerprint") == Some(plan.fingerprint.as_str())
            })
            .max_by(|a, b| a.created_at.cmp(&b.created_at).then(a.id.cmp(&b.id)));
        match latest {
            Some(r) if r.status == "pass" => evidence.push(r.id.clone()),
            _ => missing.push(format!("Missing current passing check {check}")),
        }
    }
    // Choosing review is optional by default; ignoring a concrete unresolved defect is not.
    // Keep findings across revision changes until a review or permitted evidence-backed
    // builder attestation records their resolution.
    if let Some(review) = latest_completed_review(snap, &story.id)
        && !unresolved_findings(review).is_empty()
    {
        missing.push(format!("Unresolved review findings in {}", review.id));
    }
    let latest_review = snap
        .records
        .iter()
        .filter(|r| {
            r.kind == Kind::Review
                && r.refs.contains(&plan.attempt)
                && r.text("fingerprint") == Some(plan.fingerprint.as_str())
        })
        .max_by_key(|r| {
            (
                r.data.get("round").and_then(Value::as_u64),
                &r.created_at,
                &r.id,
            )
        });
    if plan.independent_review {
        match latest_review {
            Some(r) if ["pass", "pass_with_notes"].contains(&r.status.as_str()) => {
                evidence.push(r.id.clone())
            }
            _ => missing.push("Missing current independent review".into()),
        }
    } else if let Some(review) = latest_review {
        match review.status.as_str() {
            "requested" => missing.push("Current requested review has no response".into()),
            "pass" | "pass_with_notes" => evidence.push(review.id.clone()),
            _ => {}
        }
    }
    Ok((plan, missing, evidence))
}
pub fn evaluate_gate(args: &Cli, s: &Store, snap: &Snapshot, id: &str) -> Result<Outcome> {
    let mut gate = subject(snap, id, Kind::Gate)?;
    let mut missing = vec![];
    let mut evidence = vec![];
    let stories = aep_core::scope_stories(&gate.refs, &snap.records);
    if stories.is_empty() {
        missing.push("Gate has no explicit story/container scope".into());
    }
    for id in stories {
        let story = snap.get(&id)?;
        match evidence_requirements(s, snap, story) {
            Ok((plan, problems, receipts)) => {
                missing.extend(problems.into_iter().map(|m| format!("{id}: {m}")));
                evidence.extend(receipts);
                for check in &gate.required_checks {
                    let latest = snap
                        .records
                        .iter()
                        .filter(|r| {
                            r.kind == Kind::Evidence
                                && r.refs.contains(&id)
                                && r.text("check_id") == Some(check.as_str())
                                && r.text("fingerprint") == Some(plan.fingerprint.as_str())
                        })
                        .max_by_key(|r| (&r.created_at, &r.id));
                    match latest {
                        Some(r) if r.status == "pass" => evidence.push(r.id.clone()),
                        _ => missing.push(format!("{id}: missing gate check {check}")),
                    }
                }
                if let Some(environment) = gate.text("environment").filter(|env| *env != "local")
                    && !evidence.iter().filter_map(|id| snap.get(id).ok()).any(|r| {
                        r.kind == Kind::Evidence
                            && r.refs.contains(&story.id)
                            && r.text("environment") == Some(environment)
                            && r.text("fingerprint") == Some(plan.fingerprint.as_str())
                    })
                {
                    missing.push(format!("{id}: no evidence for environment {environment}"));
                }
            }
            Err(e) => missing.push(format!("{id}: {e}")),
        }
    }
    for id in &gate.required_gates {
        let g = snap.get(id)?;
        if !aep_core::gate_current(g, &snap.records, &s.config) {
            missing.push(format!("Required gate {id} is not current/pass"));
        }
    }
    for id in &gate.required_checks {
        if !evidence
            .iter()
            .filter_map(|e| snap.get(e).ok())
            .any(|r| r.text("check_id") == Some(id.as_str()))
        {
            missing.push(format!("Gate check {id} has no passing receipt in scope"));
        }
    }
    gate.status = if missing.is_empty() {
        "pass"
    } else {
        "blocked"
    }
    .into();
    gate.set("evidence", json!(evidence));
    gate.set("missing", json!(missing));
    gate.set(
        "scope_digest",
        aep_core::gate_scope(&gate, &snap.records, &s.config),
    );
    let mut out = save(s, snap, vec![gate], BTreeMap::new(), args.dry_run)?;
    if !missing.is_empty() {
        out.exit_code = 3;
    }
    out.data["missing"] = json!(missing);
    Ok(out)
}
fn ensure_clean_integration(s: &Store) -> Result<()> {
    // The control checkout contains pending ledger receipts. Only managed context
    // may be dirty; unrelated code must never be included in an unverified merge.
    let paths = git(
        &s.root,
        &["-c", "core.quotePath=false", "diff", "HEAD", "--name-only"],
    )?;
    let untracked = git(&s.root, &["ls-files", "--others", "--exclude-standard"])?;
    for p in paths.lines().chain(untracked.lines()) {
        if !p.starts_with(&format!("{}/", s.config.stores.ledger)) {
            return Err(Error::blocked(format!(
                "Integration checkout has uncommitted changes outside ledger: {p}"
            )));
        }
    }
    Ok(())
}
fn gh(s: &Store, args: &[String]) -> Result<Value> {
    let mut argv = vec!["gh".into()];
    argv.extend(args.iter().cloned());
    let result = execute(&argv, &s.root, 120)?;
    if result.code != Some(0) || result.timed_out || result.cancelled {
        return Err(Error::new("provider", result.stderr, 6));
    }
    serde_json::from_str(result.stdout.trim())
        .or_else(|_| Ok(json!({"output":result.stdout.trim()})))
}
pub fn deliver(args: &Cli, s: &Store, snap: &Snapshot, command: &cli::Deliver) -> Result<Outcome> {
    let id = match command {
        cli::Deliver::Plan { story }
        | cli::Deliver::Pr { story, .. }
        | cli::Deliver::Merge { story, .. }
        | cli::Deliver::Status { story }
        | cli::Deliver::Reconcile { story } => story,
    };
    let mut story = subject(snap, id, Kind::Story)?;
    if matches!(command, cli::Deliver::Status { .. }) {
        let receipts: Vec<_> = snap
            .records
            .iter()
            .filter(|r| r.kind == Kind::Delivery && r.refs.contains(id))
            .collect();
        let provider = if let Some(pr) = story.text("pr_url") {
            Some(gh(
                s,
                &[
                    "pr".into(),
                    "view".into(),
                    pr.into(),
                    "--json".into(),
                    "state,headRefOid,mergeCommit,url".into(),
                ],
            )?)
        } else {
            None
        };
        return Ok(Outcome::ok(
            json!({"receipts":receipts,"provider":provider,"story":story}),
        ));
    }
    if matches!(command, cli::Deliver::Reconcile { .. }) {
        let receipt = snap
            .records
            .iter()
            .filter(|r| r.kind == Kind::Delivery && r.status == "prepared" && r.refs.contains(id))
            .max_by_key(|r| &r.created_at)
            .ok_or_else(|| Error::blocked("No pending delivery intent to reconcile"))?
            .clone();
        if args.dry_run {
            return Ok(Outcome::ok(json!({"intent":receipt})));
        }
        let attempt = subject(
            snap,
            receipt
                .text("attempt")
                .ok_or_else(|| Error::input("Intent has no attempt"))?,
            Kind::Attempt,
        )?;
        if receipt.text("operation") == Some("local_merge") {
            let head = git(&s.root, &["rev-parse", "HEAD"])?;
            if receipt.text("head") != Some(head.as_str()) {
                return Err(Error::blocked(
                    "Local merge is not confirmed at the intended head",
                ));
            }
            return finalize_delivery(s, receipt, &story.id, &attempt.id, &head);
        }
        let pr = story
            .text("pr_url")
            .or_else(|| receipt.text("pr_url"))
            .ok_or_else(|| {
                Error::blocked(
                    "No PR URL recorded; retry PR creation to reconcile the branch lookup",
                )
            })?;
        let state = gh(
            s,
            &[
                "pr".into(),
                "view".into(),
                pr.into(),
                "--json".into(),
                "state,headRefOid,mergeCommit,url".into(),
            ],
        )?;
        if state["state"] != "MERGED" || state["headRefOid"].as_str() != receipt.text("head") {
            return Err(Error::blocked(
                "Provider has not confirmed the intended merge",
            ));
        }
        let head = state["mergeCommit"]["oid"]
            .as_str()
            .ok_or_else(|| Error::blocked("Provider omitted merge revision"))?;
        return finalize_delivery(s, receipt, &story.id, &attempt.id, head);
    }
    let (plan, missing, evidence) = evidence_requirements(s, snap, &story)?;
    if matches!(command, cli::Deliver::Plan { .. }) || args.dry_run {
        return Ok(Outcome::ok(
            json!({"verification":plan,"missing":missing,"evidence":evidence,"eligible":missing.is_empty(),
                "guidance": "This checks candidate readiness only. Continue the requested delivery and closure using aep --skill deliver; use existing authorization and report any remaining decision."}),
        ));
    }
    if !missing.is_empty() {
        return Err(Error::blocked(missing.join("; ")));
    }
    let attempt = active_attempt(snap, id)?;
    let path = attempt_path(s, &attempt)?;
    let branch = attempt.text("branch").unwrap().to_string();
    let mut receipt = Record::new(
        Kind::Delivery,
        &unique_id("delivery"),
        &format!("Deliver {id}"),
    );
    receipt.refs = vec![id.clone()];
    receipt.status = "prepared".into();
    receipt.set("attempt", plan.attempt.clone());
    receipt.set("head", plan.head.clone());
    receipt.set("fingerprint", plan.fingerprint.clone());
    receipt.set("evidence", json!(evidence));
    match command {
        cli::Deliver::Pr { base, .. } => {
            if story.text("pr_url").is_some() {
                return Err(Error::blocked(
                    "A PR is already recorded; inspect deliver status",
                ));
            }
            let existing = gh(
                s,
                &[
                    "pr".into(),
                    "list".into(),
                    "--head".into(),
                    branch.clone(),
                    "--state".into(),
                    "all".into(),
                    "--json".into(),
                    "url,state,headRefOid".into(),
                ],
            )?;
            if let Some(pr) = existing.as_array().and_then(|a| a.first()) {
                if pr["headRefOid"].as_str() != Some(plan.head.as_str()) {
                    return Err(Error::conflict(
                        "Existing PR head differs; reconcile provider state",
                    ));
                }
                receipt.status = "pr".into();
                receipt.set("provider", pr.clone());
                story.set("pr_url", pr["url"].clone());
                return save(s, snap, vec![receipt, story], BTreeMap::new(), false);
            }
            receipt.set("operation", "pr");
            save(s, snap, vec![receipt.clone()], BTreeMap::new(), false)?;
            git(&path, &["push", "-u", "origin", &branch]).map_err(Error::changed)?;
            let body_path = s.root.join(".aep").join(format!("{}.md", receipt.id));
            std::fs::write(
                &body_path,
                format!(
                    "{}\n\nStory: {}\nChange: {}\nVerified head: {}\n",
                    story.description,
                    story.id,
                    story.change_ids().join(", "),
                    plan.head
                ),
            )
            .map_err(|e| Error::from(e).changed())?;
            let result = gh(
                s,
                &[
                    "pr".into(),
                    "create".into(),
                    "--head".into(),
                    branch,
                    "--base".into(),
                    base.clone(),
                    "--title".into(),
                    story.title.clone(),
                    "--body-file".into(),
                    body_path.to_string_lossy().into(),
                ],
            )
            .map_err(Error::changed)?;
            let _ = std::fs::remove_file(body_path);
            let url = result["output"].as_str().ok_or_else(|| {
                Error::new(
                    "provider",
                    "PR response has no URL; reconcile deliver status",
                    6,
                )
                .changed()
            })?;
            receipt.status = "pr".into();
            receipt.set("url", url);
            story.set("pr_url", url);
            let current = s.snapshot().map_err(Error::changed)?;
            let mut latest_story = subject(&current, &story.id, Kind::Story)?;
            latest_story.set("pr_url", story.data["pr_url"].clone());
            save(
                s,
                &current,
                vec![receipt, latest_story],
                BTreeMap::new(),
                false,
            )
            .map_err(Error::changed)
        }
        cli::Deliver::Merge { local, .. } => {
            if let Some(pr) = story.text("pr_url") {
                receipt.set("pr_url", pr.to_string());
            }
            receipt.set(
                "operation",
                if *local {
                    "local_merge"
                } else {
                    "github_merge"
                },
            );
            let integration_head = if *local {
                if s.root == path {
                    return Err(Error::blocked(
                        "Run local integration from the integration checkout",
                    ));
                }
                ensure_clean_integration(s)?;
                let target_head = git(&s.root, &["rev-parse", "HEAD"])?;
                git(&s.root,&["merge-base","--is-ancestor",&target_head,&plan.head]).map_err(|_|Error::blocked("Rebase the implementation onto the integration head, then verify and review the new candidate"))?;
                save(s, snap, vec![receipt.clone()], BTreeMap::new(), false)?;
                git(&s.root, &["merge", "--ff-only", &plan.head]).map_err(Error::changed)?;
                git(&s.root, &["rev-parse", "HEAD"]).map_err(Error::changed)?
            } else {
                let pr = story
                    .text("pr_url")
                    .ok_or_else(|| Error::blocked("Create or reconcile a PR before merging"))?;
                let state = gh(
                    s,
                    &[
                        "pr".into(),
                        "view".into(),
                        pr.into(),
                        "--json".into(),
                        "state,headRefOid,baseRefOid,mergeStateStatus".into(),
                    ],
                )?;
                if state["headRefOid"].as_str() != Some(plan.head.as_str())
                    || state["state"] != "OPEN"
                    || state["mergeStateStatus"] != "CLEAN"
                {
                    return Err(Error::blocked("PR is not clean/open at the verified head"));
                }
                let base = state["baseRefOid"]
                    .as_str()
                    .ok_or_else(|| Error::blocked("Provider base revision is unknown"))?;
                git(&path, &["merge-base", "--is-ancestor", base, &plan.head]).map_err(|_| {
                    Error::blocked(
                        "Provider base has advanced; update and reverify the implementation",
                    )
                })?;
                save(s, snap, vec![receipt.clone()], BTreeMap::new(), false)?;
                gh(
                    s,
                    &[
                        "pr".into(),
                        "merge".into(),
                        pr.into(),
                        "--squash".into(),
                        "--match-head-commit".into(),
                        plan.head.clone(),
                    ],
                )
                .map_err(Error::changed)?;
                let merged = gh(
                    s,
                    &[
                        "pr".into(),
                        "view".into(),
                        pr.into(),
                        "--json".into(),
                        "state,mergeCommit,url".into(),
                    ],
                )
                .map_err(Error::changed)?;
                if merged["state"] != "MERGED" {
                    return Err(Error::new(
                        "provider",
                        "Merge result is not confirmed; reconcile status",
                        6,
                    )
                    .changed());
                }
                receipt.set("provider", merged.clone());
                merged["mergeCommit"]["oid"]
                    .as_str()
                    .ok_or_else(|| {
                        Error::new("provider", "No merge commit in provider response", 6).changed()
                    })?
                    .into()
            };
            finalize_delivery(s, receipt, &story.id, &attempt.id, &integration_head)
        }
        _ => Err(Error::input("Unsupported delivery operation")),
    }
}

fn finalize_delivery(
    s: &Store,
    mut receipt: Record,
    story_id: &str,
    attempt_id: &str,
    head: &str,
) -> Result<Outcome> {
    // Record confirmed external reality, preserving prose/ownership edits made while the provider ran.
    let current = s.snapshot().map_err(Error::changed)?;
    let mut story = subject(&current, story_id, Kind::Story)?;
    let mut attempt = subject(&current, attempt_id, Kind::Attempt)?;
    if story.text("delivery").is_some_and(|id| id != receipt.id) {
        return Err(Error::conflict("Story already has a different integration receipt").changed());
    }
    receipt.status = "integrated".into();
    receipt.set("integration_head", head.to_string());
    if git(&s.root, &["cat-file", "-e", &format!("{head}^{{commit}}")]).is_err() {
        let _ = git(&s.root, &["fetch", "--no-tags", "origin", head]);
    }
    let actual = git(
        &s.root,
        &["rev-parse", "--verify", &format!("{head}^{{tree}}")],
    );
    let checked = git(
        &s.root,
        &[
            "rev-parse",
            "--verify",
            &format!("{}^{{tree}}", receipt.text("head").unwrap_or("")),
        ],
    );
    let same_tree = matches!((&actual, &checked), (Ok(a), Ok(b)) if a == b);
    receipt.set("tree_verified", same_tree);
    story.status = "integrated".into();
    story.set("delivery", receipt.id.clone());
    attempt.status = "done".into();
    let mut out = save(
        s,
        &current,
        vec![receipt, story, attempt],
        BTreeMap::new(),
        false,
    )
    .map_err(Error::changed)?;
    if !same_tree {
        out.exit_code = 3;
        out.diagnostics.push(json!({"code":"integration_tree", "message":"Integration is recorded, but its tree is not the verified candidate. Inspect the provider outcome and reopen for current verification."}));
    }
    Ok(out)
}

pub fn promote_release(
    args: &Cli,
    s: &Store,
    snap: &Snapshot,
    id: &str,
    environment: &str,
    by: &str,
) -> Result<Outcome> {
    let mut release = subject(snap, id, Kind::Release)?;
    if !["pending", "draft"].contains(&release.status.as_str())
        || environment.trim().is_empty()
        || environment == "local"
        || by.trim().is_empty()
        || release.required_gates.is_empty()
    {
        return Err(Error::blocked(
            "Release promotion needs an unreleased container, nonlocal environment, attributed decision, and gates",
        ));
    }
    let scope = aep_core::scope_stories(&[id.into()], &snap.records);
    if scope.is_empty() {
        return Err(Error::blocked("Release membership is empty"));
    }
    let mut covered = BTreeSet::new();
    for gate_id in &release.required_gates {
        let gate = subject(snap, gate_id, Kind::Gate)?;
        if !aep_core::gate_current(&gate, &snap.records, &s.config)
            || gate.text("environment") != Some(environment)
        {
            return Err(Error::blocked(
                "Release gate is not current for the requested environment",
            ));
        }
        covered.extend(aep_core::scope_stories(&gate.refs, &snap.records));
    }
    if !scope.is_subset(&covered) {
        return Err(Error::blocked("Release gates do not cover every member"));
    }
    let mut stories = vec![];
    let mut members = vec![];
    for id in scope {
        let mut story = subject(snap, &id, Kind::Story)?;
        if !["integrated", "released"].contains(&story.status.as_str()) {
            return Err(Error::blocked("Release contains an unintegrated story"));
        }
        let (_, missing, _) = evidence_requirements(s, snap, &story)?;
        if !missing.is_empty() {
            return Err(Error::blocked(missing.join("; ")));
        }
        members.push(
            json!({"story":story.id,"revision":story.revision(),"delivery":story.text("delivery")}),
        );
        story.status = "released".into();
        story.set("release_record", release.id.clone());
        stories.push(story);
    }
    release.status = "released".into();
    release.set("environment", environment.to_string());
    release.set("promoted_by", by.to_string());
    release.set("members", json!(members));
    stories.push(release);
    save(s, snap, stories, BTreeMap::new(), args.dry_run)
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn process_runner_drains_bounded_output_and_terminates_owned_children() {
        let root = tempfile::tempdir().unwrap();
        let run = |script: &str, timeout| {
            execute(
                &["sh".into(), "-c".into(), script.into()],
                root.path(),
                timeout,
            )
            .unwrap()
        };
        let output = run("head -c 100000 /dev/zero; head -c 100000 /dev/zero >&2", 5);
        assert_eq!(output.code, Some(0));
        assert!(output.truncated);
        assert_eq!(output.stdout.len(), 65536);
        assert_eq!(output.stderr.len(), 65536);
        let start = Instant::now();
        let timeout = run("sleep 20 & wait", 1);
        assert!(timeout.timed_out);
        assert!(start.elapsed() < Duration::from_secs(5));
        let start = Instant::now();
        let parent_exit = run("sleep 20 & exit 0", 5);
        assert_eq!(parent_exit.code, Some(0));
        assert!(start.elapsed() < Duration::from_secs(5));
        INTERRUPTED.store(true, Ordering::SeqCst);
        let cancelled = run("sleep 20", 5);
        INTERRUPTED.store(false, Ordering::SeqCst);
        assert!(cancelled.cancelled);
        assert!(!cancelled.timed_out);
    }
}
