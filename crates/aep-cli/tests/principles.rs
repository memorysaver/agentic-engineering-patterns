use serde_json::Value;
use std::{fs, path::Path, process::Command};

fn git(root: &Path, args: &[&str]) {
    let out = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(out.status.success(), "git {args:?}");
}
/// Runs the CLI with `args` in `root`; returns the exit code, stdout and stderr.
fn aep(root: &Path, args: &[&str]) -> (i32, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_aep"))
        .arg("--root")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    (
        out.status.code().unwrap(),
        String::from_utf8_lossy(&out.stdout).into(),
        String::from_utf8_lossy(&out.stderr).into(),
    )
}
fn setup() -> tempfile::TempDir {
    let d = tempfile::tempdir().unwrap();
    let root = d.path();
    git(root, &["init", "-q", "-b", "main"]);
    git(root, &["config", "user.email", "fixture@example.invalid"]);
    git(root, &["config", "user.name", "Fixture"]);
    fs::write(root.join("README.md"), "Fixture\n").unwrap();
    git(root, &["add", "."]);
    git(root, &["commit", "-qm", "base"]);
    d
}
fn read_log(root: &Path) -> Vec<Value> {
    match fs::read_to_string(root.join(".git/aep/guidance-reads.jsonl")) {
        Ok(text) => text
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect(),
        Err(_) => vec![],
    }
}

#[test]
fn the_catalog_lists_the_four_admitted_principles_only() {
    let d = setup();
    let (code, out, _) = aep(d.path(), &["--skill", "principles"]);
    assert_eq!(code, 0);
    for id in [
        "prove-it",
        "encode-lessons",
        "verifiable-units",
        "respond-in-their-terms",
    ] {
        assert!(out.contains(id), "missing {id}: {out}");
    }
    for deferred in [
        "attack-the-premise",
        "prototype-the-fork",
        "separate-before-serializing-shared-state",
    ] {
        assert!(!out.contains(deferred), "served {deferred}: {out}");
    }
}

#[test]
fn an_admitted_principle_prints_its_trigger_rule_enforcement_and_occurrences() {
    let d = setup();
    let (code, out, _) = aep(d.path(), &["--skill", "principles", "--ref", "prove-it"]);
    assert_eq!(code, 0);
    for part in ["Trigger", "Rule", "Enforcement", "Evidence basis"] {
        assert!(out.contains(part), "missing {part}: {out}");
    }
}

#[test]
fn a_read_of_a_principle_with_an_attempt_records_the_attempt() {
    let d = setup();
    let root = d.path();
    let (code, _, _) = aep(
        root,
        &[
            "--skill",
            "principles",
            "--ref",
            "encode-lessons",
            "--attempt",
            "attempt-18e3ade60f011e4d-e90a0-0",
        ],
    );
    assert_eq!(code, 0);
    let last = read_log(root).pop().unwrap();
    assert_eq!(last["procedure"], "principles");
    assert_eq!(last["reference"], "encode-lessons");
    assert_eq!(last["attempt"], "attempt-18e3ade60f011e4d-e90a0-0");
}

#[test]
fn a_deferred_principle_is_refused_naming_the_admitted_ids_and_logs_nothing() {
    let d = setup();
    let root = d.path();
    let (code, _, err) = aep(
        root,
        &["--skill", "principles", "--ref", "attack-the-premise"],
    );
    assert_eq!(code, 2);
    for id in [
        "prove-it",
        "encode-lessons",
        "verifiable-units",
        "respond-in-their-terms",
    ] {
        assert!(err.contains(id), "refusal does not name {id}: {err}");
    }
    assert!(read_log(root).is_empty());
}

#[test]
fn ordinary_dry_run_and_refused_reads_leave_no_receipts() {
    let d = setup();
    let root = d.path();
    assert_eq!(aep(root, &["--skill", "principles"]).0, 0);
    assert_eq!(
        aep(
            root,
            &[
                "--dry-run",
                "--skill",
                "principles",
                "--attempt",
                "attempt-test"
            ]
        )
        .0,
        0
    );
    assert_eq!(
        aep(
            root,
            &[
                "--skill",
                "principles",
                "--ref",
                "missing",
                "--attempt",
                "attempt-test"
            ]
        )
        .0,
        2
    );
    assert!(read_log(root).is_empty());
}
