use std::{path::Path, process::Command};

fn aep(root: &Path, args: &[&str]) -> (i32, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_aep"))
        .arg("--root")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    (
        out.status.code().unwrap(),
        String::from_utf8_lossy(&out.stdout).into(),
    )
}

#[test]
fn the_listing_marks_bug_fix_as_a_runbook() {
    let d = tempfile::tempdir().unwrap();
    let (code, out) = aep(d.path(), &["--skill"]);
    assert_eq!(code, 0);
    assert!(out.contains("- bug-fix (runbook): "), "{out}");
    assert!(
        out.contains("- design: "),
        "procedures keep their label: {out}"
    );
}

#[test]
fn bug_fix_prints_its_steps_in_order_with_their_principles() {
    let d = tempfile::tempdir().unwrap();
    let (code, out) = aep(d.path(), &["--skill", "bug-fix"]);
    assert_eq!(code, 0);
    let order = [
        "reproduce",
        "root-cause",
        "test-first",
        "fix",
        "verify",
        "learn",
    ];
    let mut at = 0;
    for id in order {
        let found = out[at..]
            .find(&format!("id: {id}"))
            .unwrap_or_else(|| panic!("{id} missing: {out}"));
        at += found + 1;
    }
    assert!(
        out.contains("prove-it") && out.contains("encode-lessons"),
        "{out}"
    );
}
