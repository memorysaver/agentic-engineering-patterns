use serde_json::json;
use std::{fs, path::Path, process::Command};

const ATTEMPT: &str = "attempt-a1b2c3d4e5f60708-aaaa-0";

fn git(root: &Path, args: &[&str]) {
    let out = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(out.status.success(), "git {args:?}");
}
fn aep(root: &Path, args: &[&str]) -> (i32, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_aep"))
        .env_remove("AEP_CHECK_RUN")
        .arg("--root")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    (out.status.code().unwrap(), text)
}
/// A repository with an initialized project and one attempt record.
fn setup() -> tempfile::TempDir {
    let d = tempfile::tempdir().unwrap();
    let root = d.path();
    git(root, &["init", "-q", "-b", "main"]);
    git(root, &["config", "user.email", "fixture@example.invalid"]);
    git(root, &["config", "user.name", "Fixture"]);
    fs::write(root.join("README.md"), "Fixture\n").unwrap();
    git(root, &["add", "."]);
    git(root, &["commit", "-qm", "base"]);
    let (code, text) = aep(root, &["init"]);
    assert_eq!(code, 0, "{text}");
    // A configured check the progress file may name as its check-id evidence.
    let config = root.join(".aep/config.toml");
    let mut text = fs::read_to_string(&config)
        .unwrap()
        .replace("\nchecks = []\n", "\n");
    text.push_str(
        "\n[[checks]]\nid = \"rust-tests\"\ncommand = [\"true\"]\nenvironment = \"local\"\nrules = []\ncwd = \".\"\ntimeout_seconds = 60\npaths = []\nenv = []\nrequired = true\n",
    );
    fs::write(&config, text).unwrap();
    let attempt = json!({
        "schema_version": 1, "kind": "attempt", "id": ATTEMPT, "title": "Fix a bug",
        "description": "", "status": "running", "refs": [], "paths": [], "depends_on": [],
        "layer": null, "wave": null, "release": null, "change": null, "changes": [],
        "owner": null, "required_checks": [], "required_gates": [], "risk": "standard",
        "created_at": "2026-01-01T00:00:00.000Z", "updated_at": "2026-01-01T00:00:00.000Z", "data": {}
    });
    let dir = root.join("project-ledger/attempts");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join(format!("{ATTEMPT}.yaml")), attempt.to_string()).unwrap();
    d
}
/// Reads a principle through the CLI, so the read log records it for the attempt.
fn read(root: &Path, principle: &str) {
    let (code, text) = aep(
        root,
        &[
            "--skill",
            "principles",
            "--ref",
            principle,
            "--attempt",
            ATTEMPT,
        ],
    );
    assert_eq!(code, 0, "{text}");
}
/// A complete progress file for the bug-fix runbook; step times are later than the reads.
fn complete(root: &Path) -> std::path::PathBuf {
    let later = "2999-01-01T00:00:00.000Z";
    let progress = [
        "runbook: bug-fix".to_string(),
        format!("attempt: {ATTEMPT}"),
        "steps:".to_string(),
        "  reproduce:".to_string(),
        "    status: done".to_string(),
        format!("    at: \"{later}\""),
        "    evidence:".to_string(),
        "      command-output: \"fails\"".to_string(),
        "    principles_read: [prove-it]".to_string(),
        "  root-cause:".to_string(),
        "    status: done".to_string(),
        format!("    at: \"{later}\""),
        "    evidence:".to_string(),
        "      file-content: \"guidance.rs:40\"".to_string(),
        "  test-first:".to_string(),
        "    status: done".to_string(),
        format!("    at: \"{later}\""),
        "    evidence:".to_string(),
        "      command-output: \"red\"".to_string(),
        "  fix:".to_string(),
        "    status: done".to_string(),
        format!("    at: \"{later}\""),
        "    evidence:".to_string(),
        "      git-ref: \"HEAD\"".to_string(),
        "  verify:".to_string(),
        "    status: done".to_string(),
        format!("    at: \"{later}\""),
        "    evidence:".to_string(),
        "      check-id: \"rust-tests\"".to_string(),
        "    principles_read: [prove-it, verifiable-units]".to_string(),
        "  learn:".to_string(),
        "    status: skipped".to_string(),
        "    reason: \"no repeat found\"".to_string(),
        String::new(),
    ]
    .join("\n");
    let path = root.join("progress.yaml");
    fs::write(&path, progress).unwrap();
    path
}

#[test]
fn a_complete_progress_file_is_accepted() {
    let d = setup();
    let root = d.path();
    read(root, "prove-it");
    read(root, "verifiable-units");
    let path = complete(root);
    let (code, text) = aep(root, &["runbook", "check", path.to_str().unwrap()]);
    assert_eq!(code, 0, "{text}");
}

#[test]
fn a_step_missing_from_the_file_is_refused() {
    let d = setup();
    let root = d.path();
    read(root, "prove-it");
    read(root, "verifiable-units");
    let path = complete(root);
    let text = fs::read_to_string(&path).unwrap().replace(
        "  learn:\n    status: skipped\n    reason: \"no repeat found\"\n",
        "",
    );
    fs::write(&path, text).unwrap();
    let (code, out) = aep(root, &["runbook", "check", path.to_str().unwrap()]);
    assert_eq!(code, 2, "{out}");
    assert!(out.contains("learn") && out.contains("missing"), "{out}");
}

#[test]
fn a_done_step_without_its_evidence_is_refused_naming_the_step_and_kind() {
    let d = setup();
    let root = d.path();
    read(root, "prove-it");
    read(root, "verifiable-units");
    let path = complete(root);
    let text = fs::read_to_string(&path)
        .unwrap()
        .replace("      command-output: \"fails\"\n", "");
    fs::write(&path, text).unwrap();
    let (code, out) = aep(root, &["runbook", "check", path.to_str().unwrap()]);
    assert_eq!(code, 2, "{out}");
    assert!(
        out.contains("reproduce") && out.contains("command-output"),
        "{out}"
    );
}

#[test]
fn a_principle_not_read_in_the_attempt_is_refused() {
    let d = setup();
    let root = d.path();
    read(root, "prove-it");
    let path = complete(root);
    let (code, out) = aep(root, &["runbook", "check", path.to_str().unwrap()]);
    assert_eq!(code, 2, "{out}");
    assert!(out.contains("verifiable-units"), "{out}");
}

#[test]
fn a_read_after_the_step_time_is_refused() {
    let d = setup();
    let root = d.path();
    read(root, "prove-it");
    read(root, "verifiable-units");
    let path = complete(root);
    let text = fs::read_to_string(&path).unwrap().replace(
        "    at: \"2999-01-01T00:00:00.000Z\"\n    evidence:\n      command-output: \"fails\"",
        "    at: \"2000-01-01T00:00:00.000Z\"\n    evidence:\n      command-output: \"fails\"",
    );
    fs::write(&path, text).unwrap();
    let (code, out) = aep(root, &["runbook", "check", path.to_str().unwrap()]);
    assert_eq!(code, 2, "{out}");
    assert!(
        out.contains("prove-it") && out.contains("reproduce"),
        "{out}"
    );
}

#[test]
fn a_skipped_step_without_a_reason_is_refused() {
    let d = setup();
    let root = d.path();
    read(root, "prove-it");
    read(root, "verifiable-units");
    let path = complete(root);
    let text = fs::read_to_string(&path)
        .unwrap()
        .replace("    reason: \"no repeat found\"\n", "");
    fs::write(&path, text).unwrap();
    let (code, out) = aep(root, &["runbook", "check", path.to_str().unwrap()]);
    assert_eq!(code, 2, "{out}");
    assert!(out.contains("learn") && out.contains("reason"), "{out}");
}

#[test]
fn an_unknown_attempt_is_refused() {
    let d = setup();
    let root = d.path();
    let path = complete(root);
    let text = fs::read_to_string(&path)
        .unwrap()
        .replace(ATTEMPT, "attempt-ffffffffffffffff-ffff-0");
    fs::write(&path, text).unwrap();
    let (code, out) = aep(root, &["runbook", "check", path.to_str().unwrap()]);
    assert_eq!(code, 2, "{out}");
    assert!(out.contains("not in the store"), "{out}");
}

#[test]
fn an_unresolved_check_id_is_refused() {
    let d = setup();
    let root = d.path();
    read(root, "prove-it");
    read(root, "verifiable-units");
    let path = complete(root);
    let text = fs::read_to_string(&path)
        .unwrap()
        .replace("check-id: \"rust-tests\"", "check-id: \"no-such-check\"");
    fs::write(&path, text).unwrap();
    let (code, out) = aep(root, &["runbook", "check", path.to_str().unwrap()]);
    assert_eq!(code, 2, "{out}");
    assert!(out.contains("no-such-check"), "{out}");
}

#[test]
fn invalid_and_out_of_order_times_are_refused() {
    let d = setup();
    let root = d.path();
    read(root, "prove-it");
    read(root, "verifiable-units");
    let path = complete(root);
    let original = fs::read_to_string(&path).unwrap();
    fs::write(
        &path,
        original.replace("2999-01-01T00:00:00.000Z", "nonsense"),
    )
    .unwrap();
    assert_eq!(
        aep(root, &["runbook", "check", path.to_str().unwrap()]).0,
        2
    );
    fs::write(
        &path,
        original.replacen("2999-01-01T00:00:00.000Z", "3000-01-01T00:00:00.000Z", 1),
    )
    .unwrap();
    let (code, out) = aep(root, &["runbook", "check", path.to_str().unwrap()]);
    assert_eq!(code, 2);
    assert!(out.contains("previous completed step"), "{out}");
}
