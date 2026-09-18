//! `aep eval`: a neutral, read-only view of a project's engineering process.
//!
//! Facts are collected from the project's records, Git state and repository
//! files into a run directory under the machine-level AEP home (`~/.aep`).
//! Rules turn facts into findings about AEP adherence and engineering
//! practice. Nothing here writes into the project.
use crate::{
    cli::{Cli, Eval},
    commands::{self, Outcome},
    guidance,
};
use aep_core::{Error, Kind, Record, Result, digest, now};
use aep_store::{Snapshot, Store, git};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

/// Machine-level AEP home: `$AEP_HOME` or `~/.aep`.
pub fn home() -> Result<PathBuf> {
    if let Some(h) = std::env::var_os("AEP_HOME") {
        return Ok(PathBuf::from(h));
    }
    let home = std::env::var_os("HOME").ok_or_else(|| Error::input("HOME is not set"))?;
    Ok(PathBuf::from(home).join(".aep"))
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct EvalConfig {
    pub root: String,
    pub observer_kind: String,
    pub retention_days: u32,
    pub projects: BTreeMap<String, ProjectOverride>,
}
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ProjectOverride {
    pub observer_kind: Option<String>,
}
impl Default for EvalConfig {
    fn default() -> Self {
        Self {
            root: "~/.aep/eval".into(),
            observer_kind: "codex".into(),
            retention_days: 90,
            projects: BTreeMap::new(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct MachineConfig {
    pub schema_version: u32,
    pub eval: EvalConfig,
}
impl Default for MachineConfig {
    fn default() -> Self {
        Self {
            schema_version: 1,
            eval: EvalConfig::default(),
        }
    }
}
fn load_config() -> Result<MachineConfig> {
    let path = home()?.join("config.toml");
    match fs::read_to_string(&path) {
        Ok(text) => {
            toml::from_str(&text).map_err(|e| Error::input(format!("{}: {e}", path.display())))
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(MachineConfig::default()),
        Err(e) => Err(e.into()),
    }
}
fn expand(path: &str) -> Result<PathBuf> {
    if let Some(rest) = path.strip_prefix("~/") {
        let home = std::env::var_os("HOME").ok_or_else(|| Error::input("HOME is not set"))?;
        return Ok(PathBuf::from(home).join(rest));
    }
    if path == "~/.aep/eval" || path.starts_with("~/.aep") {
        return home().map(|h| h.join(path.trim_start_matches("~/.aep/")));
    }
    Ok(PathBuf::from(path))
}
fn eval_root(config: &MachineConfig) -> Result<PathBuf> {
    if config.eval.root == "~/.aep/eval" {
        return Ok(home()?.join("eval"));
    }
    expand(&config.eval.root)
}

/// Stable project identity: `host/owner/repo` from the origin remote, else a path digest.
pub fn project_id(root: &Path) -> String {
    if let Ok(url) = git(root, &["remote", "get-url", "origin"]) {
        let mut u = url.trim().to_string();
        for prefix in ["https://", "http://", "ssh://", "git://"] {
            if let Some(rest) = u.strip_prefix(prefix) {
                u = rest.to_string();
            }
        }
        if let Some(rest) = u.strip_prefix("git@") {
            u = rest.replacen(':', "/", 1);
        }
        if let Some((_, rest)) = u.split_once('@') {
            u = rest.to_string();
        }
        let u = u.trim_end_matches('/').trim_end_matches(".git").to_string();
        let parts: Vec<&str> = u.split('/').filter(|p| !p.is_empty()).collect();
        if parts.len() >= 3 {
            return parts.join("/").to_lowercase();
        }
    }
    let canonical = fs::canonicalize(root).unwrap_or_else(|_| root.to_path_buf());
    format!(
        "path-{}",
        &digest(canonical.to_string_lossy().as_bytes())[..16]
    )
}
fn run_id() -> String {
    let stamp = now();
    let date = stamp.get(..19).unwrap_or(&stamp).replace(':', "-");
    let salt = &digest(aep_core::unique_id("eval").as_bytes())[..6];
    format!("{date}Z-{salt}")
}
fn project_dir(config: &MachineConfig, root: &Path) -> Result<PathBuf> {
    Ok(eval_root(config)?.join(project_id(root)))
}
fn run_dir(config: &MachineConfig, root: &Path, run: &str) -> Result<PathBuf> {
    if run.is_empty() || run.contains('/') || run.starts_with('.') {
        return Err(Error::input(format!("Invalid run id {run}")));
    }
    Ok(project_dir(config, root)?.join(run))
}
fn latest_run(config: &MachineConfig, root: &Path) -> Result<Option<String>> {
    let dir = project_dir(config, root)?;
    let mut runs: Vec<String> = match fs::read_dir(&dir) {
        Ok(entries) => entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .filter_map(|e| e.file_name().to_str().map(String::from))
            .filter(|n| !n.starts_with('_'))
            .collect(),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => vec![],
        Err(e) => return Err(e.into()),
    };
    runs.sort();
    Ok(runs.pop())
}
fn write_json(path: &Path, value: &Value) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, format!("{}\n", serde_json::to_string_pretty(value)?))?;
    Ok(())
}
fn read_json(path: &Path) -> Result<Value> {
    let text = fs::read_to_string(path)?;
    serde_json::from_str(&text).map_err(|e| Error::input(format!("{}: {e}", path.display())))
}
/// Rewrite SHA256SUMS for every file in the run directory.
fn write_sums(dir: &Path) -> Result<()> {
    let mut files = vec![];
    fn walk(base: &Path, dir: &Path, out: &mut Vec<(String, String)>) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk(base, &path, out)?;
            } else if path.file_name().is_some_and(|n| n != "SHA256SUMS") {
                let rel = path
                    .strip_prefix(base)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/");
                out.push((digest(fs::read(&path)?), rel));
            }
        }
        Ok(())
    }
    walk(dir, dir, &mut files)?;
    files.sort_by(|a, b| a.1.cmp(&b.1));
    let text: String = files.iter().map(|(h, p)| format!("{h}  {p}\n")).collect();
    fs::write(dir.join("SHA256SUMS"), text)?;
    Ok(())
}
fn ensure_manifest(dir: &Path, root: &Path, mode: &str) -> Result<Value> {
    let path = dir.join("manifest.json");
    if path.exists() {
        return read_json(&path);
    }
    let manifest = json!({
        "schema_version": 1,
        "run": dir.file_name().and_then(|n| n.to_str()),
        "project_id": project_id(root),
        "project_root": fs::canonicalize(root).unwrap_or_else(|_| root.to_path_buf()),
        "remote": git(root, &["remote", "get-url", "origin"]).ok(),
        "started_at": now(),
        "mode": mode,
        "cli_version": aep_core::VERSION,
        "cli_binary_sha256": std::env::current_exe().ok().and_then(|p| fs::read(p).ok()).map(digest),
        "guidance_digest": guidance::bundle_digest(),
    });
    write_json(&path, &manifest)?;
    Ok(manifest)
}

// ---------------------------------------------------------------- time helpers
fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}
fn day_of(timestamp: &str) -> Option<i64> {
    let y = timestamp.get(0..4)?.parse().ok()?;
    let m = timestamp.get(5..7)?.parse().ok()?;
    let d = timestamp.get(8..10)?.parse().ok()?;
    Some(days_from_civil(y, m, d))
}
fn today() -> i64 {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    secs.div_euclid(86_400)
}
fn age_days(timestamp: Option<&str>) -> Option<i64> {
    timestamp.and_then(day_of).map(|d| today() - d)
}

// ---------------------------------------------------------------- facts
fn count_by<'a>(records: impl Iterator<Item = &'a Record>) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for r in records {
        *counts
            .entry(format!("{}.{}", r.kind.name(), r.status))
            .or_insert(0) += 1;
    }
    counts
}
fn git_count(root: &Path, args: &[&str]) -> Option<i64> {
    git(root, args).ok().and_then(|s| s.trim().parse().ok())
}
fn read_dir_names(path: &Path) -> Vec<String> {
    fs::read_dir(path)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter_map(|e| e.file_name().to_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}
fn file_contains(path: &Path, needles: &[&str]) -> bool {
    fs::read_to_string(path)
        .map(|t| needles.iter().any(|n| t.contains(n)))
        .unwrap_or(false)
}
pub fn facts(s: &Store, snap: &Snapshot) -> Result<Value> {
    let root = &s.root;
    let records = &snap.records;
    // Records
    let check = commands::check(s, snap)?;
    let stories: Vec<&Record> = records.iter().filter(|r| r.kind == Kind::Story).collect();
    let imported: Vec<&Record> = stories
        .iter()
        .copied()
        .filter(|r| r.status == "imported")
        .collect();
    let receipt = records
        .iter()
        .find(|r| r.kind == Kind::Import && r.text("migration") == Some("v4-context"));
    let legacy_decisions = records
        .iter()
        .filter(|r| {
            r.kind == Kind::Decision
                && r.status == "pending"
                && r.data.keys().any(|k| k.starts_with("legacy"))
        })
        .count();
    let containers_without_description = records
        .iter()
        .filter(|r| matches!(r.kind, Kind::Layer | Kind::Wave) && r.description.trim().is_empty())
        .count();
    let events: Vec<&Record> = records.iter().filter(|r| r.kind == Kind::Event).collect();
    let recent: Vec<&Record> = events
        .iter()
        .copied()
        .filter(|r| age_days(r.created_at.as_deref()).is_some_and(|d| d <= 30))
        .collect();
    let with_note = |list: &[&Record]| {
        list.iter()
            .filter(|r| !r.description.trim().is_empty())
            .count()
    };
    let specs_prefix = format!("{}/specs/", s.config.stores.roadmap);
    let closed_without_spec: Vec<String> = records
        .iter()
        .filter(|r| r.kind == Kind::Change && r.status == "closed")
        .filter(|r| {
            let caps: Vec<String> = r
                .data
                .get("specs")
                .and_then(Value::as_array)
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v["capability"].as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();
            !caps.is_empty()
                && !caps.iter().all(|c| {
                    snap.files
                        .keys()
                        .any(|f| f.starts_with(&format!("{specs_prefix}{c}/")))
                })
        })
        .map(|r| r.id.clone())
        .collect();
    let delivered: std::collections::BTreeSet<&str> = records
        .iter()
        .filter(|r| r.kind == Kind::Delivery)
        .flat_map(|r| r.refs.iter().map(String::as_str))
        .collect();
    let integrated_without_delivery: Vec<String> = stories
        .iter()
        .filter(|r| r.status == "integrated" && !delivered.contains(r.id.as_str()))
        .map(|r| r.id.clone())
        .collect();
    // Verification
    let reviews: Vec<&Record> = records.iter().filter(|r| r.kind == Kind::Review).collect();
    let mut attribution = BTreeMap::new();
    for r in &reviews {
        *attribution
            .entry(r.text("attribution_class").unwrap_or("unknown").to_string())
            .or_insert(0usize) += 1;
    }
    let stale_active: Vec<String> = records
        .iter()
        .filter(|r| r.kind == Kind::Evidence && r.status == "stale")
        .filter(|r| {
            r.refs.iter().any(|id| {
                stories
                    .iter()
                    .any(|s| &s.id == id && matches!(s.status.as_str(), "in_progress" | "ready"))
            })
        })
        .map(|r| r.id.clone())
        .collect();
    // Git
    let branch = git(root, &["rev-parse", "--abbrev-ref", "HEAD"]).ok();
    let dirty = git(root, &["status", "--porcelain"])
        .map(|s| s.lines().filter(|l| !l.trim().is_empty()).count())
        .unwrap_or(0);
    let unpushed = git_count(root, &["rev-list", "--count", "@{u}..HEAD"]);
    let oldest_unpushed_days = git(root, &["log", "--reverse", "--format=%ct", "@{u}..HEAD"])
        .ok()
        .and_then(|s| s.lines().next().and_then(|l| l.trim().parse::<i64>().ok()))
        .map(|secs| today() - secs.div_euclid(86_400));
    let worktrees: Vec<String> = git(root, &["worktree", "list", "--porcelain"])
        .map(|s| {
            s.lines()
                .filter_map(|l| l.strip_prefix("worktree "))
                .skip(1)
                .map(String::from)
                .collect()
        })
        .unwrap_or_default();
    let running: Vec<&Record> = records
        .iter()
        .filter(|r| r.kind == Kind::Attempt && r.status == "running")
        .collect();
    let running_worktrees: Vec<&str> = running.iter().filter_map(|r| r.text("worktree")).collect();
    let orphan_worktrees: Vec<&String> = worktrees
        .iter()
        .filter(|w| {
            w.contains("/.aep/worktrees/")
                && !running_worktrees
                    .iter()
                    .any(|r| w.ends_with(r) || r.ends_with(w.as_str()))
        })
        .collect();
    let oldest_running_days = running
        .iter()
        .filter_map(|r| age_days(r.created_at.as_deref()))
        .max();
    let tags = git(root, &["tag", "-l", "v*"])
        .map(|s| s.lines().filter(|l| !l.is_empty()).count())
        .unwrap_or(0);
    // Legacy
    let legacy_skill_dirs = [".agents/skills", ".claude/skills"]
        .iter()
        .flat_map(|d| read_dir_names(&root.join(d)))
        .filter(|n| n.starts_with("aep-"))
        .count();
    let agents_md = snap.files.get("AGENTS.md");
    let legacy_stores: Vec<&str> = [
        "product-context.yaml",
        "product",
        "project-convention",
        "lessons-learned",
        "openspec",
    ]
    .into_iter()
    .filter(|p| root.join(p).exists())
    .collect();
    let migrate_verify = receipt.map(|_| {
        std::env::current_exe()
            .ok()
            .and_then(|exe| {
                Command::new(exe)
                    .arg("--root")
                    .arg(root)
                    .args(["migrate", "verify", "--json"])
                    .output()
                    .ok()
            })
            .map(|o| o.status.success())
            .unwrap_or(false)
    });
    // DevOps
    let workflows: Vec<String> = read_dir_names(&root.join(".github/workflows"))
        .into_iter()
        .filter(|n| n.ends_with(".yml") || n.ends_with(".yaml"))
        .collect();
    let workflow_text: String = workflows
        .iter()
        .filter_map(|n| fs::read_to_string(root.join(".github/workflows").join(n)).ok())
        .collect::<Vec<_>>()
        .join("\n");
    let checks: Vec<String> = s
        .config
        .checks
        .iter()
        .map(|c| c.command.join(" "))
        .collect();
    let checks_in_ci: Vec<&String> = checks
        .iter()
        .filter(|c| !c.is_empty() && workflow_text.contains(c.as_str()))
        .collect();
    let scan_needles = ["gitleaks", "trufflehog", "detect-secrets", "secretlint"];
    let secret_scan = checks
        .iter()
        .any(|c| scan_needles.iter().any(|n| c.contains(n)))
        || root.join(".gitleaks.toml").exists()
        || file_contains(&root.join("lefthook.yml"), &scan_needles)
        || file_contains(&root.join(".pre-commit-config.yaml"), &scan_needles)
        || scan_needles.iter().any(|n| workflow_text.contains(n));
    let lockfiles: Vec<&str> = [
        "bun.lock",
        "bun.lockb",
        "package-lock.json",
        "pnpm-lock.yaml",
        "yarn.lock",
        "Cargo.lock",
        "uv.lock",
        "poetry.lock",
        "go.sum",
        "Gemfile.lock",
    ]
    .into_iter()
    .filter(|f| root.join(f).exists())
    .collect();
    Ok(json!({
        "schema_version": 1,
        "collected_at": now(),
        "head": git(root, &["rev-parse", "HEAD"]).ok(),
        "records": {
            "check": {"pass": check.data["pass"], "warnings": check.data["warnings"], "diagnostics": check.diagnostics.len() + check.data["diagnostics"].as_array().map(Vec::len).unwrap_or(0)},
            "total": records.len(),
            "by_kind_status": count_by(records.iter()),
            "imported_stories": imported.len(),
            "imported_age_days": receipt.and_then(|r| age_days(r.created_at.as_deref())),
            "legacy_pending_decisions": legacy_decisions,
            "containers_without_description": containers_without_description,
            "events": {"total": events.len(), "with_note": with_note(&events), "recent": recent.len(), "recent_with_note": with_note(&recent)},
            "closed_changes_without_published_spec": closed_without_spec,
            "integrated_stories_without_delivery": integrated_without_delivery,
        },
        "verification": {
            "configured_checks": s.config.checks.len(),
            "required_checks": s.config.policy.required_checks,
            "independent_review": s.config.policy.independent_review,
            "reviews_by_attribution": attribution,
            "blocking_reviews": reviews.iter().filter(|r| r.status == "blocking").count(),
            "stale_evidence": records.iter().filter(|r| r.kind == Kind::Evidence && r.status == "stale").count(),
            "stale_evidence_on_active_stories": stale_active,
        },
        "delivery": {
            "deliveries": records.iter().filter(|r| r.kind == Kind::Delivery).count(),
            "releases": records.iter().filter(|r| r.kind == Kind::Release).count(),
            "closed_changes": records.iter().filter(|r| r.kind == Kind::Change && r.status == "closed").count(),
        },
        "git": {
            "branch": branch,
            "dirty_files": dirty,
            "unpushed_commits": unpushed,
            "oldest_unpushed_days": oldest_unpushed_days,
            "worktrees": worktrees.len(),
            "orphan_attempt_worktrees": orphan_worktrees,
            "running_attempts": running.len(),
            "oldest_running_attempt_days": oldest_running_days,
            "release_tags": tags,
            "protected_paths": s.config.policy.protected_paths,
        },
        "legacy": {
            "v4_skill_dirs": legacy_skill_dirs,
            "agents_md_has_route": agents_md.is_some_and(|t| t.contains("aep-version-route: start")),
            "agents_md_has_entrypoint": agents_md.is_some_and(|t| t.contains("aep --skill")),
            "legacy_stores_present": legacy_stores,
            "migration_receipt": receipt.map(|r| r.id.clone()),
            "migrate_verify_ok": migrate_verify,
        },
        "devops": {
            "ci_workflows": workflows,
            "checks_configured": checks,
            "checks_run_in_ci": checks_in_ci,
            "secret_scan_configured": secret_scan,
            "lockfiles": lockfiles,
            "changelog": root.join("CHANGELOG.md").exists(),
        },
    }))
}

// ---------------------------------------------------------------- rules
struct Finding {
    id: &'static str,
    category: &'static str,
    severity: &'static str,
    message: String,
    evidence: Value,
}
fn n(v: &Value) -> i64 {
    v.as_i64().unwrap_or(0)
}
fn arr_len(v: &Value) -> usize {
    v.as_array().map(Vec::len).unwrap_or(0)
}
fn findings(f: &Value) -> Vec<Finding> {
    let mut out = vec![];
    let mut push = |id, category, severity, message: String, evidence: Value| {
        out.push(Finding {
            id,
            category,
            severity,
            message,
            evidence,
        })
    };
    let v = &f["verification"];
    if n(&v["configured_checks"]) == 0 {
        push("VER-001", "verification", "warn", "No checks are configured in .aep/config.toml, so `aep verify run` records nothing and delivery has no verification evidence.".into(), json!({"configured_checks": 0}));
    } else if arr_len(&v["required_checks"]) == 0 {
        push(
            "VER-002",
            "verification",
            "advice",
            "Checks exist but none is required by policy; a story can be delivered without them."
                .into(),
            json!({"checks": f["devops"]["checks_configured"]}),
        );
    }
    if v["independent_review"] == true {
        let by = &v["reviews_by_attribution"];
        let total: i64 = by.as_object().map(|m| m.values().map(n).sum()).unwrap_or(0);
        let host = n(&by["host_reported"]);
        if total > 0 && host == total {
            push("VER-003", "verification", "advice", "Policy requires independent review, but every recorded review is host_reported (a sub-agent of the same session).".into(), by.clone());
        }
    }
    if arr_len(&v["stale_evidence_on_active_stories"]) > 0 {
        push("VER-004", "verification", "warn", "Stale evidence is attached to stories that are still active; rerun the checks before relying on it.".into(), v["stale_evidence_on_active_stories"].clone());
    }
    let r = &f["records"];
    if r["check"]["pass"] == false {
        push(
            "REC-000",
            "records",
            "warn",
            "`aep check` fails; fix record errors before anything else.".into(),
            r["check"].clone(),
        );
    }
    let imported = n(&r["imported_stories"]);
    let age = r["imported_age_days"].as_i64();
    if imported > 0 && age.is_some_and(|d| d > 14) {
        push(
            "REC-001",
            "records",
            "advice",
            format!(
                "{imported} imported stories are still unreconciled {} days after migration; reconcile with Git evidence, supersede, or reopen them.",
                age.unwrap_or(0)
            ),
            json!({"imported": imported, "days": age}),
        );
    }
    let ev = &r["events"];
    if n(&ev["recent"]) >= 5 && n(&ev["recent_with_note"]) * 2 < n(&ev["recent"]) {
        push(
            "REC-002",
            "records",
            "advice",
            format!(
                "Only {} of {} events in the last 30 days carry a note; add --note to writes so the timeline explains itself.",
                n(&ev["recent_with_note"]),
                n(&ev["recent"])
            ),
            ev.clone(),
        );
    }
    if n(&r["containers_without_description"]) > 0 {
        push(
            "REC-003",
            "records",
            "advice",
            format!(
                "{} layers or waves have no description; a container needs its observable outcome.",
                n(&r["containers_without_description"])
            ),
            json!({"count": r["containers_without_description"]}),
        );
    }
    if n(&r["legacy_pending_decisions"]) > 0 {
        push(
            "REC-004",
            "records",
            "info",
            format!(
                "{} imported decisions remain pending; accept or supersede them so context reflects current authority.",
                n(&r["legacy_pending_decisions"])
            ),
            json!({"count": r["legacy_pending_decisions"]}),
        );
    }
    if arr_len(&r["integrated_stories_without_delivery"]) > 0 {
        push(
            "DEL-002",
            "delivery",
            "warn",
            "Stories are marked integrated without a delivery receipt.".into(),
            r["integrated_stories_without_delivery"].clone(),
        );
    }
    if arr_len(&r["closed_changes_without_published_spec"]) > 0 {
        push(
            "DEL-001",
            "delivery",
            "warn",
            "Closed changes have no published specification under the roadmap store.".into(),
            r["closed_changes_without_published_spec"].clone(),
        );
    }
    let g = &f["git"];
    if g["oldest_unpushed_days"].as_i64().is_some_and(|d| d > 3) {
        push(
            "GIT-001",
            "git",
            "advice",
            format!(
                "{} commits are unpushed; the oldest is {} days old.",
                n(&g["unpushed_commits"]),
                n(&g["oldest_unpushed_days"])
            ),
            json!({"unpushed": g["unpushed_commits"], "days": g["oldest_unpushed_days"]}),
        );
    }
    if n(&g["dirty_files"]) > 0 {
        push(
            "GIT-002",
            "git",
            "info",
            format!(
                "{} uncommitted files at snapshot time.",
                n(&g["dirty_files"])
            ),
            json!({"dirty_files": g["dirty_files"]}),
        );
    }
    if g["oldest_running_attempt_days"]
        .as_i64()
        .is_some_and(|d| d > 7)
    {
        push(
            "GIT-003",
            "git",
            "advice",
            format!(
                "A running attempt is {} days old; finish, record, or cancel it.",
                n(&g["oldest_running_attempt_days"])
            ),
            json!({"days": g["oldest_running_attempt_days"]}),
        );
    }
    if arr_len(&g["orphan_attempt_worktrees"]) > 0 {
        push(
            "GIT-004",
            "git",
            "advice",
            "Attempt worktrees exist without a running attempt record.".into(),
            g["orphan_attempt_worktrees"].clone(),
        );
    }
    let l = &f["legacy"];
    if l["migration_receipt"].is_string()
        && (n(&l["v4_skill_dirs"]) > 0 || l["agents_md_has_route"] == true)
    {
        push("LEG-001", "legacy", "advice", "The project is migrated but still carries installed v4 skills or the AGENTS.md version route; see `aep --skill migrate` cleanup.".into(), json!({"v4_skill_dirs": l["v4_skill_dirs"], "route": l["agents_md_has_route"]}));
    }
    if l["migrate_verify_ok"] == false {
        push("LEG-002", "legacy", "warn", "`aep migrate verify` fails: a retained migration source changed or the entrypoint is missing.".into(), json!({}));
    }
    if l["agents_md_has_entrypoint"] == false {
        push(
            "LEG-003",
            "legacy",
            "warn",
            "AGENTS.md does not point agents at `aep --skill`.".into(),
            json!({}),
        );
    }
    let d = &f["devops"];
    if arr_len(&d["ci_workflows"]) == 0 {
        push("OPS-001", "devops", "advice", "No CI workflow under .github/workflows; configured checks run only when an agent runs them.".into(), json!({}));
    } else if n(&f["verification"]["configured_checks"]) > 0 && arr_len(&d["checks_run_in_ci"]) == 0
    {
        push("OPS-002", "devops", "advice", "CI workflows do not run any of the configured project checks; local and CI verification can diverge.".into(), json!({"checks": d["checks_configured"], "workflows": d["ci_workflows"]}));
    }
    if d["secret_scan_configured"] == false {
        push(
            "OPS-003",
            "devops",
            "advice",
            "No secret scanning is configured (checks, hooks, or CI).".into(),
            json!({}),
        );
    }
    if arr_len(&d["lockfiles"]) == 0 {
        push(
            "OPS-004",
            "devops",
            "info",
            "No dependency lockfile found at the repository root.".into(),
            json!({}),
        );
    }
    if d["changelog"] == false {
        push(
            "OPS-005",
            "devops",
            "info",
            "No CHANGELOG.md at the repository root.".into(),
            json!({}),
        );
    }
    out
}
fn report_value(run: &str, project: &str, snapshot: &Value) -> Value {
    let list = findings(snapshot);
    let counts = ["warn", "advice", "info"]
        .iter()
        .map(|s| {
            (
                s.to_string(),
                list.iter().filter(|f| f.severity == *s).count(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    json!({
        "schema_version": 1,
        "run": run,
        "project_id": project,
        "generated_at": now(),
        "snapshot_collected_at": snapshot["collected_at"],
        "head": snapshot["head"],
        "scope": "Engineering process and AEP adherence only; product decisions, feature value and technical choices are out of scope",
        "counts": counts,
        "findings": list.iter().map(|f| json!({"id": f.id, "category": f.category, "severity": f.severity, "message": f.message, "evidence": f.evidence})).collect::<Vec<_>>(),
    })
}
fn report_markdown(report: &Value) -> String {
    let mut out = format!(
        "# aep eval report\n\nProject: {}\nRun: {}\nHead: {}\nSnapshot: {}\n\nScope: {}\n\n",
        report["project_id"].as_str().unwrap_or(""),
        report["run"].as_str().unwrap_or(""),
        report["head"].as_str().unwrap_or("unknown"),
        report["snapshot_collected_at"].as_str().unwrap_or(""),
        report["scope"].as_str().unwrap_or("")
    );
    let findings = report["findings"].as_array().cloned().unwrap_or_default();
    if findings.is_empty() {
        out.push_str("No findings.\n");
        return out;
    }
    for severity in ["warn", "advice", "info"] {
        let group: Vec<&Value> = findings
            .iter()
            .filter(|f| f["severity"] == severity)
            .collect();
        if group.is_empty() {
            continue;
        }
        out.push_str(&format!("## {severity}\n\n"));
        for f in group {
            out.push_str(&format!(
                "- **{}** ({}): {}\n",
                f["id"].as_str().unwrap_or(""),
                f["category"].as_str().unwrap_or(""),
                f["message"].as_str().unwrap_or("")
            ));
        }
        out.push('\n');
    }
    out
}
fn summary_text(snapshot: &Value) -> String {
    let r = &snapshot["records"];
    let g = &snapshot["git"];
    format!(
        "  records {} · check {} · imported stories {} · events with note {}/{}\n  branch {} · dirty {} · unpushed {} · running attempts {}\n  checks {} · CI workflows {} · secret scan {}\n",
        r["total"],
        if r["check"]["pass"] == true {
            "pass"
        } else {
            "FAIL"
        },
        r["imported_stories"],
        r["events"]["with_note"],
        r["events"]["total"],
        g["branch"].as_str().unwrap_or("?"),
        g["dirty_files"],
        g["unpushed_commits"],
        g["running_attempts"],
        snapshot["verification"]["configured_checks"],
        arr_len(&snapshot["devops"]["ci_workflows"]),
        snapshot["devops"]["secret_scan_configured"],
    )
}

// ---------------------------------------------------------------- commands
pub fn run(args: &Cli, command: &Eval) -> Result<Outcome> {
    let config = load_config()?;
    match command {
        Eval::Init => {
            let home = home()?;
            let path = home.join("config.toml");
            if path.exists() {
                let out = Outcome::ok(json!({"home": home, "config": path, "created": false}));
                return Ok(with_text(
                    out,
                    format!("eval init — {} already exists\n", path.display()),
                ));
            }
            if !args.dry_run {
                fs::create_dir_all(home.join("eval"))?;
                fs::write(
                    &path,
                    toml::to_string_pretty(&MachineConfig::default())
                        .map_err(|e| Error::input(e.to_string()))?,
                )?;
            }
            let mut out = Outcome::ok(
                json!({"home": home, "config": path, "created": !args.dry_run, "dry_run": args.dry_run}),
            );
            out.changed = !args.dry_run;
            Ok(with_text(
                out,
                format!("eval init — wrote {}\n", path.display()),
            ))
        }
        Eval::Snapshot { run } => {
            let s = Store::open(&args.root)?;
            let snap = s.snapshot()?;
            let facts = facts(&s, &snap)?;
            let run = run.clone().unwrap_or_else(run_id);
            let dir = run_dir(&config, &s.root, &run)?;
            let mut result =
                json!({"run": run, "dir": dir, "facts": facts, "dry_run": args.dry_run});
            if !args.dry_run {
                fs::create_dir_all(&dir)?;
                ensure_manifest(&dir, &s.root, "snapshot")?;
                let stamp = facts["collected_at"]
                    .as_str()
                    .unwrap_or("")
                    .replace(':', "-");
                write_json(&dir.join("snapshot.json"), &facts)?;
                write_json(&dir.join("snapshots").join(format!("{stamp}.json")), &facts)?;
                write_sums(&dir)?;
                result["written"] = json!([
                    "manifest.json",
                    "snapshot.json",
                    format!("snapshots/{stamp}.json"),
                    "SHA256SUMS"
                ]);
            }
            let mut out = Outcome::ok(result);
            out.changed = !args.dry_run;
            let text = format!(
                "eval snapshot — run {run}\n{}  saved under {}\n",
                summary_text(&facts),
                dir.display()
            );
            Ok(with_text(out, text))
        }
        Eval::Report { run } => {
            let s = Store::open(&args.root)?;
            let run = match run {
                Some(r) => r.clone(),
                None => latest_run(&config, &s.root)?.ok_or_else(|| {
                    Error::blocked("No eval run for this project; run `aep eval snapshot` first")
                })?,
            };
            let dir = run_dir(&config, &s.root, &run)?;
            let snapshot = read_json(&dir.join("snapshot.json"))
                .map_err(|_| Error::blocked(format!("Run {run} has no snapshot.json")))?;
            let report = report_value(&run, &project_id(&s.root), &snapshot);
            let markdown = report_markdown(&report);
            if !args.dry_run {
                write_json(&dir.join("report.json"), &report)?;
                fs::write(dir.join("report.md"), &markdown)?;
                write_sums(&dir)?;
            }
            let mut out = Outcome::ok(json!({"run": run, "dir": dir, "report": report}));
            out.changed = !args.dry_run;
            Ok(with_text(out, markdown))
        }
        Eval::Record { run, file } => {
            let s = Store::open(&args.root)?;
            let dir = run_dir(&config, &s.root, run)?;
            if !dir.join("manifest.json").exists() {
                return Err(Error::blocked(format!("Unknown eval run {run}")));
            }
            let text = if file.as_os_str() == "-" {
                let mut buf = String::new();
                std::io::Read::read_to_string(&mut std::io::stdin(), &mut buf)?;
                buf
            } else {
                fs::read_to_string(file)?
            };
            let value: Value = serde_json::from_str(&text)
                .map_err(|e| Error::input(format!("Observation is not JSON: {e}")))?;
            if !value.is_object() {
                return Err(Error::input("Observation needs a JSON object"));
            }
            let stamp = now().replace(':', "-");
            let name = format!("observations/{stamp}.json");
            let entry = json!({"file": name, "recorded_at": now(), "note": commands::note(), "digest": digest(text.as_bytes()), "claim": value.get("claim").cloned().unwrap_or(Value::Null)});
            if !args.dry_run {
                write_json(&dir.join(&name), &value)?;
                let index = dir.join("observations").join("index.jsonl");
                let mut line = serde_json::to_string(&entry)?;
                line.push('\n');
                let mut existing = fs::read_to_string(&index).unwrap_or_default();
                existing.push_str(&line);
                fs::write(&index, existing)?;
                write_sums(&dir)?;
            }
            let mut out =
                Outcome::ok(json!({"run": run, "recorded": entry, "dry_run": args.dry_run}));
            out.changed = !args.dry_run;
            Ok(with_text(
                out,
                format!("eval record — saved {name} in run {run}\n"),
            ))
        }
        Eval::List => {
            let s = Store::open(&args.root)?;
            let dir = project_dir(&config, &s.root)?;
            let mut runs = vec![];
            if let Ok(entries) = fs::read_dir(&dir) {
                for e in entries.filter_map(|e| e.ok()).filter(|e| e.path().is_dir()) {
                    let manifest =
                        read_json(&e.path().join("manifest.json")).unwrap_or(Value::Null);
                    let has = |f: &str| e.path().join(f).exists();
                    let observations =
                        fs::read_to_string(e.path().join("observations/index.jsonl"))
                            .map(|t| t.lines().count())
                            .unwrap_or(0);
                    runs.push(json!({"run": e.file_name().to_string_lossy(), "mode": manifest["mode"], "started_at": manifest["started_at"], "snapshot": has("snapshot.json"), "report": has("report.json"), "observations": observations}));
                }
            }
            runs.sort_by(|a, b| a["run"].as_str().cmp(&b["run"].as_str()));
            let mut text = format!(
                "eval runs for {} under {}\n",
                project_id(&s.root),
                dir.display()
            );
            for r in &runs {
                text.push_str(&format!(
                    "  {}  {}  snapshot {}  report {}  observations {}\n",
                    r["run"].as_str().unwrap_or(""),
                    r["mode"].as_str().unwrap_or("?"),
                    r["snapshot"],
                    r["report"],
                    r["observations"]
                ));
            }
            if runs.is_empty() {
                text.push_str("  none\n");
            }
            Ok(with_text(
                Outcome::ok(json!({"project_id": project_id(&s.root), "dir": dir, "runs": runs})),
                text,
            ))
        }
        Eval::Show { run } => {
            let s = Store::open(&args.root)?;
            let dir = run_dir(&config, &s.root, run)?;
            let manifest = read_json(&dir.join("manifest.json"))
                .map_err(|_| Error::blocked(format!("Unknown eval run {run}")))?;
            let snapshot = read_json(&dir.join("snapshot.json")).ok();
            let report = read_json(&dir.join("report.json")).ok();
            let observations: Vec<Value> = fs::read_to_string(dir.join("observations/index.jsonl"))
                .map(|t| {
                    t.lines()
                        .filter_map(|l| serde_json::from_str(l).ok())
                        .collect()
                })
                .unwrap_or_default();
            let mut text = format!(
                "eval run {run}\n  project {}\n  mode {} · started {}\n",
                manifest["project_id"].as_str().unwrap_or(""),
                manifest["mode"].as_str().unwrap_or("?"),
                manifest["started_at"].as_str().unwrap_or("?")
            );
            if let Some(snap) = &snapshot {
                text.push_str(&summary_text(snap));
            }
            if let Some(rep) = &report {
                text.push_str(&format!(
                    "  findings: warn {} · advice {} · info {}\n",
                    rep["counts"]["warn"], rep["counts"]["advice"], rep["counts"]["info"]
                ));
            }
            text.push_str(&format!("  observations: {}\n", observations.len()));
            Ok(with_text(
                Outcome::ok(
                    json!({"run": run, "dir": dir, "manifest": manifest, "snapshot": snapshot, "report": report, "observations": observations}),
                ),
                text,
            ))
        }
        Eval::Watch {
            no_spawn,
            kind,
            target,
        } => watch(args, &config, *no_spawn, kind.as_deref(), target.as_deref()),
    }
}
fn with_text(mut out: Outcome, text: String) -> Outcome {
    out.text = Some(text);
    out
}
fn herdr(args: &[&str]) -> Result<Value> {
    let output = Command::new("herdr")
        .args(args)
        .output()
        .map_err(|e| Error::blocked(format!("herdr is not available: {e}")))?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    if !output.status.success() {
        return Err(Error::new(
            "herdr",
            format!(
                "herdr {} failed: {}",
                args.join(" "),
                if stderr.trim().is_empty() {
                    stdout.trim()
                } else {
                    stderr.trim()
                }
            ),
            1,
        ));
    }
    serde_json::from_str(&stdout).map_err(|_| {
        Error::new(
            "herdr",
            format!("herdr {} returned no JSON", args.join(" ")),
            1,
        )
    })
}
fn watch(
    args: &Cli,
    config: &MachineConfig,
    no_spawn: bool,
    kind: Option<&str>,
    target: Option<&str>,
) -> Result<Outcome> {
    let s = Store::open(&args.root)?;
    if std::env::var("HERDR_ENV").ok().as_deref() != Some("1") {
        return Err(Error::blocked(
            "Not running under Herdr (HERDR_ENV != 1); use `aep eval snapshot` and `aep eval report` directly",
        ));
    }
    let target = target
        .map(String::from)
        .or_else(|| std::env::var("HERDR_PANE_ID").ok())
        .ok_or_else(|| Error::input("No target pane: pass --target <pane-id>"))?;
    let project = project_id(&s.root);
    let kind = kind
        .map(String::from)
        .or_else(|| {
            config
                .eval
                .projects
                .get(&project)
                .and_then(|p| p.observer_kind.clone())
        })
        .unwrap_or_else(|| config.eval.observer_kind.clone());
    let run = run_id();
    let dir = run_dir(config, &s.root, &run)?;
    let root = fs::canonicalize(&s.root).unwrap_or_else(|_| s.root.clone());
    let skill = guidance::asset("eval/SKILL.md")?;
    let prompt = format!(
        "You are the neutral aep eval observer for this project. Parameters:\n- project root: {}\n- eval run id: {run}\n- run directory: {}\n- target pane (the working agent to observe): {target}\n- this pane hosts you; do not send input to the target pane.\n\nFollow the procedure below exactly as written. Read `aep --skill eval --ref observer` and `aep --skill eval --ref rules` for the details.\n\n---\n{}",
        root.display(),
        dir.display(),
        skill
    );
    let name = format!("aep-eval-{}", &run[run.len() - 6..]);
    let mut result = json!({"run": run, "dir": dir, "project_id": project, "target_pane": target, "observer_kind": kind, "observer_name": name, "spawned": false, "dry_run": args.dry_run});
    let commands = json!([
        format!(
            "herdr pane split --current --direction right --cwd {} --no-focus",
            root.display()
        ),
        format!("herdr agent start {name} --kind {kind} --pane <returned pane id>"),
        format!(
            "herdr agent prompt {name} \"$(cat {}/prompt.md)\"",
            dir.display()
        ),
    ]);
    result["commands"] = commands.clone();
    if args.dry_run {
        return Ok(with_text(
            Outcome::ok(result),
            format!("eval watch — dry run for {project}; would create run {run}\n"),
        ));
    }
    fs::create_dir_all(&dir)?;
    let mut manifest = ensure_manifest(&dir, &s.root, "watch")?;
    fs::write(dir.join("prompt.md"), &prompt)?;
    let facts = facts(&s, &s.snapshot()?)?;
    write_json(&dir.join("snapshot.json"), &facts)?;
    manifest["target_pane"] = json!(target);
    manifest["observer"] = json!({"name": name, "kind": kind, "spawned": false});
    if !no_spawn {
        let split = herdr(&[
            "pane",
            "split",
            "--current",
            "--direction",
            "right",
            "--cwd",
            &root.to_string_lossy(),
            "--no-focus",
        ])?;
        let pane = split["result"]["pane"]["pane_id"]
            .as_str()
            .ok_or_else(|| Error::new("herdr", "pane split returned no pane id", 1))?
            .to_string();
        herdr(&["agent", "start", &name, "--kind", &kind, "--pane", &pane])?;
        herdr(&["agent", "prompt", &name, &prompt])?;
        manifest["observer"] = json!({"name": name, "kind": kind, "pane": pane, "spawned": true, "prompted_at": now()});
        result["spawned"] = json!(true);
        result["observer_pane"] = json!(pane);
    }
    write_json(&dir.join("manifest.json"), &manifest)?;
    write_sums(&dir)?;
    let mut out = Outcome::ok(result);
    out.changed = true;
    let text = if no_spawn {
        format!(
            "eval watch — run {run} prepared under {}\n  prompt: {}/prompt.md\n  start the observer yourself:\n    {}\n    {}\n    {}\n",
            dir.display(),
            dir.display(),
            commands[0].as_str().unwrap_or(""),
            commands[1].as_str().unwrap_or(""),
            commands[2].as_str().unwrap_or("")
        )
    } else {
        format!(
            "eval watch — run {run}\n  observer {name} ({kind}) started in pane {} watching {target}\n  run directory {}\n",
            out.data["observer_pane"].as_str().unwrap_or("?"),
            dir.display()
        )
    };
    Ok(with_text(out, text))
}
