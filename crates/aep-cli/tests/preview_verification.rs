use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn git(root: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim().into()
}
fn call(root: &Path, args: &[&str], code: i32) -> Value {
    let out = Command::new(env!("CARGO_BIN_EXE_aep"))
        .arg("--json")
        .arg("--root")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    let result: Value =
        serde_json::from_slice(&out.stdout).unwrap_or_else(|_| panic!("invalid JSON: {:?}", out));
    assert_eq!(out.status.code(), Some(code), "{args:?}: {result}");
    assert_eq!(result["exit_code"], code);
    result["data"].clone()
}
fn write(root: &Path, path: &str, content: &str) -> PathBuf {
    let path = root.join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, content).unwrap();
    path
}
fn record(root: &Path, kind: &str, value: Value) {
    let file = tempfile::NamedTempFile::new().unwrap();
    fs::write(file.path(), value.to_string()).unwrap();
    call(
        root,
        &[kind, "new", "--file", file.path().to_str().unwrap()],
        0,
    );
}
fn setup() -> tempfile::TempDir {
    let d = tempfile::tempdir().unwrap();
    let root = d.path();
    git(root, &["init", "-q", "-b", "main"]);
    git(root, &["config", "user.email", "fixture@example.invalid"]);
    git(root, &["config", "user.name", "Fixture"]);
    write(root, "README.md", "Fixture\n");
    git(root, &["add", "."]);
    git(root, &["commit", "-qm", "base"]);
    call(root, &["init"], 0);
    let config = root.join(".aep/config.toml");
    let mut settings: aep_core::Config =
        toml::from_str(&fs::read_to_string(&config).unwrap()).unwrap();
    settings.checks.push(aep_core::Check {
        id: "test".into(),
        environment: "local".into(),
        rules: vec![],
        command: vec!["sh".into(), "-c".into(), "test -f src/result.txt".into()],
        cwd: ".".into(),
        timeout_seconds: 5,
        paths: vec![],
        env: vec![],
        required: true,
    });
    fs::write(config, toml::to_string_pretty(&settings).unwrap()).unwrap();
    d
}
const BDD: &str = "## ADDED Requirements\n### Requirement: Result\nThe system SHALL write a result.\n#### Scenario: success\n- **GIVEN** an input\n- **WHEN** work completes\n- **THEN** a result exists\n";
fn prepared(root: &Path) -> (String, PathBuf) {
    let delta = "project-ledger/changes/C/specs/result/spec.md";
    write(root, delta, BDD);
    record(
        root,
        "change",
        json!({"kind":"change","id":"C","title":"Result contract","description":"Create a result file.","data":{"specs":[{"capability":"result","path":delta,"baseline":null}]}}),
    );
    call(root, &["change", "accept", "C", "--by", "designer"], 0);
    record(
        root,
        "layer",
        json!({"kind":"layer","id":"L","title":"Layer"}),
    );
    record(
        root,
        "story",
        json!({"kind":"story","id":"S","title":"Implement result","change":"C","layer":"L","paths":["src"]}),
    );
    git(root, &["add", "."]);
    git(root, &["commit", "-qm", "accepted design"]);
    let data = call(
        root,
        &[
            "dispatch", "start", "--story", "S", "--base", "HEAD", "--owner", "builder",
        ],
        0,
    );
    assert_eq!(data["worker_started"], false);
    let id = data["attempt"]["record"]["id"].as_str().unwrap().into();
    let path = PathBuf::from(data["worktree"].as_str().unwrap());
    write(&path, "src/result.txt", "result\n");
    git(&path, &["add", "."]);
    git(&path, &["commit", "-qm", "implement result"]);
    (id, path)
}

fn response(root: &Path, value: &Value, code: i32) {
    let file = tempfile::NamedTempFile::new().unwrap();
    fs::write(file.path(), value.to_string()).unwrap();
    call(
        root,
        &["review", "record", "--file", file.path().to_str().unwrap()],
        code,
    );
}
fn requested(root: &Path) -> Value {
    call(root, &["review", "request", "--story", "S"], 0)["request"]["response"].clone()
}
fn check(root: &Path) -> String {
    call(root, &["verify", "run", "--story", "S"], 0);
    let plan = call(root, &["verify", "plan", "--story", "S"], 0);
    call(root, &["query", "--kind", "evidence"], 0)["records"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| &value["record"])
        .find(|record| record["data"]["fingerprint"] == plan["fingerprint"])
        .unwrap()["id"]
        .as_str()
        .unwrap()
        .to_string()
}
fn eligible(root: &Path) -> bool {
    call(root, &["deliver", "plan", "--story", "S"], 0)["eligible"]
        .as_bool()
        .unwrap()
}
fn gate(root: &Path) {
    record(
        root,
        "gate",
        json!({"kind":"gate","id":"G","title":"Result gate","refs":["S"]}),
    );
}

#[test]
fn default_self_verification_completes_standard_lifecycle_and_keeps_stale_checks_blocked() {
    let d = setup();
    let root = d.path();
    let (_, worker) = prepared(root);
    let plan = call(root, &["verify", "plan", "--story", "S"], 0);
    assert_eq!(plan["risk"], "standard");
    assert_eq!(plan["independent_review"], false);
    assert!(!eligible(root));
    check(root);
    assert!(eligible(root));
    gate(root);
    call(root, &["gate", "evaluate", "G"], 0);
    write(&worker, "src/result.txt", "Updated result\n");
    git(&worker, &["add", "."]);
    git(&worker, &["commit", "-qm", "Update result"]);
    assert!(!eligible(root));
    call(root, &["gate", "evaluate", "G"], 3);
    check(root);
    call(root, &["deliver", "merge", "--story", "S", "--local"], 0);
    call(root, &["spec", "publish", "--change", "C"], 0);
    call(root, &["change", "close", "C"], 0);
    call(root, &["gate", "evaluate", "G"], 0);
    call(root, &["check"], 0);
    assert!(
        call(root, &["query", "--kind", "review"], 0)["records"]
            .as_array()
            .unwrap()
            .is_empty()
    );
}

#[test]
fn explicit_independent_policy_survives_init_and_requires_current_review() {
    let d = setup();
    let root = d.path();
    let path = root.join(".aep/config.toml");
    let mut config: aep_core::Config = toml::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
    config.policy.independent_review = true;
    fs::write(&path, toml::to_string_pretty(&config).unwrap()).unwrap();
    call(root, &["init"], 0);
    let (_, worker) = prepared(root);
    assert_eq!(
        call(root, &["verify", "plan", "--story", "S"], 0)["independent_review"],
        true
    );
    check(root);
    gate(root);
    assert!(!eligible(root));
    call(root, &["gate", "evaluate", "G"], 3);
    let mut result = requested(root);
    result["reviewer"] = json!("builder");
    response(root, &result, 3);
    result["reviewer"] = json!("reviewer");
    response(root, &result, 0);
    assert!(eligible(root));
    write(&worker, "src/result.txt", "A revised result\n");
    git(&worker, &["add", "."]);
    git(&worker, &["commit", "-qm", "New candidate"]);
    check(root);
    assert!(!eligible(root));
    // A formerly passing review does not cap new review at two rounds.
    for _ in 0..3 {
        let result = requested(root);
        response(root, &result, 0);
    }
    let mut blocked = requested(root);
    blocked["findings"] = json!([{"severity":"blocking","description":"Require a confirmed correction","resolved":false,"evidence":[]}]);
    response(root, &blocked, 0);
    let evidence = check(root);
    blocked["attestation"] = json!(true);
    blocked["reviewer"] = json!("builder");
    blocked["findings"][0]["resolved"] = json!(true);
    blocked["findings"][0]["evidence"] = json!([evidence]);
    response(root, &blocked, 3);
    let mut confirmed = requested(root);
    confirmed["findings"] = blocked["findings"].clone();
    response(root, &confirmed, 0);
    call(root, &["gate", "evaluate", "G"], 0);
    call(root, &["deliver", "merge", "--story", "S", "--local"], 0);
}

#[test]
fn protected_paths_classify_risk_without_forcing_review_topology() {
    let d = setup();
    let root = d.path();
    let path = root.join(".aep/config.toml");
    let mut config: aep_core::Config = toml::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
    config.policy.protected_paths.push("src".into());
    fs::write(path, toml::to_string_pretty(&config).unwrap()).unwrap();
    prepared(root);
    let plan = call(root, &["verify", "plan", "--story", "S"], 0);
    assert_eq!(plan["risk"], "deep");
    assert_eq!(plan["independent_review"], false);
    check(root);
    assert!(eligible(root));
}

#[test]
fn optional_findings_survive_revision_drift_and_allow_evidence_backed_self_resolution() {
    let d = setup();
    let root = d.path();
    let (_, worker) = prepared(root);
    let old_evidence = check(root);
    gate(root);
    let mut result = requested(root);
    result["findings"] = json!([
        {"severity":"blocking","description":"The result needs a heading","resolved":false,"evidence":[]},
        {"severity":"material","description":"The result needs a trailing newline","resolved":false,"evidence":[]}
    ]);
    response(root, &result, 0);
    assert!(!eligible(root));
    call(root, &["gate", "evaluate", "G"], 3);
    write(&worker, "src/result.txt", "# Result\n");
    git(&worker, &["add", "."]);
    git(&worker, &["commit", "-qm", "Fix findings"]);
    let evidence = check(root);
    assert!(!eligible(root));
    let current = call(root, &["verify", "plan", "--story", "S"], 0);
    result["head"] = current["head"].clone();
    result["fingerprint"] = current["fingerprint"].clone();
    result["attestation"] = json!(true);
    result["reviewer"] = json!("builder");
    for finding in result["findings"].as_array_mut().unwrap() {
        finding["resolved"] = json!(true);
        finding["evidence"] = json!([old_evidence]);
    }
    response(root, &result, 3);
    for finding in result["findings"].as_array_mut().unwrap() {
        finding["evidence"] = json!([evidence]);
    }
    response(root, &result, 0);
    assert!(eligible(root));
    call(root, &["gate", "evaluate", "G"], 0);
    // Resolved findings do not reappear in later review requests.
    let followup = call(root, &["review", "request", "--story", "S"], 0);
    assert_eq!(
        followup["request"]["record"]["data"]["required_findings"],
        json!([])
    );
    response(root, &followup["request"]["response"], 0);
    assert!(eligible(root));
}

#[test]
fn reviews_carry_unresolved_material_findings_and_allow_replacing_stale_requests() {
    let d = setup();
    let root = d.path();
    let (_, worker) = prepared(root);
    check(root);
    let stale = requested(root);
    call(root, &["review", "request", "--story", "S"], 3);
    write(&worker, "src/result.txt", "New candidate\n");
    git(&worker, &["add", "."]);
    git(&worker, &["commit", "-qm", "Replace candidate"]);
    check(root);
    let mut result = requested(root);
    response(root, &stale, 3);
    result["findings"] = json!([{"severity":"material","description":"Missing result metadata","resolved":false,"evidence":[]}]);
    response(root, &result, 0);
    let mut next = requested(root);
    response(root, &next, 3);
    next["findings"] = result["findings"].clone();
    response(root, &next, 0);
    assert!(!eligible(root));
    let third = call(root, &["review", "request", "--story", "S"], 0);
    assert_eq!(
        third["request"]["record"]["data"]["required_findings"],
        result["findings"]
    );
}

#[test]
fn replacing_an_attempt_cannot_drop_the_storys_known_review_findings() {
    let d = setup();
    let root = d.path();
    let (attempt, _) = prepared(root);
    check(root);
    let mut result = requested(root);
    result["findings"] = json!([{"severity":"blocking","description":"The result needs metadata","resolved":false,"evidence":[]}]);
    response(root, &result, 0);
    call(root, &["attempt", "recover", &attempt, "--cancel"], 0);
    git(root, &["add", "."]);
    git(root, &["commit", "-qm", "Prepare retry"]);
    let started = call(
        root,
        &[
            "dispatch",
            "start",
            "--story",
            "S",
            "--base",
            "HEAD",
            "--owner",
            "new-builder",
        ],
        0,
    );
    let worker = PathBuf::from(started["worktree"].as_str().unwrap());
    write(&worker, "src/result.txt", "Result with metadata\n");
    git(&worker, &["add", "."]);
    git(&worker, &["commit", "-qm", "Retry with metadata"]);
    let evidence = check(root);
    assert!(!eligible(root));
    let mut confirmed = requested(root);
    response(root, &confirmed, 3);
    confirmed["findings"] = result["findings"].clone();
    confirmed["findings"][0]["resolved"] = json!(true);
    confirmed["findings"][0]["evidence"] = json!([evidence]);
    response(root, &confirmed, 0);
    assert!(eligible(root));
}
