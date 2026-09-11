use serde_json::{Value, json};
use std::{
    fs,
    path::Path,
    process::{Command, Output},
};
fn run(root: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_aep"))
        .args(args)
        .current_dir(root)
        .env("NO_COLOR", "1")
        .output()
        .unwrap()
}
fn data(root: &Path, args: &[&str]) -> Value {
    let mut a = vec!["--json"];
    a.extend_from_slice(args);
    let r = run(root, &a);
    assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stdout));
    serde_json::from_slice::<Value>(&r.stdout).unwrap()["data"].clone()
}
fn repo() -> tempfile::TempDir {
    let d = tempfile::tempdir().unwrap();
    assert!(
        Command::new("git")
            .args(["init", "-q"])
            .current_dir(d.path())
            .status()
            .unwrap()
            .success()
    );
    data(d.path(), &["init"]);
    d
}
#[test]
fn skill_is_valid_markdown_offline_and_has_no_transport_footer() {
    let d = tempfile::tempdir().unwrap();
    let r = Command::new(env!("CARGO_BIN_EXE_aep"))
        .args(["--skill"])
        .current_dir(d.path())
        .env("PATH", "")
        .output()
        .unwrap();
    assert!(r.status.success());
    let s = String::from_utf8(r.stdout).unwrap();
    let (front, body) = aep_store::frontmatter(&s).unwrap();
    let meta: Value = aep_store::parse_yaml(front).unwrap();
    assert_eq!(meta["name"], "aep");
    assert!(!meta["description"].as_str().unwrap().is_empty());
    assert!(body.contains("aep --skill design"));
    let catalog = data(d.path(), &["--skill"]);
    assert_eq!(catalog["content"], s);
    for entry in catalog["skills"].as_array().unwrap() {
        let name = entry["name"].as_str().unwrap();
        let r = run(d.path(), &["--skill", name]);
        assert!(r.status.success());
        let body = String::from_utf8(r.stdout).unwrap();
        let detail = data(d.path(), &["--skill", name]);
        assert_eq!(body, detail["content"]); // byte-exact SKILL.md, no suffix
        aep_store::frontmatter(&body).unwrap();
        for reference in detail["references"].as_array().unwrap() {
            let id = reference["name"].as_str().unwrap();
            let r = run(d.path(), &["--skill", name, "--ref", id]);
            assert!(r.status.success());
            assert_eq!(
                String::from_utf8(r.stdout).unwrap(),
                data(d.path(), &["--skill", name, "--ref", id])["content"]
            );
            let path = reference["path"].as_str().unwrap();
            let linked = run(d.path(), &["--skill", name, "--ref", path]);
            assert!(linked.status.success());
            assert_eq!(
                linked.stdout,
                run(d.path(), &["--skill", name, "--ref", id]).stdout
            );
        }
    }
}
#[test]
fn reference_links_are_catalogued_resources_not_arbitrary_paths() {
    let d = repo();
    let p = d.path().join("project-rules/skills/check-product");
    fs::create_dir_all(p.join("references")).unwrap();
    fs::write(p.join("SKILL.md"), "---\nname: check-product\ndescription: Verify product behavior.\n---\n\nRead [Checks](references/checks.md).\n").unwrap();
    fs::write(
        p.join("references/checks.md"),
        "# Checks\n\nRun the product.\n",
    )
    .unwrap();
    fs::write(d.path().join("private.md"), "outside-catalog-sentinel").unwrap();
    let short = run(d.path(), &["--skill", "check-product", "--ref", "checks"]);
    let link = run(
        d.path(),
        &["--skill", "check-product", "--ref", "references/checks.md"],
    );
    assert!(short.status.success() && link.status.success());
    assert_eq!(short.stdout, link.stdout);
    for path in [
        "../../private.md",
        "references/../SKILL.md",
        "/private.md",
        "references/missing.md",
    ] {
        let r = run(d.path(), &["--skill", "check-product", "--ref", path]);
        assert_eq!(r.status.code(), Some(2));
        let error = String::from_utf8(r.stderr).unwrap();
        assert!(error.contains("aep --skill check-product --ref checks"));
        assert!(!error.contains("outside-catalog-sentinel"));
    }
}
#[test]
fn skill_discovery_includes_local_procedures_and_reserved_names_fail() {
    let d = repo();
    let p = d.path().join("project-rules/skills/check-product");
    fs::create_dir_all(&p).unwrap();
    fs::write(p.join("SKILL.md"), "---\nname: check-product\ndescription: Exercise the actual product.\n---\n\n# Check product\n").unwrap();
    let r = run(d.path(), &["--skill"]);
    assert!(r.status.success());
    let catalog = String::from_utf8(r.stdout).unwrap();
    let (builtin, project) = catalog.split_once("### Project procedures").unwrap();
    assert!(builtin.contains("### Built-in procedures"));
    assert!(builtin.contains("aep --skill validate"));
    assert!(!builtin.contains("aep --skill check-product"));
    assert!(project.contains("aep --skill check-product"));
    assert!(project.contains("Source: `project-rules/skills/check-product/SKILL.md`"));
    let body = fs::read(p.join("SKILL.md")).unwrap();
    assert_eq!(run(d.path(), &["--skill", "check-product"]).stdout, body);
    let outside = tempfile::tempdir().unwrap();
    let outside_catalog = run(outside.path(), &["--skill"]);
    assert!(!String::from_utf8_lossy(&outside_catalog.stdout).contains("check-product"));
    let local = data(d.path(), &["--skill"]);
    let local = local["skills"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["name"] == "check-product")
        .unwrap();
    assert_eq!(
        local["source"],
        "project:project-rules/skills/check-product/SKILL.md"
    );
    fs::write(
        p.join("SKILL.md"),
        "---\nname: aep\ndescription: Shadow the entrypoint.\n---\n\nBad.\n",
    )
    .unwrap();
    assert_eq!(run(d.path(), &["--skill"]).status.code(), Some(2));
}
#[test]
fn help_and_skill_cannot_accidentally_execute_an_operation() {
    let d = tempfile::tempdir().unwrap();
    for args in [&[][..], &["--help"], &["verify"], &["migrate"], &["story"]] {
        let r = run(d.path(), args);
        assert!(
            r.status.success(),
            "{:?}: {}",
            args,
            String::from_utf8_lossy(&r.stderr)
        );
        assert!(String::from_utf8_lossy(&r.stdout).contains("Usage:"));
    }
    for args in [
        &["--skill", "init"][..],
        &["--skill=aep", "init"],
        &["--ref", "bdd"],
        &["skills"],
        &["--skills"],
        &["story", "new"],
    ] {
        let r = run(d.path(), args);
        assert_eq!(r.status.code(), Some(2), "{args:?}");
    }
    assert_eq!(fs::read_dir(d.path()).unwrap().count(), 0);
    let result: Value =
        serde_json::from_slice(&run(d.path(), &["--json", "--help"]).stdout).unwrap();
    assert_eq!(result["ok"], true);
}
#[test]
fn human_status_is_bounded_while_json_retains_every_story_and_readiness_reason() {
    let d = repo();
    let p = d.path().join("project-ledger/stories");
    fs::create_dir_all(&p).unwrap();
    for i in 0..45 {
        let r = aep_core::Record::new(
            aep_core::Kind::Story,
            &format!("S-{i:02}"),
            "Unicode context 中文",
        );
        fs::write(
            p.join(format!("{}.yaml", r.id)),
            serde_yaml_ng::to_string(&r).unwrap(),
        )
        .unwrap();
    }
    let before = aep_store::Store::open(d.path())
        .unwrap()
        .snapshot()
        .unwrap()
        .revision;
    let r = run(d.path(), &["status"]);
    assert!(r.status.success());
    let human = String::from_utf8(r.stdout).unwrap();
    assert!(human.contains("45 stories"));
    assert!(human.lines().count() < 70);
    assert!(human.contains("--json"));
    assert!(serde_json::from_str::<Value>(&human).is_err());
    let structured = data(d.path(), &["status"]);
    assert_eq!(structured["stories"].as_array().unwrap().len(), 45);
    assert!(structured["stories"][0]["readiness"]["reasons"].is_array());
    assert_eq!(
        data(d.path(), &["query", "--kind", "story"])["records"]
            .as_array()
            .unwrap()
            .len(),
        45
    );
    assert_eq!(
        aep_store::Store::open(d.path())
            .unwrap()
            .snapshot()
            .unwrap()
            .revision,
        before
    );
}
#[test]
fn human_failures_preserve_exit_status_and_structured_diagnostics() {
    let d = repo();
    let p = d.path().join("project-ledger/stories");
    fs::create_dir_all(&p).unwrap();
    let mut r = aep_core::Record::new(aep_core::Kind::Story, "bad", "Missing dependency");
    r.depends_on = vec!["absent".into()];
    fs::write(p.join("bad.yaml"), serde_yaml_ng::to_string(&r).unwrap()).unwrap();
    let human = run(d.path(), &["check"]);
    let structured = run(d.path(), &["check", "--json"]);
    assert_eq!(human.status.code(), Some(1));
    assert_eq!(human.status.code(), structured.status.code());
    assert!(String::from_utf8_lossy(&human.stdout).contains("absent"));
    let json: Value = serde_json::from_slice(&structured.stdout).unwrap();
    assert_eq!(json["ok"], false);
    assert!(!json["diagnostics"].as_array().unwrap().is_empty());
    assert_eq!(json["side_effects"], false);
}
#[test]
fn native_writes_show_outcome_and_dry_run_remains_non_mutating() {
    let d = repo();
    let input = d.path().join("story.json");
    fs::write(
        &input,
        json!({"kind":"story","id":"S","title":"Test"}).to_string(),
    )
    .unwrap();
    let preview = run(
        d.path(),
        &["story", "new", "--file", "story.json", "--dry-run"],
    );
    assert!(preview.status.success());
    assert!(String::from_utf8_lossy(&preview.stdout).contains("nothing applied"));
    assert!(!d.path().join("project-ledger/stories/S.yaml").exists());
    let saved = run(d.path(), &["story", "new", "--file", "story.json"]);
    assert!(saved.status.success());
    assert!(String::from_utf8_lossy(&saved.stdout).contains("Saved"));
    assert_eq!(
        data(d.path(), &["story", "show", "S"])["record"]["title"],
        "Test"
    );
}
