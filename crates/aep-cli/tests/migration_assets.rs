use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};

const IMAGE: &[u8] = b"\x89PNG\r\n\x1a\n\0fixture";

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
fn write(root: &Path, path: &str, bytes: impl AsRef<[u8]>) {
    let path = root.join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, bytes).unwrap();
}
fn call(root: &Path, args: &[&str], code: i32) -> Value {
    let out = Command::new(env!("CARGO_BIN_EXE_aep"))
        .arg("--json")
        .arg("--root")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    let value: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(out.status.code(), Some(code), "{args:?}: {value}");
    value
}
fn fixture(note: &str) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    git(root, &["init", "-q", "-b", "main"]);
    git(root, &["config", "user.name", "Fixture"]);
    git(root, &["config", "user.email", "fixture@example.invalid"]);
    write(
        root,
        "product-context.yaml",
        json!({"stories":[{"id":"one","status":"pending"}]}).to_string(),
    );
    write(root, "lessons-learned/dogfood/note.md", note);
    write(root, "lessons-learned/dogfood/screens/capture.png", IMAGE);
    write(root, "product/vision.md", "![diagram](diagram.png)\n");
    write(root, "product/diagram.png", IMAGE);
    write(
        root,
        "project-convention/security.md",
        "See `screens/security.png`.\n",
    );
    write(root, "project-convention/screens/security.png", IMAGE);
    git(root, &["add", "."]);
    git(root, &["commit", "-qm", "fixture"]);
    dir
}
#[test]
fn migration_retains_binary_assets_with_provenance_rebases_links_and_checks_drift() {
    let dir = fixture("![capture](screens/capture.png)\nEvidence: `screens/capture.png`.\n");
    let root = dir.path();
    let plan = call(root, &["migrate", "plan"], 0)["data"]["plan"].clone();
    assert_eq!(plan["diagnostics"], json!([]));
    let receipt = plan["records"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["data"]["migration"] == "v4-context")
        .unwrap();
    assert_eq!(
        receipt["data"]["retained_assets"].as_array().unwrap().len(),
        3
    );
    let source = "lessons-learned/dogfood/screens/capture.png";
    assert_eq!(plan["sources"][source], aep_core::digest(IMAGE));
    assert!(
        receipt["data"]["retained_assets"]
            .as_array()
            .unwrap()
            .iter()
            .any(|asset| asset["path"] == source
                && asset["sha256"] == aep_core::digest(IMAGE)
                && asset["source_commit"] == plan["base_commit"])
    );
    let file = tempfile::NamedTempFile::new().unwrap();
    fs::write(file.path(), plan.to_string()).unwrap();
    let apply = ["migrate", "apply", "--plan", file.path().to_str().unwrap()];
    write(root, source, b"changed bytes");
    let changed = call(root, &apply, 5);
    assert!(changed.to_string().contains("Migration source changed"));
    write(root, source, IMAGE);
    call(root, &apply, 0);
    call(root, &["migrate", "verify"], 0);
    call(root, &["check"], 0);
    for (document, reference, original, absent) in [
        (
            "lesson-learned/dogfood/note.md",
            "../../lessons-learned/dogfood/screens/capture.png",
            source,
            "lesson-learned/dogfood/screens/capture.png",
        ),
        (
            "project-roadmap/product/vision.md",
            "../../product/diagram.png",
            "product/diagram.png",
            "project-roadmap/product/diagram.png",
        ),
        (
            "project-rules/security.md",
            "../project-convention/screens/security.png",
            "project-convention/screens/security.png",
            "project-rules/screens/security.png",
        ),
    ] {
        let body = fs::read_to_string(root.join(document)).unwrap();
        assert!(body.contains(reference), "{document}: {body}");
        assert_eq!(
            fs::read(root.join(document).parent().unwrap().join(reference)).unwrap(),
            IMAGE
        );
        assert_eq!(fs::read(root.join(original)).unwrap(), IMAGE);
        assert!(!root.join(absent).exists());
    }
    write(root, source, b"drift after cutover");
    call(root, &["migrate", "verify"], 5);
    call(root, &["migrate", "plan"], 5);
}
#[test]
fn unfamiliar_asset_reference_requires_mapping_instead_of_silent_broken_link() {
    let dir = fixture("Screenshot location: screens/capture.png\n");
    let root = dir.path();
    let plan = call(root, &["migrate", "plan"], 0)["data"]["plan"].clone();
    assert!(plan["diagnostics"].as_array().unwrap().iter().any(|d| {
        d.as_str()
            .unwrap()
            .contains("map retained asset reference screens/capture.png")
    }));
    let file = tempfile::NamedTempFile::new().unwrap();
    fs::write(file.path(), plan.to_string()).unwrap();
    call(
        root,
        &["migrate", "apply", "--plan", file.path().to_str().unwrap()],
        3,
    );
    assert_eq!(
        fs::read(root.join("lessons-learned/dogfood/screens/capture.png")).unwrap(),
        IMAGE
    );
    assert!(!root.join("lesson-learned").exists());
}
