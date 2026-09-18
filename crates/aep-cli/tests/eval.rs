use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

fn git(root: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap().trim().into()
}
fn aep(root: &Path, home: &Path, args: &[&str], code: i32) -> Value {
    let out = Command::new(env!("CARGO_BIN_EXE_aep"))
        .env("AEP_HOME", home)
        .env_remove("HERDR_ENV")
        .env_remove("HERDR_PANE_ID")
        .arg("--json")
        .arg("--root")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    let result: Value =
        serde_json::from_slice(&out.stdout).unwrap_or_else(|_| panic!("invalid JSON: {out:?}"));
    assert_eq!(out.status.code(), Some(code), "{args:?}: {result}");
    result["data"].clone()
}
fn setup() -> (tempfile::TempDir, tempfile::TempDir) {
    let project = tempfile::tempdir().unwrap();
    let home = tempfile::tempdir().unwrap();
    let root = project.path();
    git(root, &["init", "-q", "-b", "main"]);
    git(root, &["config", "user.email", "fixture@example.invalid"]);
    git(root, &["config", "user.name", "Fixture"]);
    fs::write(root.join("README.md"), "Fixture\n").unwrap();
    git(root, &["add", "."]);
    git(root, &["commit", "-qm", "base"]);
    git(
        root,
        &[
            "remote",
            "add",
            "origin",
            "git@github.com:Example/Widget.git",
        ],
    );
    aep(root, home.path(), &["init"], 0);
    git(root, &["add", "."]);
    git(root, &["commit", "-qm", "init"]);
    (project, home)
}
fn tree_digest(root: &Path) -> String {
    git(root, &["add", "-A", "--dry-run"]) + &git(root, &["status", "--porcelain"])
}

#[test]
fn eval_writes_only_under_the_aep_home_and_reports_structural_findings() {
    let (project, home) = setup();
    let root = project.path();
    let before = tree_digest(root);

    let init = aep(root, home.path(), &["eval", "init"], 0);
    assert_eq!(init["created"], true);
    assert!(home.path().join("config.toml").exists());

    let snap = aep(root, home.path(), &["eval", "snapshot"], 0);
    let run = snap["run"].as_str().unwrap().to_string();
    let dir = Path::new(snap["dir"].as_str().unwrap()).to_path_buf();
    assert!(dir.starts_with(home.path().join("eval").join("github.com/example/widget")));
    assert!(dir.join("snapshot.json").exists());
    assert!(dir.join("manifest.json").exists());
    assert!(dir.join("SHA256SUMS").exists());
    assert_eq!(snap["facts"]["records"]["check"]["pass"], true);
    assert_eq!(snap["facts"]["verification"]["configured_checks"], 0);
    assert_eq!(snap["facts"]["legacy"]["agents_md_has_entrypoint"], true);

    let report = aep(root, home.path(), &["eval", "report"], 0);
    let ids: Vec<&str> = report["report"]["findings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["id"].as_str().unwrap())
        .collect();
    assert!(ids.contains(&"VER-001"), "{ids:?}");
    assert!(ids.contains(&"OPS-001"), "{ids:?}");
    assert!(!ids.contains(&"GIT-002"), "clean tree: {ids:?}");
    assert!(!ids.contains(&"LEG-003"), "{ids:?}");
    assert!(dir.join("report.md").exists());

    // A dirty tree is reported as info on the next snapshot of the same run.
    fs::write(root.join("scratch.txt"), "x\n").unwrap();
    aep(root, home.path(), &["eval", "snapshot", "--run", &run], 0);
    let report = aep(root, home.path(), &["eval", "report", "--run", &run], 0);
    assert!(
        report["report"]["findings"]
            .as_array()
            .unwrap()
            .iter()
            .any(|f| f["id"] == "GIT-002")
    );
    fs::remove_file(root.join("scratch.txt")).unwrap();

    // Observations are stored with the note and indexed.
    let obs = tempfile::NamedTempFile::new().unwrap();
    fs::write(
        obs.path(),
        json!({"claim":"pushed","source":"transcript","matches":false}).to_string(),
    )
    .unwrap();
    let rec = aep(
        root,
        home.path(),
        &[
            "eval",
            "record",
            "--run",
            &run,
            "--file",
            obs.path().to_str().unwrap(),
            "--note",
            "claim did not match git",
        ],
        0,
    );
    assert_eq!(rec["recorded"]["note"], "claim did not match git");
    assert_eq!(rec["recorded"]["claim"], "pushed");
    aep(
        root,
        home.path(),
        &[
            "eval",
            "record",
            "--run",
            &run,
            "--file",
            obs.path().to_str().unwrap(),
        ],
        0,
    );
    let show = aep(root, home.path(), &["eval", "show", &run], 0);
    assert_eq!(show["observations"].as_array().unwrap().len(), 2);
    let list = aep(root, home.path(), &["eval", "list"], 0);
    assert_eq!(list["runs"].as_array().unwrap().len(), 1);
    assert_eq!(list["runs"][0]["observations"], 2);

    // Nothing was written into the project.
    assert_eq!(before, tree_digest(root));
    assert!(!root.join(".aep").join("eval").exists());
}

#[test]
fn eval_watch_needs_herdr_and_prepares_a_run_for_the_calling_agent() {
    let (project, home) = setup();
    let root = project.path();
    let out = Command::new(env!("CARGO_BIN_EXE_aep"))
        .env("AEP_HOME", home.path())
        .env_remove("HERDR_ENV")
        .args(["--json", "--root"])
        .arg(root)
        .args(["eval", "watch"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(3), "{out:?}");

    // A fake herdr on PATH answers agent list/get so the flow can be exercised.
    let bin = tempfile::tempdir().unwrap();
    let fake = bin.path().join("herdr");
    fs::write(
        &fake,
        format!(
            "#!/bin/sh\ncase \"$1 $2\" in\n  \"agent list\") printf '%s' '{{\"result\":{{\"agents\":[{{\"pane_id\":\"w9:p1\",\"agent\":\"codex\",\"agent_status\":\"working\",\"terminal_title_stripped\":\"work\",\"cwd\":\"{root}\"}},{{\"pane_id\":\"w9:p9\",\"agent\":\"claude\",\"agent_status\":\"idle\",\"terminal_title_stripped\":\"me\",\"cwd\":\"{root}\"}},{{\"pane_id\":\"w1:p1\",\"agent\":\"codex\",\"agent_status\":\"idle\",\"terminal_title_stripped\":\"other\",\"cwd\":\"/elsewhere\"}}]}}}}' ;;\n  \"agent get\") printf '%s' '{{\"result\":{{\"agent\":\"codex\",\"agent_status\":\"working\",\"pane_id\":\"w9:p1\",\"terminal_title_stripped\":\"work\"}}}}' ;;\n  *) echo unexpected >&2; exit 1 ;;\nesac\n",
            root = root.canonicalize().unwrap().display()
        ),
    )
    .unwrap();
    let mut perms = fs::metadata(&fake).unwrap().permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut perms, 0o755);
    fs::set_permissions(&fake, perms).unwrap();
    let path = format!(
        "{}:{}",
        bin.path().display(),
        std::env::var("PATH").unwrap()
    );

    let out = Command::new(env!("CARGO_BIN_EXE_aep"))
        .env("AEP_HOME", home.path())
        .env("HERDR_ENV", "1")
        .env("HERDR_PANE_ID", "w9:p9")
        .env("PATH", &path)
        .args(["--json", "--root"])
        .arg(root)
        .args(["eval", "watch"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0), "{out:?}");
    let result: Value = serde_json::from_slice(&out.stdout).unwrap();
    let candidates = result["data"]["candidates"].as_array().unwrap();
    assert_eq!(
        candidates.len(),
        1,
        "own pane and other projects excluded: {candidates:?}"
    );
    assert_eq!(candidates[0]["pane_id"], "w9:p1");

    let out = Command::new(env!("CARGO_BIN_EXE_aep"))
        .env("AEP_HOME", home.path())
        .env("HERDR_ENV", "1")
        .env("HERDR_PANE_ID", "w9:p9")
        .env("PATH", &path)
        .args(["--json", "--root"])
        .arg(root)
        .args(["eval", "watch", "--target", "w9:p1"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0), "{out:?}");
    let result: Value = serde_json::from_slice(&out.stdout).unwrap();
    let data = &result["data"];
    assert_eq!(data["target_pane"], "w9:p1");
    assert_eq!(data["observer_pane"], "w9:p9");
    assert_eq!(data["target_agent"]["kind"], "codex");
    let dir = Path::new(data["dir"].as_str().unwrap());
    let procedure = fs::read_to_string(dir.join("procedure.md")).unwrap();
    assert!(procedure.contains("eval run id"));
    assert!(procedure.contains("aep --skill eval --ref observer"));
    assert!(dir.join("snapshot.json").exists());
    let manifest: Value =
        serde_json::from_str(&fs::read_to_string(dir.join("manifest.json")).unwrap()).unwrap();
    assert_eq!(manifest["mode"], "watch");
    assert_eq!(manifest["observer"]["pane"], "w9:p9");
    assert_eq!(git(root, &["status", "--porcelain"]), "");

    let out = Command::new(env!("CARGO_BIN_EXE_aep"))
        .env("AEP_HOME", home.path())
        .env("HERDR_ENV", "1")
        .env("HERDR_PANE_ID", "w9:p9")
        .env("PATH", &path)
        .args(["--json", "--root"])
        .arg(root)
        .args(["eval", "watch", "--target", "w9:p9"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(2), "own pane rejected: {out:?}");
}

#[test]
fn eval_skill_is_bundled_with_its_references() {
    let (project, home) = setup();
    let root = project.path();
    let catalog = aep(root, home.path(), &["--skill"], 0);
    let names: Vec<&str> = catalog["skills"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|s| s["name"].as_str())
        .collect();
    assert!(names.contains(&"eval"), "{names:?}");
    let body = aep(root, home.path(), &["--skill", "eval", "--ref", "rules"], 0);
    assert!(body.to_string().contains("VER-001"));
}
