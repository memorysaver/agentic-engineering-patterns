//! Git-backed, reviewable records with optimistic conflicts and recoverable writes.
use aep_core::{Config, Error, Kind, Record, Result, digest, unique_id};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Component, Path, PathBuf},
    process::Command,
};

pub fn git_bytes(root: &Path, args: &[&str]) -> Result<Vec<u8>> {
    let out = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()?;
    if !out.status.success() {
        return Err(Error::new(
            "git",
            String::from_utf8_lossy(&out.stderr).trim().to_string(),
            6,
        ));
    }
    Ok(out.stdout)
}
pub fn git(root: &Path, args: &[&str]) -> Result<String> {
    Ok(String::from_utf8_lossy(&git_bytes(root, args)?)
        .trim_end()
        .to_string())
}
pub fn repository(path: &Path) -> Result<PathBuf> {
    let root = git(path, &["rev-parse", "--show-toplevel"])?;
    Ok(fs::canonicalize(root)?)
}
/// Reject escapes and symlinks, including existing parents of new destinations.
pub fn contained(root: &Path, relative: &str) -> Result<PathBuf> {
    let rel = Path::new(relative);
    if rel.as_os_str().is_empty() || rel.is_absolute() {
        return Err(Error::input(format!("Expected relative path: {relative}")));
    }
    let mut target = root.to_path_buf();
    for part in rel.components() {
        match part {
            Component::Normal(p) => target.push(p),
            Component::CurDir => {}
            _ => return Err(Error::input(format!("Path escapes project: {relative}"))),
        }
        match fs::symlink_metadata(&target) {
            Ok(meta) if meta.file_type().is_symlink() => {
                return Err(Error::input(format!("Symlink in managed path: {relative}")));
            }
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e.into()),
        }
    }
    Ok(target)
}
/// Validate an explicitly requested output destination, including every existing parent.
pub fn destination(path: &Path) -> Result<PathBuf> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };
    let mut out = PathBuf::new();
    for part in absolute.components() {
        match part {
            Component::ParentDir => return Err(Error::input("Destination cannot contain ..")),
            Component::CurDir => continue,
            _ => out.push(part.as_os_str()),
        }
        if fs::symlink_metadata(&out).is_ok_and(|m| m.file_type().is_symlink()) {
            // macOS exposes system temporary directories through these fixed aliases.
            // Resolve only the OS prefix; project-controlled symlinks still fail below.
            #[cfg(target_os = "macos")]
            if matches!(out.to_str(), Some("/var" | "/tmp")) {
                let expected = Path::new("/private").join(out.strip_prefix("/").unwrap());
                if fs::canonicalize(&out)? == expected {
                    out = expected;
                    continue;
                }
            }
            return Err(Error::input(format!(
                "Symlink in destination: {}",
                out.display()
            )));
        }
    }
    Ok(out)
}
pub fn read_optional(path: &Path) -> Result<Option<String>> {
    match fs::read_to_string(path) {
        Ok(s) => Ok(Some(s)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.into()),
    }
}
pub fn parse_yaml<T: serde::de::DeserializeOwned>(text: &str) -> Result<T> {
    // Mapping's visitor rejects duplicate keys; deserializing directly into a
    // BTreeMap would otherwise silently retain only the last value.
    let value: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(text).map_err(|e| Error::input(e.to_string()))?;
    serde_yaml_ng::from_value(value).map_err(|e| Error::input(e.to_string()))
}
pub fn yaml<T: Serialize>(value: &T) -> Result<String> {
    serde_yaml_ng::to_string(value).map_err(|e| Error::input(e.to_string()))
}

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub files: BTreeMap<String, String>,
    pub records: Vec<Record>,
    pub revision: String,
}
impl Snapshot {
    pub fn get(&self, id: &str) -> Result<&Record> {
        self.records
            .iter()
            .find(|r| r.id == id)
            .ok_or_else(|| Error::new("not_found", format!("No record {id}"), 2))
    }
}
#[derive(Debug, Clone)]
pub struct Store {
    pub root: PathBuf,
    pub config: Config,
    config_was_present: bool,
    runtime: PathBuf,
}
struct StoreLock(File);
impl Drop for StoreLock {
    fn drop(&mut self) {
        // Explicit unlock also releases the lock if a concurrent fork briefly
        // inherited the open file description before exec closes its descriptor.
        let _ = self.0.unlock();
    }
}
impl Store {
    pub fn open(path: &Path) -> Result<Self> {
        let root = repository(path)?;
        let config_path = contained(&root, ".aep/config.toml")?;
        let config = read_optional(&config_path)?
            .map(|s| toml::from_str::<Config>(&s).map_err(|e| Error::input(e.to_string())))
            .transpose()?
            .ok_or_else(|| {
                Error::blocked("Project has no .aep/config.toml; inspect the onboarding skill")
            })?;
        Self::with_config(root, config)
    }
    pub fn uninitialized(path: &Path) -> Result<Self> {
        Self::with_config(repository(path)?, Config::default())
    }
    pub fn with_config(root: PathBuf, mut config: Config) -> Result<Self> {
        let root = fs::canonicalize(root)?;
        for path in [
            &mut config.stores.ledger,
            &mut config.stores.roadmap,
            &mut config.stores.rules,
            &mut config.stores.lessons,
            &mut config.stores.designs,
        ] {
            *path = aep_core::normalize_scope(path)
                .ok_or_else(|| Error::input("Invalid store path"))?;
        }
        // Git owns a stable per-checkout identity; moving a checkout must not hide recovery.
        let metadata = fs::canonicalize(git(&root, &["rev-parse", "--absolute-git-dir"])?)?;
        let runtime = contained(&metadata, "aep")?;
        let config_was_present = contained(&root, ".aep/config.toml")?.exists();
        let store = Self {
            config_was_present,
            root,
            config,
            runtime,
        };
        let roots = store.roots();
        for (i, p) in roots.iter().enumerate() {
            let path = contained(&store.root, p)?;
            if path == store.root
                || p == ".git"
                || p.starts_with(".git/")
                || p == ".aep"
                || p.starts_with(".aep/")
            {
                return Err(Error::input(format!("Invalid store root {p}")));
            }
            if roots
                .iter()
                .skip(i + 1)
                .any(|other| aep_core::path_overlap(p, other))
            {
                return Err(Error::input("Canonical store roots overlap"));
            }
        }
        Ok(store)
    }
    pub fn roots(&self) -> Vec<String> {
        let s = &self.config.stores;
        vec![
            s.ledger.clone(),
            s.roadmap.clone(),
            s.rules.clone(),
            s.lessons.clone(),
            s.designs.clone(),
        ]
    }
    pub fn check_version(&self) -> Result<()> {
        self.check_config_snapshot()?;

        if self.config.schema_version != aep_core::SCHEMA {
            return Err(Error::unsupported("Unsupported project schema"));
        }
        if self.config.openspec_profile != "1.12.0-common" {
            return Err(Error::unsupported(
                "Unsupported OpenSpec compatibility profile",
            ));
        }
        if self.config.cli_version != aep_core::VERSION {
            return Err(Error::unsupported(format!(
                "Project pins CLI {}; running {}",
                self.config.cli_version,
                aep_core::VERSION
            )));
        }
        Ok(())
    }
    fn check_config_snapshot(&self) -> Result<()> {
        let raw = read_optional(&contained(&self.root, ".aep/config.toml")?)?;
        if raw.is_none() && self.config_was_present {
            return Err(Error::conflict(
                "Configuration was removed after opening the store",
            ));
        }
        if let Some(raw) = raw {
            let loaded: Config = toml::from_str(&raw).map_err(|e| Error::input(e.to_string()))?;
            let current = Self::with_config(self.root.clone(), loaded)?;
            if serde_json::to_value(&current.config)? != serde_json::to_value(&self.config)? {
                return Err(Error::conflict(
                    "Configuration changed after opening the store; retry",
                ));
            }
        }
        Ok(())
    }
    pub fn record_path(&self, kind: Kind, id: &str) -> Result<String> {
        if !aep_core::valid_id(id) {
            return Err(Error::input(format!("Invalid ID {id}")));
        }
        let s = &self.config.stores;
        let (root, ext) = match kind {
            Kind::Decision | Kind::Roadmap => (&s.roadmap, "md"),
            Kind::Lesson => (&s.lessons, "md"),
            Kind::Rule => (&s.rules, "md"),
            _ => (&s.ledger, "yaml"),
        };
        Ok(if kind == Kind::Change {
            format!("{root}/changes/{id}/change.yaml")
        } else {
            format!("{root}/{}/{id}.{ext}", kind.folder())
        })
    }
    pub fn record_bytes(&self, r: &Record) -> Result<String> {
        if [Kind::Decision, Kind::Roadmap, Kind::Lesson, Kind::Rule].contains(&r.kind) {
            let mut meta = r.clone();
            meta.description.clear();
            Ok(format!(
                "---\n{}---\n\n{}\n",
                yaml(&meta)?,
                r.description.trim_end()
            ))
        } else {
            yaml(r)
        }
    }
    fn raw_files(&self) -> Result<BTreeMap<String, String>> {
        fn walk(root: &Path, path: &Path, out: &mut BTreeMap<String, String>) -> Result<()> {
            if !path.exists() {
                return Ok(());
            }
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let p = entry.path();
                let rel = p
                    .strip_prefix(root)
                    .map_err(|e| Error::input(e.to_string()))?
                    .to_string_lossy()
                    .replace('\\', "/");
                contained(root, &rel)?;
                if entry.file_type()?.is_dir() {
                    walk(root, &p, out)?;
                } else if entry.file_type()?.is_file() {
                    if fs::metadata(&p)?.len() > 16 * 1024 * 1024 {
                        return Err(Error::input(format!("Managed text file too large: {rel}")));
                    }
                    out.insert(rel, fs::read_to_string(p)?);
                }
            }
            Ok(())
        }
        let mut files = BTreeMap::new();
        for root in self.roots() {
            walk(&self.root, &contained(&self.root, &root)?, &mut files)?;
        }
        for path in [".aep/config.toml", "AGENTS.md", "CLAUDE.md", ".gitignore"] {
            if let Some(s) = read_optional(&contained(&self.root, path)?)? {
                files.insert(path.into(), s);
            }
        }
        Ok(files)
    }
    pub fn snapshot(&self) -> Result<Snapshot> {
        self.check_config_snapshot()?;
        self.ensure_no_journal()?;
        let files = self.raw_files()?;
        let mut records = vec![];
        for (path, text) in &files {
            let Some(expected_kind) = self.path_kind(path) else {
                continue;
            };
            let mut r: Record = if path.ends_with(".md") {
                let (front, body) = frontmatter(text)?;
                let mut r: Record = parse_yaml(front)?;
                r.description = body.trim().into();
                r
            } else {
                parse_yaml(text).map_err(|e| Error::input(format!("{path}: {e}")))?
            };
            if r.kind != expected_kind || self.record_path(r.kind, &r.id)? != *path {
                return Err(Error::input(format!(
                    "Record identity/path mismatch: {path}"
                )));
            }
            // Canonical Markdown has one body; YAML uses the description field.
            if path.ends_with(".md") {
                r.description = r.description.trim_end().into();
            }
            records.push(r);
        }
        let revision = digest(serde_json::to_vec(&files)?);
        self.ensure_no_journal()?;
        if revision != digest(serde_json::to_vec(&self.raw_files()?)?) {
            return Err(Error::conflict("Store changed during read; retry"));
        }
        Ok(Snapshot {
            files,
            records,
            revision,
        })
    }
    fn path_kind(&self, path: &str) -> Option<Kind> {
        for kind in [
            Kind::Story,
            Kind::Layer,
            Kind::Wave,
            Kind::Release,
            Kind::Gate,
            Kind::Change,
            Kind::Attempt,
            Kind::Evidence,
            Kind::Review,
            Kind::Delivery,
            Kind::Decision,
            Kind::Roadmap,
            Kind::Lesson,
            Kind::Rule,
            Kind::Event,
            Kind::Import,
        ] {
            let sample = self.record_path(kind, "SAMPLE").ok()?;
            let (prefix, suffix) = sample.rsplit_once("SAMPLE")?;
            if let Some(id) = path
                .strip_prefix(prefix)
                .and_then(|p| p.strip_suffix(suffix))
                && !id.contains('/')
            {
                return Some(kind);
            }
        }
        None
    }
    fn ensure_no_journal(&self) -> Result<()> {
        if self.runtime.join("transaction.json").exists() {
            Err(Error::new(
                "recovery_required",
                "Interrupted transaction; inspect with aep recover",
                5,
            ))
        } else {
            Ok(())
        }
    }
    pub fn plan(
        &self,
        snapshot: &Snapshot,
        writes: BTreeMap<String, Option<String>>,
    ) -> Result<Transaction> {
        let mut items = vec![];
        let mut targets = BTreeSet::new();
        for (path, after) in writes {
            let path = managed_path(&path)?;
            if !targets.insert(path.clone()) {
                return Err(Error::input("Duplicate canonical transaction target"));
            }
            let dest = contained(&self.root, &path)?;
            let before = snapshot.files.get(&path).cloned();
            if read_optional(&dest)? != before {
                return Err(Error::conflict(format!(
                    "Target changed or was not part of the read snapshot: {path}"
                )));
            }
            if before != after {
                items.push(Change {
                    path,
                    before,
                    after,
                    before_mode: mode(&dest)?,
                });
            }
        }
        Ok(Transaction {
            schema_version: aep_core::SCHEMA,
            id: unique_id("txn"),
            root: self.root.to_string_lossy().into(),
            source_revision: snapshot.revision.clone(),
            changes: items,
        })
    }
    fn lock(&self) -> Result<StoreLock> {
        // Runtime state is under Git's common directory, outside tracked project records.
        fs::create_dir_all(&self.runtime)?;
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(self.runtime.join("lock"))?;
        file.try_lock()
            .map_err(|e| Error::conflict(format!("Store is busy: {e}")))?;
        Ok(StoreLock(file))
    }
    pub fn apply(&self, plan: &Transaction) -> Result<()> {
        self.check_version()?;
        if plan.root != self.root.to_string_lossy() || plan.schema_version != aep_core::SCHEMA {
            return Err(Error::input("Plan belongs to another project/schema"));
        }
        let _lock = self.lock()?;
        self.ensure_no_journal()?;
        if digest(serde_json::to_vec(&self.raw_files()?)?) != plan.source_revision {
            return Err(Error::conflict("Plan source changed; rebuild the plan"));
        }
        let mut targets = BTreeSet::new();
        for c in &plan.changes {
            if !targets.insert(managed_path(&c.path)?) {
                return Err(Error::input("Duplicate canonical transaction target"));
            }
            if read_optional(&contained(&self.root, &c.path)?)? != c.before
                || mode(&contained(&self.root, &c.path)?)? != c.before_mode
            {
                return Err(Error::conflict(format!("Plan target changed: {}", c.path)));
            }
        }
        if plan.changes.is_empty() {
            return Ok(());
        }
        let journal = self.runtime.join("transaction.json");
        atomic_write(&journal, Some(&serde_json::to_string_pretty(plan)?))?;
        for c in &plan.changes {
            let result = (|| {
                let target = contained(&self.root, &c.path)?;
                if read_optional(&target)? != c.before || mode(&target)? != c.before_mode {
                    return Err(Error::conflict(format!("Concurrent edit: {}", c.path)));
                }
                atomic_write_mode(&target, c.after.as_deref(), c.before_mode)
            })();
            if let Err(e) = result {
                return Err(e.changed());
            }
        }
        fs::remove_file(journal).map_err(|e| Error::from(e).changed())?;
        sync_dir(&self.runtime)?;
        Ok(())
    }
    pub fn recovery(&self) -> Result<Option<Transaction>> {
        read_optional(&self.runtime.join("transaction.json"))?
            .map(|s| serde_json::from_str(&s).map_err(Error::from))
            .transpose()
    }
    pub fn recover(&self, rollback: bool) -> Result<Option<String>> {
        self.check_version()?;
        let _lock = self.lock()?;
        let Some(plan) = self.recovery()? else {
            return Ok(None);
        };
        if plan.schema_version != aep_core::SCHEMA {
            return Err(Error::input("Recovery journal has an unsupported schema"));
        }
        let mut targets = BTreeSet::new();
        for c in &plan.changes {
            if !targets.insert(managed_path(&c.path)?) {
                return Err(Error::input("Duplicate canonical transaction target"));
            }
            let actual = read_optional(&contained(&self.root, &c.path)?)?;
            if (actual != c.before && actual != c.after)
                || (actual.is_some()
                    && c.before_mode.is_some()
                    && mode(&contained(&self.root, &c.path)?)? != c.before_mode)
            {
                return Err(Error::conflict(format!(
                    "Later edit prevents recovery: {}",
                    c.path
                )));
            }
        }
        for c in &plan.changes {
            atomic_write_mode(
                &contained(&self.root, &c.path)?,
                if rollback {
                    c.before.as_deref()
                } else {
                    c.after.as_deref()
                },
                c.before_mode,
            )
            .map_err(Error::changed)?;
        }
        fs::remove_file(self.runtime.join("transaction.json"))?;
        sync_dir(&self.runtime)?;
        Ok(Some(plan.id))
    }
}
pub fn frontmatter(text: &str) -> Result<(&str, &str)> {
    let rest = text
        .strip_prefix("---\n")
        .ok_or_else(|| Error::input("Missing YAML frontmatter"))?;
    rest.split_once("\n---\n")
        .ok_or_else(|| Error::input("Unclosed YAML frontmatter"))
}
fn sync_dir(path: &Path) -> Result<()> {
    #[cfg(unix)]
    File::open(path)?.sync_all()?;
    #[cfg(not(unix))]
    let _ = path;
    Ok(())
}
pub fn atomic_write(path: &Path, text: Option<&str>) -> Result<()> {
    atomic_write_mode(path, text, None)
}
fn managed_path(path: &str) -> Result<String> {
    let normalized =
        aep_core::normalize_scope(path).ok_or_else(|| Error::input("Invalid transaction path"))?;
    if normalized == "."
        || normalized == ".git"
        || normalized.starts_with(".git/")
        || normalized == ".aep"
        || (normalized.starts_with(".aep/") && normalized != ".aep/config.toml")
    {
        return Err(Error::input("Reserved transaction path"));
    }
    Ok(normalized)
}
fn mode(path: &Path) -> Result<Option<u32>> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        match fs::metadata(path) {
            Ok(meta) => Ok(Some(meta.permissions().mode())),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        Ok(None)
    }
}
fn atomic_write_mode(path: &Path, text: Option<&str>, restore_mode: Option<u32>) -> Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| Error::input("No parent directory"))?;
    if let Some(text) = text {
        fs::create_dir_all(parent)?;
        let tmp = parent.join(format!(".{}", unique_id("aep-write")));
        let result = (|| -> Result<()> {
            let mut file = OpenOptions::new().write(true).create_new(true).open(&tmp)?;
            if let Ok(meta) = fs::metadata(path) {
                file.set_permissions(meta.permissions())?;
            }
            #[cfg(unix)]
            if let Some(mode) = restore_mode {
                use std::os::unix::fs::PermissionsExt;
                file.set_permissions(fs::Permissions::from_mode(mode))?;
            }
            #[cfg(not(unix))]
            let _ = restore_mode;
            file.write_all(text.as_bytes())?;
            file.sync_all()?;
            fs::rename(&tmp, path)?;
            sync_dir(parent)
        })();
        if result.is_err() {
            let _ = fs::remove_file(&tmp);
        }
        result
    } else {
        if path.exists() {
            fs::remove_file(path)?;
            sync_dir(parent)?;
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Change {
    pub path: String,
    pub before: Option<String>,
    pub after: Option<String>,
    #[serde(default)]
    pub before_mode: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Transaction {
    pub schema_version: u32,
    pub id: String,
    pub root: String,
    pub source_revision: String,
    pub changes: Vec<Change>,
}

#[cfg(test)]
mod tests {
    use super::*;
    fn setup() -> (tempfile::TempDir, Store) {
        let d = tempfile::tempdir().unwrap();
        git(d.path(), &["init", "-q"]).unwrap();
        let s = Store::uninitialized(d.path()).unwrap();
        (d, s)
    }
    #[test]
    fn stale_plan_preserves_manual_edit() {
        let (_d, s) = setup();
        let snap = s.snapshot().unwrap();
        let plan = s
            .plan(
                &snap,
                BTreeMap::from([("project-rules/README.md".into(), Some("new".into()))]),
            )
            .unwrap();
        fs::create_dir_all(s.root.join("project-rules")).unwrap();
        fs::write(s.root.join("project-rules/README.md"), "manual").unwrap();
        assert_eq!(s.apply(&plan).unwrap_err().exit_code, 5);
        assert_eq!(
            fs::read_to_string(s.root.join("project-rules/README.md")).unwrap(),
            "manual"
        );
    }
    #[test]
    fn recovery_completes_partial_set_and_refuses_later_edits() {
        let (_d, s) = setup();
        let snap = s.snapshot().unwrap();
        let plan = s
            .plan(
                &snap,
                BTreeMap::from([
                    ("project-rules/a.md".into(), Some("A".into())),
                    ("project-rules/b.md".into(), Some("B".into())),
                ]),
            )
            .unwrap();
        fs::create_dir_all(&s.runtime).unwrap();
        atomic_write(
            &s.runtime.join("transaction.json"),
            Some(&serde_json::to_string(&plan).unwrap()),
        )
        .unwrap();
        atomic_write(&s.root.join("project-rules/a.md"), Some("later")).unwrap();
        assert_eq!(s.recover(false).unwrap_err().exit_code, 5);
        atomic_write(&s.root.join("project-rules/a.md"), Some("A")).unwrap();
        s.recover(false).unwrap();
        assert_eq!(
            fs::read_to_string(s.root.join("project-rules/b.md")).unwrap(),
            "B"
        );
        assert!(s.recovery().unwrap().is_none());
    }
    #[test]
    fn yaml_rejects_duplicates() {
        assert!(parse_yaml::<BTreeMap<String, String>>("a: one\na: two\n").is_err());
    }
    #[cfg(unix)]
    #[test]
    fn symlink_parent_cannot_escape() {
        let (d, s) = setup();
        std::os::unix::fs::symlink(d.path(), s.root.join("escape")).unwrap();
        assert!(contained(&s.root, "escape/test").is_err());
        assert!(contained(&s.root, "../test").is_err());
        assert!(destination(&s.root.join("escape/test")).is_err());
    }
    #[test]
    fn reserved_aliases_and_duplicate_targets_are_rejected() {
        let (_d, s) = setup();
        let snap = s.snapshot().unwrap();
        assert!(
            s.plan(
                &snap,
                BTreeMap::from([("./.git/config".into(), Some("bad".into()))])
            )
            .is_err()
        );
        assert!(
            s.plan(
                &snap,
                BTreeMap::from([
                    ("./project-rules/a.md".into(), Some("A".into())),
                    ("project-rules/a.md".into(), Some("B".into()))
                ])
            )
            .is_err()
        );
        let mut config = Config::default();
        config.stores.ledger = "./.git".into();
        assert!(Store::with_config(s.root.clone(), config).is_err());
    }
    #[test]
    fn instructions_and_config_drift_cannot_be_adopted_by_a_stale_reader() {
        let (_d, s) = setup();
        fs::write(s.root.join("AGENTS.md"), "original").unwrap();
        let snap = s.snapshot().unwrap();
        fs::write(s.root.join("AGENTS.md"), "user edit").unwrap();
        assert!(
            s.plan(
                &snap,
                BTreeMap::from([("AGENTS.md".into(), Some("generated".into()))])
            )
            .is_err()
        );
        fs::create_dir_all(s.root.join(".aep")).unwrap();
        let mut config = s.config.clone();
        config.cli_version = "future".into();
        fs::write(
            s.root.join(".aep/config.toml"),
            toml::to_string(&config).unwrap(),
        )
        .unwrap();
        assert!(s.snapshot().is_err());
        fs::write(
            s.root.join(".aep/config.toml"),
            toml::to_string(&Config::default()).unwrap(),
        )
        .unwrap();
        let initialized = Store::open(&s.root).unwrap();
        fs::remove_file(s.root.join(".aep/config.toml")).unwrap();
        assert!(initialized.snapshot().is_err());
    }
    #[test]
    fn normalized_store_roots_keep_records_visible() {
        let (_d, s) = setup();
        let mut config = Config::default();
        config.stores.ledger = "./SAMPLE-ledger/".into();
        let s = Store::with_config(s.root.clone(), config).unwrap();
        let record = Record::new(Kind::Story, "S", "Story");
        let plan = s
            .plan(
                &s.snapshot().unwrap(),
                BTreeMap::from([(
                    s.record_path(Kind::Story, "S").unwrap(),
                    Some(s.record_bytes(&record).unwrap()),
                )]),
            )
            .unwrap();
        s.apply(&plan).unwrap();
        assert_eq!(s.snapshot().unwrap().get("S").unwrap().id, "S");
    }
    #[cfg(unix)]
    #[test]
    fn deletion_recovery_restores_modes_and_chmod_conflicts() {
        use std::os::unix::fs::PermissionsExt;
        let (_d, s) = setup();
        let path = s.root.join("project-rules/tool.md");
        atomic_write(&path, Some("script")).unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
        let plan = s
            .plan(
                &s.snapshot().unwrap(),
                BTreeMap::from([("project-rules/tool.md".into(), None)]),
            )
            .unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();
        assert!(s.apply(&plan).is_err());
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
        fs::create_dir_all(&s.runtime).unwrap();
        atomic_write(
            &s.runtime.join("transaction.json"),
            Some(&serde_json::to_string(&plan).unwrap()),
        )
        .unwrap();
        fs::remove_file(&path).unwrap();
        s.recover(true).unwrap();
        assert_eq!(
            fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            0o755
        );
    }
    #[test]
    fn moving_repository_keeps_recovery_visible() {
        let parent = tempfile::tempdir().unwrap();
        let old = parent.path().join("old");
        fs::create_dir(&old).unwrap();
        git(&old, &["init", "-q"]).unwrap();
        let s = Store::uninitialized(&old).unwrap();
        let plan = s
            .plan(
                &s.snapshot().unwrap(),
                BTreeMap::from([("AGENTS.md".into(), Some("entry".into()))]),
            )
            .unwrap();
        fs::create_dir_all(&s.runtime).unwrap();
        atomic_write(
            &s.runtime.join("transaction.json"),
            Some(&serde_json::to_string(&plan).unwrap()),
        )
        .unwrap();
        let new = parent.path().join("new");
        fs::rename(old, &new).unwrap();
        let relocated = Store::uninitialized(&new).unwrap();
        assert_eq!(relocated.snapshot().unwrap_err().code, "recovery_required");
        relocated.recover(false).unwrap();
        assert_eq!(fs::read_to_string(new.join("AGENTS.md")).unwrap(), "entry");
    }
}
