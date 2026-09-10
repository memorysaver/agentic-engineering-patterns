use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

fn git(root: &Path, args: &[&str]) {
    let out = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}
fn write(root: &Path, path: &str, body: &str) {
    let path = root.join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, body).unwrap();
}
fn call(root: &Path, args: &[&str], code: i32) -> Value {
    let output = Command::new(env!("CARGO_BIN_EXE_aep"))
        .arg("--json")
        .arg("--root")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    let result: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(output.status.code(), Some(code), "{args:?}: {result}");
    result["data"].clone()
}
fn commit(root: &Path) {
    git(root, &["add", "."]);
    git(root, &["commit", "-qm", "fixture"]);
}
fn repo(legacy: bool) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    git(root, &["init", "-q", "-b", "main"]);
    git(root, &["config", "user.name", "Fixture"]);
    git(root, &["config", "user.email", "fixture@example.invalid"]);
    write(root, "README.md", "Fixture\n");
    if legacy {
        write(
            root,
            "product-context.yaml",
            &json!({"stories":[{"id":"one","status":"pending"},{"id":"two","status":"pending"}]})
                .to_string(),
        );
        write(
            root,
            "AGENTS.md",
            "<!-- aep-agents-template: v4.1.0 -->\nProtect customer data. Use /aep-build.\n",
        );
        write(
            root,
            "project-convention/README.md",
            "Read [security](project-convention/security.md).\n",
        );
        write(
            root,
            "project-convention/security.md",
            "Protect customer data.\n",
        );
        write(
            root,
            "lessons-learned/notes.md",
            "See [rules](../project-convention/security.md).\n",
        );
        write(
            root,
            ".agents/skills/aep-build/SKILL.md",
            "Installed legacy skill\n",
        );
    }
    commit(root);
    dir
}
fn apply(root: &Path, plan: &Value, code: i32) {
    let file = tempfile::NamedTempFile::new().unwrap();
    fs::write(file.path(), plan.to_string()).unwrap();
    call(
        root,
        &["migrate", "apply", "--plan", file.path().to_str().unwrap()],
        code,
    );
}
#[test]
fn init_preserves_legacy_default_and_migration_selects_v5_without_changing_sources() {
    let d = repo(true);
    let root = d.path();
    let paths = [
        "product-context.yaml",
        "project-convention/README.md",
        "project-convention/security.md",
        "lessons-learned/notes.md",
        ".agents/skills/aep-build/SKILL.md",
    ];
    let original: Vec<_> = paths
        .iter()
        .map(|p| fs::read(root.join(p)).unwrap())
        .collect();
    call(root, &["skills"], 0);
    assert!(!root.join(".aep/config.toml").exists());
    call(root, &["init"], 0);
    let entry = fs::read(root.join("AGENTS.md")).unwrap();
    assert!(String::from_utf8_lossy(&entry).contains("AEP default: v4"));
    call(root, &["init"], 0);
    assert_eq!(entry, fs::read(root.join("AGENTS.md")).unwrap());
    commit(root);
    let plan = call(root, &["migrate", "plan"], 0)["plan"].clone();
    assert_eq!(plan["diagnostics"], json!([]));
    apply(root, &plan, 0);
    apply(root, &plan, 0);
    let migrated = fs::read_to_string(root.join("AGENTS.md")).unwrap();
    assert!(migrated.contains("AEP default: v5"));
    assert!(migrated.contains("Protect customer data"));
    assert_eq!(
        entry,
        fs::read(root.join("project-rules/legacy-entrypoint.md")).unwrap()
    );
    for (path, bytes) in paths.iter().zip(original) {
        assert_eq!(bytes, fs::read(root.join(path)).unwrap(), "{path}");
    }
    call(root, &["migrate", "verify"], 0);
    assert_eq!(
        call(root, &["migrate", "plan"], 0)["already_migrated"],
        true
    );
    write(root, "project-convention/security.md", "Source changed\n");
    call(root, &["migrate", "plan"], 5);
    call(root, &["migrate", "verify"], 5);
}
#[test]
fn fresh_init_uses_v5_and_custom_rules_survive_cutover() {
    let d = repo(false);
    call(d.path(), &["init"], 0);
    assert!(
        fs::read_to_string(d.path().join("AGENTS.md"))
            .unwrap()
            .contains("AEP default: v5")
    );
    let d = repo(true);
    let root = d.path();
    let mut config = aep_core::Config::default();
    config.stores.rules = "native-rules".into();
    write(
        root,
        ".aep/config.toml",
        &toml::to_string_pretty(&config).unwrap(),
    );
    commit(root);
    let plan = call(root, &["migrate", "plan"], 0)["plan"].clone();
    apply(root, &plan, 0);
    assert!(
        fs::read_to_string(root.join("AGENTS.md"))
            .unwrap()
            .contains("native-rules/README.md")
    );
    assert!(root.join("native-rules/security.md").exists());
    call(root, &["migrate", "verify"], 0);
}
#[test]
fn migration_blocks_source_target_overlap_and_existing_product_collision() {
    let d = repo(true);
    let root = d.path();
    let mut config = aep_core::Config::default();
    config.stores.rules = "project-convention".into();
    write(
        root,
        ".aep/config.toml",
        &toml::to_string_pretty(&config).unwrap(),
    );
    call(root, &["migrate", "plan"], 5);
    assert!(root.join("project-convention/security.md").exists());
    fs::remove_file(root.join(".aep/config.toml")).unwrap();
    write(root, "product/vision.md", "Legacy vision\n");
    write(root, "project-roadmap/product/vision.md", "Native vision\n");
    commit(root);
    call(root, &["migrate", "plan"], 5);
    assert_eq!(
        fs::read_to_string(root.join("project-roadmap/product/vision.md")).unwrap(),
        "Native vision\n"
    );
}
#[test]
fn later_scope_preserves_edited_native_copies_and_linked_rules() {
    let d = repo(true);
    let root = d.path();
    write(root, "product/vision.md", "Legacy vision\n");
    write(root, "product/retired.md", "Retired native copy\n");
    commit(root);
    let first = call(root, &["migrate", "plan", "--story", "one"], 0)["plan"].clone();
    apply(root, &first, 0);
    write(root, "project-roadmap/product/vision.md", "Native edits\n");
    fs::remove_file(root.join("project-roadmap/product/retired.md")).unwrap();
    write(root, "project-rules/security.md", "Native security edits\n");
    let second = call(root, &["migrate", "plan", "--story", "two"], 0)["plan"].clone();
    assert_eq!(second["diagnostics"], json!([]));
    apply(root, &second, 0);
    assert!(!root.join("project-roadmap/product/retired.md").exists());
    assert_eq!(
        fs::read_to_string(root.join("project-roadmap/product/vision.md")).unwrap(),
        "Native edits\n"
    );
    assert_eq!(
        fs::read_to_string(root.join("project-rules/security.md")).unwrap(),
        "Native security edits\n"
    );
}
#[test]
fn host_consumer_review_is_explicit_and_preserves_host_files() {
    let d = repo(true);
    let root = d.path();
    let settings =
        r#"{"hooks":{"Stop":[{"hooks":[{"type":"command","command":"legacy-autopilot tick"}]}]}}"#;
    write(root, ".claude/settings.json", settings);
    write(root, "CLAUDE.md", "Use /aep-autopilot for scheduling.\n");
    commit(root);
    let blocked = call(root, &["migrate", "plan"], 0)["plan"].clone();
    assert!(!blocked["diagnostics"].as_array().unwrap().is_empty());
    apply(root, &blocked, 3);
    write(
        root,
        "consumer-review.md",
        "The hook adapter checks the selected version and remains idle for v5. CLAUDE legacy workflow applies only under the active AGENTS version route. Agent inspected host scheduling.\n",
    );
    commit(root);
    let plan = call(
        root,
        &["migrate", "plan", "--consumer-review", "consumer-review.md"],
        0,
    )["plan"]
        .clone();
    assert_eq!(plan["diagnostics"], json!([]));
    apply(root, &plan, 0);
    assert_eq!(
        fs::read_to_string(root.join(".claude/settings.json")).unwrap(),
        settings
    );
    assert_eq!(
        fs::read_to_string(root.join("CLAUDE.md")).unwrap(),
        "Use /aep-autopilot for scheduling.\n"
    );
    call(root, &["migrate", "verify"], 0);
}
#[test]
fn published_v4_read_only_guards_need_no_consumer_mapping() {
    let d = repo(true);
    let root = d.path();
    let settings =
        include_str!("../../../skills/project-setup/onboard/references/settings-template.json");
    write(root, ".claude/settings.json", settings);
    commit(root);
    let plan = call(root, &["migrate", "plan"], 0)["plan"].clone();
    assert_eq!(plan["diagnostics"], json!([]));
    apply(root, &plan, 0);
    assert_eq!(
        fs::read_to_string(root.join(".claude/settings.json")).unwrap(),
        settings
    );
}
#[test]
fn source_and_target_drift_after_plan_are_rejected() {
    let d = repo(true);
    let root = d.path();
    let plan = call(root, &["migrate", "plan"], 0)["plan"].clone();
    write(root, "product-context.yaml", "stories: []\n");
    apply(root, &plan, 5);
    let d = repo(true);
    let root = d.path();
    let plan = call(root, &["migrate", "plan"], 0)["plan"].clone();
    write(
        root,
        "project-rules/security.md",
        "Concurrent native edits\n",
    );
    apply(root, &plan, 5);
}

#[test]
fn custom_source_overlap_and_product_relative_links_are_explicit() {
    let d = repo(true);
    let root = d.path();
    let mut config = aep_core::Config::default();
    config.stores.rules = "context".into();
    write(
        root,
        ".aep/config.toml",
        &toml::to_string_pretty(&config).unwrap(),
    );
    write(root, "context/legacy.yaml", "stories: []\n");
    commit(root);
    call(
        root,
        &["migrate", "plan", "--source", "context/legacy.yaml"],
        5,
    );
    fs::remove_file(root.join(".aep/config.toml")).unwrap();
    write(root, "product/vision.md", "See [project](../README.md).\n");
    commit(root);
    let plan = call(root, &["migrate", "plan"], 0)["plan"].clone();
    assert!(
        plan["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v.as_str().unwrap().contains("directory depth"))
    );
    apply(root, &plan, 3);
}

#[test]
fn ignored_local_host_settings_stay_local_and_digest_bound() {
    let d = repo(true);
    let root = d.path();
    write(root, ".gitignore", ".claude/settings.local.json\n");
    commit(root);
    let settings =
        include_str!("../../../skills/project-setup/onboard/references/settings-template.json");
    write(root, ".claude/settings.local.json", settings);
    let plan = call(root, &["migrate", "plan"], 0)["plan"].clone();
    assert_eq!(plan["diagnostics"], json!([]));
    apply(root, &plan, 0);
    call(root, &["migrate", "verify"], 0);
    assert_eq!(
        fs::read_to_string(root.join(".claude/settings.local.json")).unwrap(),
        settings
    );
    write(root, ".claude/settings.local.json", "{}");
    call(root, &["migrate", "verify"], 5);
}
#[test]
fn nested_rule_links_and_missing_claude_discovery_require_mapping() {
    let d = repo(true);
    let root = d.path();
    let mut config = aep_core::Config::default();
    config.stores.rules = "native/rules".into();
    write(
        root,
        ".aep/config.toml",
        &toml::to_string_pretty(&config).unwrap(),
    );
    write(
        root,
        "project-convention/security.md",
        "Read [root](../README.md).\n",
    );
    write(root, "CLAUDE.md", "Keep project tests meaningful.\n");
    commit(root);
    let plan = call(root, &["migrate", "plan"], 0)["plan"].clone();
    let diagnostics = plan["diagnostics"].as_array().unwrap();
    assert!(diagnostics.iter().any(|v| {
        v.as_str()
            .unwrap()
            .contains("project-convention/security.md")
    }));
    assert!(diagnostics.iter().any(|v| {
        v.as_str()
            .unwrap()
            .contains("instruction discovery CLAUDE.md")
    }));
    apply(root, &plan, 3);
}

#[test]
fn overlapping_story_and_change_ids_keep_stable_provenance_across_scopes() {
    let d = repo(true);
    let root = d.path();
    write(
        root,
        "product-context.yaml",
        &json!({"stories":[
            {"id":"one","status":"pending","change_id":"one"},
            {"id":"two","status":"pending","openspec_change":"two"}
        ]})
        .to_string(),
    );
    for id in ["one", "two"] {
        write(
            root,
            &format!("openspec/changes/{id}/proposal.md"),
            "Documented change\n",
        );
        write(
            root,
            &format!("openspec/changes/{id}/design.md"),
            "Preserve source intent\n",
        );
    }
    commit(root);
    let first = call(root, &["migrate", "plan", "--story", "one"], 0)["plan"].clone();
    assert_eq!(first["diagnostics"], json!([]));
    let records = first["records"].as_array().unwrap();
    let story = records.iter().find(|r| r["id"] == "one").unwrap();
    assert_eq!(story["change"], "openspec-change-one");
    let change = records
        .iter()
        .find(|r| r["id"] == "openspec-change-one")
        .unwrap();
    assert_eq!(change["data"]["source"]["change"], "one");
    assert!(
        first["writes"]
            .get("project-ledger/changes/openspec-change-one/imported/design.md")
            .is_some()
    );
    apply(root, &first, 0);
    call(root, &["migrate", "verify"], 0);
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
    let second = call(root, &["migrate", "plan", "--story", "two"], 0)["plan"].clone();
    assert_eq!(second["diagnostics"], json!([]));
    let story = second["records"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["id"] == "two")
        .unwrap();
    assert_eq!(story["change"], "openspec-change-two");
    apply(root, &second, 0);
    call(root, &["migrate", "verify"], 0);
}

#[test]
fn migration_preserves_non_dispatchable_legacy_states() {
    let d = repo(true);
    let root = d.path();
    let states = [
        "blocked",
        "deferred",
        "cancelled",
        "archived",
        "awaiting_owner",
    ];
    write(root, "product-context.yaml", &json!({"stories":states.iter().map(|status| json!({"id":status,"status":status})).collect::<Vec<_>>()} ).to_string());
    commit(root);
    let mut args = vec!["migrate", "plan"];
    for status in &states {
        args.extend(["--story", status]);
    }
    let plan = call(root, &args, 0)["plan"].clone();
    assert_eq!(plan["diagnostics"], json!([]));
    for status in states {
        let record = plan["records"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["id"] == status)
            .unwrap();
        assert_eq!(record["status"], status);
    }
    apply(root, &plan, 0);
    call(root, &["migrate", "verify"], 0);
}

#[test]
fn legacy_change_link_ignores_same_source_id_from_another_bundle() {
    let d = repo(true);
    let root = d.path();
    write(
        root,
        "product-context.yaml",
        &json!({"stories":[{"id":"one","status":"pending","change_id":"one"}]}).to_string(),
    );
    write(
        root,
        "openspec/changes/one/proposal.md",
        "This repository's change\n",
    );
    call(root, &["init"], 0);
    let unrelated = tempfile::NamedTempFile::new().unwrap();
    fs::write(
        unrelated.path(),
        json!({
            "kind":"change", "id":"foreign-one", "title":"Unrelated imported change",
            "data":{"source":{"path":root.join("other-openspec"),"change":"one"}}
        })
        .to_string(),
    )
    .unwrap();
    call(
        root,
        &[
            "change",
            "new",
            "--file",
            unrelated.path().to_str().unwrap(),
        ],
        0,
    );
    commit(root);

    let plan = call(root, &["migrate", "plan"], 0)["plan"].clone();
    assert_eq!(plan["diagnostics"], json!([]));
    let story = plan["records"]
        .as_array()
        .unwrap()
        .iter()
        .find(|record| record["id"] == "one")
        .unwrap();
    assert_eq!(story["change"], "openspec-change-one");
    let imported = plan["records"]
        .as_array()
        .unwrap()
        .iter()
        .find(|record| record["id"] == "openspec-change-one")
        .unwrap();
    assert_eq!(
        imported["data"]["source"]["path"],
        json!(fs::canonicalize(root.join("openspec")).unwrap())
    );
    assert_eq!(imported["data"]["source"]["change"], "one");
    apply(root, &plan, 0);
    call(root, &["migrate", "verify"], 0);
    assert_eq!(
        call(root, &["story", "show", "one"], 0)["record"]["change"],
        "openspec-change-one"
    );
}

#[test]
fn reviewed_migration_rejects_invalid_prospective_config_before_writes() {
    let d = repo(true);
    let root = d.path();
    let plan = call(root, &["migrate", "plan"], 0)["plan"].clone();
    let before = fs::read(root.join("AGENTS.md")).unwrap();
    for config in [
        Some("unknown_field = true\n"),
        None,
        Some("[stores]\nrules = 'changed-rules'\n"),
    ] {
        let mut invalid = plan.clone();
        invalid["writes"][".aep/config.toml"] = json!(config);
        apply(root, &invalid, 2);
        assert!(!root.join(".aep/config.toml").exists());
        assert!(!root.join("project-ledger").exists());
        assert_eq!(fs::read(root.join("AGENTS.md")).unwrap(), before);
    }
    let mut missing = plan.clone();
    missing["writes"]
        .as_object_mut()
        .unwrap()
        .remove(".aep/config.toml");
    apply(root, &missing, 2);
    assert!(!root.join(".aep/config.toml").exists());
    assert!(!root.join("project-ledger").exists());
    let mut plan = plan;
    plan["writes"][".aep/config.toml"] = json!("[[checks]]\nid='smoke'\ncommand=['true']\n");
    plan["records"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|r| r["id"] == "one")
        .unwrap()["required_checks"] = json!(["smoke"]);
    apply(root, &plan, 0);
    call(root, &["migrate", "verify"], 0);
}
