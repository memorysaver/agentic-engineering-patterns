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
fn review(root: &Path, findings: Value, code: i32) -> Value {
    let request = call(root, &["review", "request", "--story", "S"], 0);
    let mut response = request["request"]["response"].clone();
    response["findings"] = findings;
    let file = tempfile::NamedTempFile::new().unwrap();
    fs::write(file.path(), response.to_string()).unwrap();
    call(
        root,
        &["review", "record", "--file", file.path().to_str().unwrap()],
        code,
    )
}
#[test]
fn offline_guidance_and_idempotent_setup() {
    let d = tempfile::tempdir().unwrap();
    let offline = Command::new(env!("CARGO_BIN_EXE_aep"))
        .args(["--json", "skills"])
        .current_dir(d.path())
        .env("PATH", "")
        .output()
        .unwrap();
    assert!(
        offline.status.success(),
        "{}",
        String::from_utf8_lossy(&offline.stdout)
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&offline.stdout).unwrap()["operation"],
        "skills"
    );
    let skills = call(d.path(), &["skills"], 0);
    assert!(skills.to_string().contains("design"));
    let selected = call(d.path(), &["skills", "show", "design", "--ref", "bdd"], 0);
    assert!(
        selected["content"]
            .as_str()
            .unwrap()
            .contains("# BDD contracts")
    );
    call(d.path(), &["skills", "route"], 2);
    call(d.path(), &["skills", "show", "missing"], 2);
    let d = setup();
    let root = d.path();
    let before = fs::read(root.join("AGENTS.md")).unwrap();
    call(root, &["init"], 0);
    assert_eq!(before, fs::read(root.join("AGENTS.md")).unwrap());
    write(
        root,
        "project-rules/skills/monet-test/SKILL.md",
        "---\nname: monet-test\ndescription: Project-specific test procedure.\n---\n\nRead project-rules/README.md.\n",
    );
    assert!(
        call(root, &["skills"], 0)
            .to_string()
            .contains("monet-test")
    );
    let config_path = root.join(".aep/config.toml");
    let mut config: aep_core::Config =
        toml::from_str(&fs::read_to_string(&config_path).unwrap()).unwrap();
    config.stores.rules = "./project-rules/".into();
    config.skill_paths = vec!["project-rules/skills/monet-test".into()];
    fs::write(&config_path, toml::to_string(&config).unwrap()).unwrap();
    write(root, "package/.aep/config.toml", "cli_version = '6.0.0'\n");
    let nested = Command::new(env!("CARGO_BIN_EXE_aep"))
        .args(["--json", "skills"])
        .current_dir(root.join("package"))
        .env("PATH", "")
        .output()
        .unwrap();
    assert!(
        nested.status.success(),
        "{}",
        String::from_utf8_lossy(&nested.stdout)
    );
    let catalog: Value = serde_json::from_slice(&nested.stdout).unwrap();
    assert_eq!(
        catalog["data"]["skills"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|s| s["name"] == "monet-test")
            .count(),
        1
    );
    write(
        root,
        "project-rules/skills/collision/SKILL.md",
        "---\nname: design\ndescription: Duplicate.\n---\n\nContent.\n",
    );
    call(root, &["skills"], 2);
}
#[test]
fn local_lifecycle_preserves_evidence_through_integration_and_publication() {
    let d = setup();
    let root = d.path();
    let (attempt, _) = prepared(root);
    call(root, &["verify", "run", "--story", "S"], 0);
    review(root, json!([]), 0);
    assert_eq!(
        call(root, &["deliver", "plan", "--story", "S"], 0)["eligible"],
        true
    );
    call(root, &["deliver", "merge", "--story", "S", "--local"], 0);
    assert_eq!(
        call(root, &["story", "show", "S"], 0)["record"]["status"],
        "integrated"
    );
    call(root, &["attempt", "recover", &attempt, "--cancel"], 3);
    call(root, &["spec", "publish", "--change", "C"], 0);
    call(root, &["change", "close", "C"], 0);
    record(
        root,
        "gate",
        json!({"kind":"gate","id":"G","title":"Layer gate","refs":["L"]}),
    );
    call(root, &["gate", "evaluate", "G"], 0);
    call(root, &["check"], 0);
    assert!(
        fs::read_to_string(root.join("project-roadmap/specs/result/spec.md"))
            .unwrap()
            .contains("Scenario: success")
    );
    record(
        root,
        "lesson",
        json!({"kind":"lesson","id":"lesson-result","title":"Result validation","description":"A result needs a concrete check.","refs":["S"]}),
    );
    assert!(
        call(root, &["lesson", "find", "concrete"], 0)
            .to_string()
            .contains("lesson-result")
    );
}
#[test]
fn changed_bdd_and_new_failed_evidence_block_delivery_and_gates() {
    let d = setup();
    let root = d.path();
    let (_, path) = prepared(root);
    call(root, &["verify", "run", "--story", "S"], 0);
    review(root, json!([]), 0);
    record(
        root,
        "gate",
        json!({"kind":"gate","id":"G","title":"Scope gate","refs":["L"]}),
    );
    call(root, &["gate", "evaluate", "G"], 0);
    record(
        root,
        "story",
        json!({"kind":"story","id":"D","title":"Dependent","change":"C","paths":["other"],"required_gates":["G"]}),
    );
    assert_eq!(
        call(root, &["dispatch", "plan", "--story", "D"], 0)["stories"][0]["ready"],
        true
    );
    write(
        root,
        "project-ledger/changes/C/specs/result/spec.md",
        &BDD.replace("a result exists", "a verified result exists"),
    );
    assert_eq!(
        call(root, &["deliver", "plan", "--story", "S"], 0)["eligible"],
        false
    );
    write(root, "project-ledger/changes/C/specs/result/spec.md", BDD);
    fs::remove_file(path.join("src/result.txt")).unwrap();
    git(&path, &["add", "."]);
    git(&path, &["commit", "-qm", "regression"]);
    call(root, &["verify", "run", "--story", "S"], 1);
    assert_eq!(
        call(root, &["dispatch", "plan", "--story", "D"], 0)["stories"][0]["ready"],
        false
    );
}
#[test]
fn second_review_cannot_erase_blocking_findings() {
    let d = setup();
    let root = d.path();
    prepared(root);
    call(root, &["verify", "run", "--story", "S"], 0);
    review(
        root,
        json!([{"severity":"blocking","description":"Missing required result behavior","resolved":false,"evidence":[]}]),
        0,
    );
    review(root, json!([]), 3);
    assert_eq!(
        call(root, &["deliver", "plan", "--story", "S"], 0)["eligible"],
        false
    );
}
#[test]
fn shared_code_drift_blocks_local_integration() {
    let d = setup();
    let root = d.path();
    prepared(root);
    call(root, &["verify", "run", "--story", "S"], 0);
    review(root, json!([]), 0);
    write(root, "README.md", "Unverified edit\n");
    call(root, &["deliver", "merge", "--story", "S", "--local"], 3);
    assert_eq!(
        fs::read_to_string(root.join("README.md")).unwrap(),
        "Unverified edit\n"
    );
}

fn legacy_repo() -> tempfile::TempDir {
    let d = setup();
    fs::remove_dir_all(d.path().join(".aep")).unwrap();
    fs::remove_dir_all(d.path().join("project-rules")).unwrap();
    fs::remove_file(d.path().join("AGENTS.md")).unwrap();
    fs::remove_file(d.path().join(".gitignore")).unwrap();
    d
}
fn apply_migration(root: &Path, plan: &Value, code: i32) {
    let file = tempfile::NamedTempFile::new().unwrap();
    fs::write(file.path(), plan.to_string()).unwrap();
    call(
        root,
        &["migrate", "apply", "--plan", file.path().to_str().unwrap()],
        code,
    );
}
#[test]
fn migration_preserves_gates_rules_adrs_paths_and_git_sources() {
    let d = legacy_repo();
    let root = d.path();
    let legacy = json!({"stories":[
      {"id":"old","title":"Existing result","status":"completed","layer":"1","files_affected":["src/result.txt"]},
      {"id":"new","title":"Next result","status":"pending","layer":"2","dependencies":["old"],"files_affected":["src/next.txt"]}],
      "layer_gates":[{"layer":"1","status":"passed","test_definition":"Current integration checks"}],
      "architecture":{"adrs":[{"id":"ADR-001","title":"Persist results","decision":"Keep result files"}]}});
    write(root, "product-context.yaml", &legacy.to_string());
    write(
        root,
        "project-convention/README.md",
        "# Rules\nRead security.md\n",
    );
    write(
        root,
        "project-convention/security.md",
        "Protect retained results.\n",
    );
    git(root, &["add", "."]);
    git(root, &["commit", "-qm", "legacy context"]);
    let before = git(root, &["status", "--porcelain"]);
    let result = call(root, &["migrate", "plan", "--dry-run"], 0);
    let plan = &result["plan"];
    assert_eq!(before, git(root, &["status", "--porcelain"]));
    assert_eq!(plan["diagnostics"], json!([]));
    apply_migration(root, plan, 0);
    apply_migration(root, plan, 0);
    let story = call(root, &["story", "show", "new"], 0);
    assert_eq!(story["record"]["required_gates"], json!(["gate-layer-1-1"]));
    assert_eq!(story["record"]["paths"], json!(["src/next.txt"]));
    assert_eq!(
        call(root, &["story", "show", "old"], 0)["record"]["status"],
        "imported"
    );
    call(root, &["decision", "show", "ADR-001"], 0);
    call(root, &["migrate", "verify"], 0);
    assert!(!root.join("project-convention/security.md").exists());
    assert!(root.join("project-rules/security.md").exists());
    call(
        root,
        &[
            "story",
            "reconcile",
            "old",
            "--commit",
            "HEAD",
            "--by",
            "maintainer",
        ],
        0,
    );
    assert_eq!(
        call(root, &["query", "--kind", "evidence"], 0)["records"],
        json!([])
    );
    call(root, &["gate", "evaluate", "gate-layer-1-1"], 3);
}
#[test]
fn migration_scopes_are_independent_and_wave_barriers_are_explicit() {
    let d = legacy_repo();
    let root = d.path();
    write(root, "product-context.yaml", &json!({"stories":[{"id":"S1","status":"pending","layer":1,"openspec_change":"alpha"},{"id":"S2","status":"pending","layer":1,"openspec_change":"beta"}],"waves":[{"layer":1,"wave":1,"stories":["S1"]},{"layer":1,"wave":2,"stories":["S2"]}]}).to_string());
    git(root, &["add", "."]);
    write(
        root,
        "openspec/changes/alpha/proposal.md",
        "Alpha contract with enough context for preserving current project behavior.",
    );
    write(root, "openspec/changes/alpha/specs/result/spec.md", BDD);
    write(
        root,
        "openspec/changes/beta/proposal.md",
        "Beta contract with enough context for preserving current project behavior.",
    );
    write(root, "openspec/changes/beta/specs/result/spec.md", BDD);
    git(root, &["add", "."]);
    git(root, &["commit", "-qm", "legacy waves"]);
    let first = call(root, &["migrate", "plan", "--story", "S1"], 0);
    apply_migration(root, &first["plan"], 0);
    let second = call(root, &["migrate", "plan", "--story", "S2"], 0);
    assert!(second.get("plan").is_some());
    apply_migration(root, &second["plan"], 0);
    assert_eq!(
        call(root, &["story", "show", "S2"], 0)["record"]["depends_on"],
        json!(["S1"])
    );
    assert_eq!(
        call(root, &["story", "show", "S2"], 0)["record"]["change"],
        "beta"
    );
    call(root, &["migrate", "verify"], 0);
}
#[test]
fn migration_refuses_lost_git_sources_collisions_and_empty_verification() {
    let d = legacy_repo();
    let root = d.path();
    call(root, &["migrate", "verify"], 3);
    write(
        root,
        "product-context.yaml",
        &json!({"stories":[{"id":"S","status":"pending"}]}).to_string(),
    );
    let result = call(root, &["migrate", "plan"], 0);
    assert!(!result["plan"]["diagnostics"].as_array().unwrap().is_empty());
    apply_migration(root, &result["plan"], 3);
    write(
        root,
        "product-context.yaml",
        &json!({"stories":[{"id":"imported-product-direction","status":"pending"}]}).to_string(),
    );
    git(root, &["add", "."]);
    git(root, &["commit", "-qm", "colliding context"]);
    call(root, &["migrate", "plan"], 5);
}
#[test]
fn custom_openspec_context_is_retained_and_requires_mapping() {
    let d = setup();
    let root = d.path();
    write(
        root,
        "openspec/config.yaml",
        "schema: security-flow\ncontext: Retain audit records.\n",
    );
    write(
        root,
        "openspec/schemas/security-flow/schema.yaml",
        "name: security-flow\n",
    );
    write(
        root,
        "openspec/changes/C/.openspec.yaml",
        "schema: security-flow\n",
    );
    write(
        root,
        "openspec/changes/C/proposal.md",
        "A result with a security review.\n",
    );
    write(root, "openspec/changes/C/specs/result/spec.md", BDD);
    call(
        root,
        &[
            "openspec",
            "import",
            "--source",
            root.join("openspec").to_str().unwrap(),
        ],
        0,
    );
    call(root, &["change", "accept", "C", "--by", "designer"], 4);
    assert!(
        root.join("project-ledger/imports/openspec-artifacts/config.yaml")
            .exists()
    );
    let exported = d.path().join("exported");
    call(
        root,
        &["openspec", "export", "--output", exported.to_str().unwrap()],
        0,
    );
    assert_eq!(
        fs::read_to_string(exported.join("config.yaml")).unwrap(),
        "schema: security-flow\ncontext: Retain audit records.\n"
    );
}

#[cfg(unix)]
fn provider_fixture(
    scenario: &str,
) -> (tempfile::TempDir, tempfile::TempDir, Vec<(String, String)>) {
    use std::os::unix::fs::PermissionsExt;
    let d = setup();
    let root = d.path();
    let extra = tempfile::tempdir().unwrap();
    let remote = extra.path().join("remote");
    fs::create_dir(&remote).unwrap();
    git(&remote, &["init", "--bare", "-q"]);
    git(root, &["remote", "add", "origin", remote.to_str().unwrap()]);
    let (_, worker) = prepared(root);
    call(root, &["verify", "run", "--story", "S"], 0);
    review(root, json!([]), 0);
    let head = git(&worker, &["rev-parse", "HEAD"]);
    let base = git(root, &["rev-parse", "HEAD"]);
    let provider = extra.path().join("provider");
    git(
        root,
        &[
            "worktree",
            "add",
            "-b",
            "provider",
            provider.to_str().unwrap(),
            &base,
        ],
    );
    let mock = write(extra.path(), "bin/gh", include_str!("fixtures/gh-mock.py"));
    fs::set_permissions(&mock, fs::Permissions::from_mode(0o755)).unwrap();
    let env = vec![
        (
            "PATH".into(),
            format!(
                "{}:{}",
                mock.parent().unwrap().display(),
                std::env::var("PATH").unwrap()
            ),
        ),
        (
            "AEP_TEST_STATE".into(),
            extra.path().join("state.json").to_string_lossy().into(),
        ),
        (
            "AEP_TEST_STORY".into(),
            root.join("project-ledger/stories/S.yaml")
                .to_string_lossy()
                .into(),
        ),
        ("AEP_TEST_SCENARIO".into(), scenario.into()),
        ("AEP_TEST_HEAD".into(), head),
        ("AEP_TEST_BASE".into(), base),
        (
            "AEP_TEST_PROVIDER".into(),
            provider.to_string_lossy().into(),
        ),
    ];
    (d, extra, env)
}
#[cfg(unix)]
fn provider_call(root: &Path, env: &[(String, String)], args: &[&str], code: i32) -> Value {
    let result = Command::new(env!("CARGO_BIN_EXE_aep"))
        .arg("--json")
        .arg("--root")
        .arg(root)
        .envs(env.iter().cloned())
        .args(args)
        .output()
        .unwrap();
    let json: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(result.status.code(), Some(code), "{args:?}: {json}");
    json
}
#[cfg(unix)]
#[test]
fn provider_pr_preserves_concurrent_prose_and_merge_reconciles_lost_response() {
    let (d, _extra, env) = provider_fixture("pr-edit");
    provider_call(
        d.path(),
        &env,
        &["deliver", "pr", "--story", "S", "--base", "main"],
        0,
    );
    assert_eq!(
        call(d.path(), &["story", "show", "S"], 0)["record"]["description"],
        "Concurrent clarification"
    );
    let (d, _extra, env) = provider_fixture("lost-merge");
    let root = d.path();
    provider_call(
        root,
        &env,
        &["deliver", "pr", "--story", "S", "--base", "main"],
        0,
    );
    let result = provider_call(root, &env, &["deliver", "merge", "--story", "S"], 6);
    assert_eq!(result["side_effects"], true);
    assert_eq!(
        call(root, &["story", "show", "S"], 0)["record"]["status"],
        "in_progress"
    );
    provider_call(root, &env, &["deliver", "reconcile", "--story", "S"], 0);
    assert_eq!(
        call(root, &["story", "show", "S"], 0)["record"]["status"],
        "integrated"
    );
}
#[cfg(unix)]
#[test]
fn provider_tree_drift_is_recorded_without_certifying_specification() {
    let (d, _extra, env) = provider_fixture("tree-drift");
    let root = d.path();
    provider_call(
        root,
        &env,
        &["deliver", "pr", "--story", "S", "--base", "main"],
        0,
    );
    provider_call(root, &env, &["deliver", "merge", "--story", "S"], 3);
    assert_eq!(
        call(root, &["story", "show", "S"], 0)["record"]["status"],
        "integrated"
    );
    call(root, &["spec", "publish", "--change", "C"], 3);
}
#[test]
fn integrated_validation_refuses_a_different_or_dirty_worktree() {
    let d = setup();
    let root = d.path();
    let (_, worker) = prepared(root);
    call(root, &["verify", "run", "--story", "S"], 0);
    review(root, json!([]), 0);
    call(root, &["deliver", "merge", "--story", "S", "--local"], 0);
    write(&worker, "src/result.txt", "Unverified edit\n");
    call(root, &["verify", "run", "--story", "S"], 3);
    git(&worker, &["add", "."]);
    git(&worker, &["commit", "-qm", "Later candidate"]);
    call(root, &["verify", "run", "--story", "S"], 3);
}

#[test]
fn rule_adoption_needs_explicit_evaluation_of_the_committed_proposal() {
    let d = setup();
    let root = d.path();
    record(
        root,
        "rule",
        json!({"kind":"rule","id":"R","title":"Check results","description":"Validate that the result file exists."}),
    );
    let mut config: aep_core::Config =
        toml::from_str(&fs::read_to_string(root.join(".aep/config.toml")).unwrap()).unwrap();
    config.checks[0].rules = vec!["R".into()];
    let settings = tempfile::NamedTempFile::new().unwrap();
    fs::write(settings.path(), serde_json::to_string(&config).unwrap()).unwrap();
    let revision = call(root, &["config", "show"], 0)["revision"]
        .as_str()
        .unwrap()
        .to_string();
    call(
        root,
        &[
            "config",
            "update",
            "--file",
            settings.path().to_str().unwrap(),
            "--expect",
            &revision,
        ],
        0,
    );
    prepared(root);
    call(root, &["verify", "run", "--story", "S"], 0);
    review(root, json!([]), 0);
    call(root, &["deliver", "merge", "--story", "S", "--local"], 0);
    let result = call(root, &["query", "--kind", "evidence"], 0);
    let evidence = result["records"][0]["record"]["id"].as_str().unwrap();
    call(root, &["rule", "adopt", "R", "--by", "maintainer"], 3);
    call(
        root,
        &[
            "rule",
            "adopt",
            "R",
            "--by",
            "maintainer",
            "--evidence",
            evidence,
        ],
        0,
    );
    assert_eq!(
        call(root, &["rule", "show", "R"], 0)["record"]["status"],
        "adopted"
    );
}
#[test]
fn release_promotion_requires_current_environment_gate_evidence() {
    let d = setup();
    let root = d.path();
    let mut config: aep_core::Config =
        toml::from_str(&fs::read_to_string(root.join(".aep/config.toml")).unwrap()).unwrap();
    let mut stage = config.checks[0].clone();
    stage.id = "stage".into();
    stage.environment = "staging-fixture".into();
    stage.required = false;
    config.checks.push(stage);
    fs::write(
        root.join(".aep/config.toml"),
        toml::to_string_pretty(&config).unwrap(),
    )
    .unwrap();
    prepared(root);
    call(root, &["verify", "run", "--story", "S"], 0);
    review(root, json!([]), 0);
    call(root, &["deliver", "merge", "--story", "S", "--local"], 0);
    record(
        root,
        "gate",
        json!({"kind":"gate","id":"GR","title":"Staging gate","refs":["S"],"required_checks":["stage"],"data":{"environment":"staging-fixture"}}),
    );
    record(
        root,
        "release",
        json!({"kind":"release","id":"R","title":"Release","refs":["S"],"required_gates":["GR"]}),
    );
    call(
        root,
        &[
            "release",
            "promote",
            "R",
            "--environment",
            "staging-fixture",
            "--by",
            "maintainer",
        ],
        3,
    );
    call(
        root,
        &["verify", "run", "--story", "S", "--check", "stage"],
        0,
    );
    call(root, &["gate", "evaluate", "GR"], 0);
    call(
        root,
        &[
            "release",
            "promote",
            "R",
            "--environment",
            "staging-fixture",
            "--by",
            "maintainer",
        ],
        0,
    );
    assert_eq!(
        call(root, &["story", "show", "S"], 0)["record"]["status"],
        "released"
    );
}

#[test]
fn mapping_drift_blocks_acceptance_until_explicitly_reconciled() {
    let d = setup();
    let root = d.path();
    write(
        root,
        "openspec/config.yaml",
        "schema: security-flow\ncontext: Threat review is required.\n",
    );
    call(
        root,
        &[
            "openspec",
            "import",
            "--source",
            root.join("openspec").to_str().unwrap(),
        ],
        0,
    );
    record(
        root,
        "change",
        json!({"kind":"change","id":"C","title":"Guidance","description":"Retain the required project threat review when guidance changes.","data":{"documentation_only":true}}),
    );
    call(root, &["change", "accept", "C", "--by", "designer"], 4);
    let imported = call(root, &["query", "--kind", "import"], 0)["records"][0]["record"].clone();
    let target = "project-rules/security.md";
    write(
        root,
        target,
        "Require threat review before accepting a behavior change.\n",
    );
    let file = tempfile::NamedTempFile::new().unwrap();
    fs::write(
        file.path(),
        json!({"record":imported["id"],"by":"maintainer","mappings":{"config.yaml":target}})
            .to_string(),
    )
    .unwrap();
    call(
        root,
        &["openspec", "map", "--file", file.path().to_str().unwrap()],
        0,
    );
    call(root, &["check"], 0);
    fs::remove_file(root.join(target)).unwrap();
    call(root, &["check"], 1);
    call(root, &["change", "accept", "C", "--by", "designer"], 4);
    write(
        root,
        target,
        "Require an attributed threat review before change acceptance.\n",
    );
    call(
        root,
        &["openspec", "map", "--file", file.path().to_str().unwrap()],
        0,
    );
    call(root, &["change", "accept", "C", "--by", "designer"], 0);
}

#[test]
fn config_can_repair_constraints_but_cannot_abandon_rule_only_stores() {
    let d = setup();
    let root = d.path();
    let path = root.join(".aep/config.toml");
    let mut config: aep_core::Config = toml::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
    config.policy.max_parallel = 0;
    fs::write(&path, toml::to_string(&config).unwrap()).unwrap();
    let revision = call(root, &["config", "show"], 0)["revision"]
        .as_str()
        .unwrap()
        .to_string();
    config.policy.max_parallel = 2;
    let file = tempfile::Builder::new().suffix(".toml").tempfile().unwrap();
    fs::write(file.path(), toml::to_string(&config).unwrap()).unwrap();
    call(
        root,
        &[
            "config",
            "update",
            "--file",
            file.path().to_str().unwrap(),
            "--expect",
            &revision,
        ],
        0,
    );
    let revision = call(root, &["config", "show"], 0)["revision"]
        .as_str()
        .unwrap()
        .to_string();
    config.stores.rules = "new-rules".into();
    fs::write(file.path(), toml::to_string(&config).unwrap()).unwrap();
    call(
        root,
        &[
            "config",
            "update",
            "--file",
            file.path().to_str().unwrap(),
            "--expect",
            &revision,
        ],
        3,
    );
    assert!(root.join("project-rules/README.md").exists());
}

#[test]
fn empty_active_migration_retains_split_product_adrs_and_lesson_links() {
    let d = legacy_repo();
    let root = d.path();
    write(root, "product-context.yaml", &json!({"stories":[{"id":"old","status":"completed"}],"architecture":{"adrs":[{"id":"ADR-old","title":"Encryption","decision":"Retain encryption"}]}}).to_string());
    write(
        root,
        "product/index.yaml",
        "product:\n  name: Current product\n  decisions: []\ncapabilities:\n  - id: secure\n    map_path: maps/secure\n",
    );
    write(
        root,
        "product/maps/secure/map.yaml",
        "journey: encrypted conversation\n",
    );
    write(root, "docs/design.md", "Current design context.\n");
    write(
        root,
        "lessons-learned/README.md",
        "Read [timeout](notes/timeout.md) and [design](../docs/design.md).\n",
    );
    write(
        root,
        "lessons-learned/notes/timeout.md",
        "Retain timeout evidence.\n",
    );
    git(root, &["add", "."]);
    git(root, &["commit", "-qm", "split legacy context"]);
    let plan = call(root, &["migrate", "plan"], 0);
    apply_migration(root, &plan["plan"], 0);
    call(root, &["decision", "show", "ADR-old"], 0);
    assert!(
        call(root, &["roadmap", "show", "imported-product-direction"], 0)
            .to_string()
            .contains("Current product")
    );
    assert!(
        root.join("project-roadmap/product/maps/secure/map.yaml")
            .exists()
    );
    assert_eq!(
        fs::read_to_string(root.join("lesson-learned/README.md")).unwrap(),
        "Read [timeout](notes/timeout.md) and [design](../docs/design.md).\n"
    );
    assert!(
        call(root, &["lesson", "find", "timeout"], 0)
            .to_string()
            .contains("Retain timeout evidence")
    );
    call(root, &["migrate", "verify"], 0);
}
