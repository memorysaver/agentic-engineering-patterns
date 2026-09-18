use std::{env, fs, path::Path};
fn main() {
    let root = Path::new("../../skills/native");
    println!("cargo:rerun-if-changed={}", root.display());
    let mut files = std::collections::BTreeMap::new();
    fn walk(root: &Path, dir: &Path, files: &mut std::collections::BTreeMap<String, String>) {
        for entry in fs::read_dir(dir).expect("native skill sources") {
            let entry = entry.unwrap();
            let path = entry.path();
            assert!(
                !entry.file_type().unwrap().is_symlink(),
                "embedded skill symlink"
            );
            if path.is_dir() {
                walk(root, &path, files);
            } else {
                let key = path
                    .strip_prefix(root)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace('\\', "/");
                files.insert(key, fs::read_to_string(path).expect("UTF-8 skill source"));
            }
        }
    }
    walk(root, root, &mut files);
    fs::write(
        Path::new(&env::var_os("OUT_DIR").unwrap()).join("skills.json"),
        serde_json::to_string(&files).unwrap(),
    )
    .unwrap();
}
