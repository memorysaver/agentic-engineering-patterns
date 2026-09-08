//! Deterministic project records and constraints. Task interpretation belongs to the agent.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const SCHEMA: u32 = 1;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize)]
pub struct Error {
    pub code: String,
    pub message: String,
    pub exit_code: i32,
    pub side_effects: bool,
}
impl Error {
    pub fn new(code: &str, message: impl Into<String>, exit_code: i32) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            exit_code,
            side_effects: false,
        }
    }
    pub fn input(message: impl Into<String>) -> Self {
        Self::new("invalid_input", message, 2)
    }
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new("conflict", message, 5)
    }
    pub fn blocked(message: impl Into<String>) -> Self {
        Self::new("prerequisite", message, 3)
    }
    pub fn unsupported(message: impl Into<String>) -> Self {
        Self::new("unsupported", message, 4)
    }
    pub fn changed(mut self) -> Self {
        self.side_effects = true;
        self
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}
impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::new("io", e.to_string(), 6)
    }
}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::input(e.to_string())
    }
}

pub fn digest(bytes: impl AsRef<[u8]>) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
pub fn now() -> String {
    chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}
pub fn unique_id(prefix: &str) -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static SEQUENCE: AtomicU64 = AtomicU64::new(0);
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!(
        "{prefix}-{time:x}-{:x}-{:x}",
        std::process::id(),
        SEQUENCE.fetch_add(1, Ordering::Relaxed)
    )
}
pub fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 128
        && id.as_bytes()[0].is_ascii_alphanumeric()
        && id
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"._-".contains(&c))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Story,
    Layer,
    Wave,
    Release,
    Gate,
    Change,
    Attempt,
    Evidence,
    Review,
    Delivery,
    Decision,
    Roadmap,
    Lesson,
    Rule,
    Event,
    Import,
}
impl Kind {
    pub fn name(self) -> &'static str {
        match self {
            Self::Story => "story",
            Self::Layer => "layer",
            Self::Wave => "wave",
            Self::Release => "release",
            Self::Gate => "gate",
            Self::Change => "change",
            Self::Attempt => "attempt",
            Self::Evidence => "evidence",
            Self::Review => "review",
            Self::Delivery => "delivery",
            Self::Decision => "decision",
            Self::Roadmap => "roadmap",
            Self::Lesson => "lesson",
            Self::Rule => "rule",
            Self::Event => "event",
            Self::Import => "import",
        }
    }
    pub fn folder(self) -> &'static str {
        match self {
            Self::Story => "stories",
            Self::Layer => "layers",
            Self::Wave => "waves",
            Self::Release => "releases",
            Self::Gate => "gates",
            Self::Change => "changes",
            Self::Attempt => "attempts",
            Self::Evidence => "evidence",
            Self::Review => "reviews",
            Self::Delivery => "deliveries",
            Self::Decision => "decisions",
            Self::Roadmap => "plans",
            Self::Lesson => "observations",
            Self::Rule => "records",
            Self::Event => "events",
            Self::Import => "imports",
        }
    }
    pub fn parse(s: &str) -> Result<Self> {
        serde_json::from_value(Value::String(s.into())).map_err(Error::from)
    }
    pub fn protected(self) -> bool {
        matches!(
            self,
            Self::Attempt
                | Self::Evidence
                | Self::Review
                | Self::Delivery
                | Self::Event
                | Self::Import
        )
    }
}

fn schema() -> u32 {
    SCHEMA
}
fn pending() -> String {
    "pending".into()
}
fn standard() -> String {
    "standard".into()
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Record {
    #[serde(default = "schema")]
    pub schema_version: u32,
    pub kind: Kind,
    pub id: String,
    pub title: String,
    #[serde(default = "pending")]
    pub status: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub refs: Vec<String>,
    #[serde(default)]
    pub paths: Vec<String>,
    #[serde(default)]
    pub depends_on: Vec<String>,
    #[serde(default)]
    pub layer: Option<String>,
    #[serde(default)]
    pub wave: Option<String>,
    #[serde(default)]
    pub release: Option<String>,
    #[serde(default)]
    pub change: Option<String>,
    #[serde(default)]
    pub changes: Vec<String>,
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub required_checks: Vec<String>,
    #[serde(default)]
    pub required_gates: Vec<String>,
    #[serde(default = "standard")]
    pub risk: String,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub data: BTreeMap<String, Value>,
}
impl Record {
    pub fn new(kind: Kind, id: &str, title: &str) -> Self {
        Self {
            schema_version: SCHEMA,
            kind,
            id: id.into(),
            title: title.into(),
            status: pending(),
            description: String::new(),
            refs: vec![],
            paths: vec![],
            depends_on: vec![],
            layer: None,
            wave: None,
            release: None,
            change: None,
            changes: vec![],
            owner: None,
            required_checks: vec![],
            required_gates: vec![],
            risk: standard(),
            created_at: Some(now()),
            updated_at: Some(now()),
            data: BTreeMap::new(),
        }
    }
    pub fn text(&self, key: &str) -> Option<&str> {
        self.data.get(key).and_then(Value::as_str)
    }
    pub fn set(&mut self, key: &str, value: impl Into<Value>) {
        self.data.insert(key.into(), value.into());
    }
    pub fn revision(&self) -> String {
        digest(serde_json::to_vec(self).expect("serializable record"))
    }
    pub fn change_ids(&self) -> Vec<&str> {
        self.change
            .iter()
            .chain(&self.changes)
            .map(String::as_str)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }
    pub fn links(&self) -> Vec<&str> {
        self.refs
            .iter()
            .chain(&self.depends_on)
            .chain(&self.required_gates)
            .chain(&self.changes)
            .map(String::as_str)
            .chain(self.layer.as_deref())
            .chain(self.wave.as_deref())
            .chain(self.release.as_deref())
            .chain(self.change.as_deref())
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Stores {
    pub ledger: String,
    pub roadmap: String,
    pub rules: String,
    pub lessons: String,
    pub designs: String,
}
impl Default for Stores {
    fn default() -> Self {
        Self {
            ledger: "project-ledger".into(),
            roadmap: "project-roadmap".into(),
            rules: "project-rules".into(),
            lessons: "lesson-learned".into(),
            designs: "docs/design".into(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Check {
    pub id: String,
    pub command: Vec<String>,
    #[serde(default = "local_environment")]
    pub environment: String,
    #[serde(default)]
    pub rules: Vec<String>,
    #[serde(default = "dot")]
    pub cwd: String,
    #[serde(default = "timeout")]
    pub timeout_seconds: u64,
    #[serde(default)]
    pub paths: Vec<String>,
    #[serde(default)]
    pub env: Vec<String>,
    #[serde(default = "yes")]
    pub required: bool,
}
fn local_environment() -> String {
    "local".into()
}
fn dot() -> String {
    ".".into()
}
fn timeout() -> u64 {
    300
}
fn yes() -> bool {
    true
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Policy {
    pub max_parallel: usize,
    pub independent_review: bool,
    pub required_checks: Vec<String>,
    pub protected_paths: Vec<String>,
}
impl Default for Policy {
    fn default() -> Self {
        Self {
            max_parallel: 2,
            independent_review: true,
            required_checks: vec![],
            protected_paths: vec![".aep/".into(), "project-rules/".into(), ".github/".into()],
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    #[serde(skip)]
    pub context_revision: String,
    pub schema_version: u32,
    pub cli_version: String,
    pub openspec_profile: String,
    pub stores: Stores,
    pub policy: Policy,
    pub checks: Vec<Check>,
    pub skill_paths: Vec<String>,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            context_revision: String::new(),
            schema_version: SCHEMA,
            cli_version: VERSION.into(),
            openspec_profile: "1.12.0-common".into(),
            stores: Stores::default(),
            policy: Policy::default(),
            checks: vec![],
            skill_paths: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Diagnostic {
    pub code: String,
    pub severity: String,
    pub record: Option<String>,
    pub message: String,
}
impl Diagnostic {
    pub fn error(code: &str, record: Option<&str>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            severity: "error".into(),
            record: record.map(String::from),
            message: message.into(),
        }
    }
}
pub fn validate(records: &[Record], config: &Config) -> Vec<Diagnostic> {
    let mut out = vec![];
    let mut ids = BTreeMap::new();
    let checks: BTreeSet<_> = config.checks.iter().map(|c| c.id.as_str()).collect();
    if config.schema_version != SCHEMA {
        out.push(Diagnostic::error(
            "schema_version",
            None,
            "Unsupported configuration schema",
        ));
    }
    if config.policy.max_parallel == 0 {
        out.push(Diagnostic::error(
            "capacity",
            None,
            "max_parallel must be positive",
        ));
    }
    for c in &config.checks {
        if !valid_id(&c.id)
            || c.command.is_empty()
            || c.command[0].is_empty()
            || c.timeout_seconds == 0
        {
            out.push(Diagnostic::error(
                "check_config",
                None,
                format!("Invalid check {}", c.id),
            ));
        }
    }
    if checks.len() != config.checks.len() {
        out.push(Diagnostic::error(
            "duplicate_check",
            None,
            "Check IDs must be unique",
        ));
    }
    for id in &config.policy.required_checks {
        if !checks.contains(id.as_str()) {
            out.push(Diagnostic::error("missing_check", None, id.clone()));
        }
    }
    for r in records {
        if !valid_id(&r.id) || r.title.trim().is_empty() {
            out.push(Diagnostic::error(
                "record_shape",
                Some(&r.id),
                "Record needs a safe ID and title",
            ));
        }
        if r.schema_version != SCHEMA {
            out.push(Diagnostic::error(
                "schema_version",
                Some(&r.id),
                "Unsupported record schema",
            ));
        }
        if ids.insert(r.id.as_str(), r).is_some() {
            out.push(Diagnostic::error(
                "duplicate_id",
                Some(&r.id),
                "IDs are unique across stores",
            ));
        }
        if !["light", "standard", "deep"].contains(&r.risk.as_str()) {
            out.push(Diagnostic::error("risk", Some(&r.id), "Unknown risk tier"));
        }
        for path in &r.paths {
            if normalize_scope(path).is_none() {
                out.push(Diagnostic::error(
                    "invalid_scope",
                    Some(&r.id),
                    path.clone(),
                ));
            }
        }
        for c in &r.required_checks {
            if !checks.contains(c.as_str()) {
                out.push(Diagnostic::error("missing_check", Some(&r.id), c.clone()));
            }
        }
    }
    for r in records {
        for link in r.links() {
            if !ids.contains_key(link) {
                out.push(Diagnostic::error("missing_reference", Some(&r.id), link));
            }
        }
        for (link, kind) in [
            (&r.layer, Kind::Layer),
            (&r.wave, Kind::Wave),
            (&r.release, Kind::Release),
            (&r.change, Kind::Change),
        ] {
            if let Some(id) = link
                && let Some(target) = ids.get(id.as_str())
                && target.kind != kind
            {
                out.push(Diagnostic::error(
                    "reference_kind",
                    Some(&r.id),
                    format!("{id} must be {}", kind.name()),
                ));
            }
        }
        for id in &r.changes {
            if ids
                .get(id.as_str())
                .is_some_and(|target| target.kind != Kind::Change)
            {
                out.push(Diagnostic::error(
                    "reference_kind",
                    Some(&r.id),
                    format!("{id} must be a change"),
                ));
            }
        }
        for id in &r.required_gates {
            if let Some(g) = ids.get(id.as_str())
                && g.kind != Kind::Gate
            {
                out.push(Diagnostic::error(
                    "reference_kind",
                    Some(&r.id),
                    format!("{id} is not a gate"),
                ));
            }
        }
        if r.kind == Kind::Story
            && ["integrated", "released"].contains(&r.status.as_str())
            && !has_delivery(r, &ids)
        {
            out.push(Diagnostic::error(
                "missing_delivery",
                Some(&r.id),
                "Completion requires a delivery receipt for this story",
            ));
        }
    }
    fn visit<'a>(
        id: &'a str,
        ids: &BTreeMap<&'a str, &'a Record>,
        active: &mut BTreeSet<&'a str>,
        done: &mut BTreeSet<&'a str>,
    ) -> bool {
        if active.contains(id) {
            return true;
        }
        if done.contains(id) {
            return false;
        }
        active.insert(id);
        if let Some(r) = ids.get(id) {
            for dep in r.depends_on.iter().chain(&r.required_gates) {
                if visit(dep, ids, active, done) {
                    return true;
                }
            }
        }
        active.remove(id);
        done.insert(id);
        false
    }
    let mut done = BTreeSet::new();
    for id in ids.keys() {
        if visit(id, &ids, &mut BTreeSet::new(), &mut done) {
            out.push(Diagnostic::error(
                "dependency_cycle",
                Some(id),
                "Dependency cycle",
            ));
            break;
        }
    }
    out
}
fn has_delivery(r: &Record, ids: &BTreeMap<&str, &Record>) -> bool {
    r.text("delivery")
        .and_then(|id| ids.get(id))
        .is_some_and(|d| {
            d.kind == Kind::Delivery
                && d.status == "integrated"
                && d.refs.contains(&r.id)
                && d.text("head").is_some()
        })
}
#[derive(Debug, Serialize)]
pub struct Readiness {
    pub story: String,
    pub ready: bool,
    pub reasons: Vec<String>,
}
pub fn dependency_records<'a>(story: &Record, records: &'a [Record]) -> Vec<&'a Record> {
    let mut queue = story.depends_on.clone();
    let mut seen = BTreeSet::new();
    let mut result = vec![];
    for id in [&story.layer, &story.wave].into_iter().flatten() {
        if let Some(r) = records.iter().find(|r| &r.id == id) {
            queue.extend(r.depends_on.iter().cloned());
        }
    }
    while let Some(id) = queue.pop() {
        if !seen.insert(id.clone()) {
            continue;
        }
        if let Some(r) = records.iter().find(|r| r.id == id) {
            if matches!(r.kind, Kind::Layer | Kind::Wave | Kind::Release) {
                queue.extend(scope_stories(&[id], records));
                queue.extend(r.depends_on.iter().cloned());
            }
            queue.extend(r.required_gates.iter().cloned());
            result.push(r);
        }
    }
    result
}
pub fn readiness(story: &Record, records: &[Record], config: &Config) -> Readiness {
    let ids: BTreeMap<_, _> = records.iter().map(|r| (r.id.as_str(), r)).collect();
    let mut reasons = vec![];
    if story.kind != Kind::Story {
        reasons.push("Subject is not a story".into());
    }
    if !["pending", "ready"].contains(&story.status.as_str()) {
        reasons.push(format!("Story status is {}", story.status));
    }
    for r in dependency_records(story, records) {
        let complete = match r.kind {
            Kind::Story => {
                ["integrated", "released"].contains(&r.status.as_str())
                    && has_delivery(r, &ids)
                    && r.text("delivery")
                        .and_then(|id| ids.get(id))
                        .is_some_and(|d| d.data.get("tree_verified") != Some(&Value::Bool(false)))
            }
            Kind::Gate => gate_current(r, records, config),
            Kind::Layer | Kind::Wave => {
                !scope_stories(std::slice::from_ref(&r.id), records).is_empty()
            }
            Kind::Release => r.status == "released",
            _ => false,
        };
        if !complete {
            reasons.push(format!(
                "Dependency {} lacks required completion evidence",
                r.id
            ));
        }
    }
    for dep in &story.depends_on {
        if !ids.contains_key(dep.as_str()) {
            reasons.push(format!("Missing dependency {dep}"));
        }
    }
    if story.change_ids().is_empty() {
        reasons.push("An accepted change contract is required".into());
    }
    for id in story.change_ids() {
        if !ids.get(id).is_some_and(|c| {
            c.kind == Kind::Change
                && ["accepted", "published", "closed"].contains(&c.status.as_str())
        }) {
            reasons.push(format!("Change {id} is not accepted"));
        }
    }
    let mut gates: BTreeSet<&str> = story.required_gates.iter().map(String::as_str).collect();
    for id in [&story.layer, &story.wave].into_iter().flatten() {
        if let Some(container) = ids.get(id.as_str()) {
            gates.extend(container.required_gates.iter().map(String::as_str));
        }
    }
    for gate in gates {
        match ids.get(gate) {
            Some(g) if gate_current(g, records, config) => {}
            _ => reasons.push(format!("Gate {gate} is missing, failed, or stale")),
        }
    }
    let active: Vec<_> = records
        .iter()
        .filter(|r| {
            r.kind == Kind::Attempt
                && ["prepared", "running", "review"].contains(&r.status.as_str())
        })
        .collect();
    if active.len() >= config.policy.max_parallel {
        reasons.push("Project attempt capacity reached".into());
    }
    for attempt in active {
        if attempt.refs.contains(&story.id) {
            reasons.push(format!("Claimed by {}", attempt.id));
        }
        for id in &attempt.refs {
            if let Some(other) = ids.get(id.as_str())
                && other.kind == Kind::Story
                && story
                    .paths
                    .iter()
                    .any(|a| other.paths.iter().any(|b| path_overlap(a, b)))
            {
                reasons.push(format!("Write scope overlaps {id}"));
            }
        }
    }
    Readiness {
        story: story.id.clone(),
        ready: reasons.is_empty(),
        reasons,
    }
}
pub fn path_overlap(a: &str, b: &str) -> bool {
    let (Some(a), Some(b)) = (normalize_scope(a), normalize_scope(b)) else {
        return false;
    };
    a == "."
        || b == "."
        || a == b
        || a.starts_with(&format!("{b}/"))
        || b.starts_with(&format!("{a}/"))
}
pub fn normalize_scope(path: &str) -> Option<String> {
    if path.is_empty()
        || path.starts_with('/')
        || path.contains('\\')
        || path.contains(':')
        || path.contains('\0')
    {
        return None;
    }
    let mut parts = vec![];
    for part in path.split('/') {
        match part {
            ".." => return None,
            "" | "." => {}
            _ => parts.push(part),
        }
    }
    Some(if parts.is_empty() {
        ".".into()
    } else {
        parts.join("/")
    })
}
pub fn scope_stories(subjects: &[String], records: &[Record]) -> BTreeSet<String> {
    let mut scope: BTreeSet<String> = subjects.iter().cloned().collect();
    for container in records.iter().filter(|r| {
        subjects.contains(&r.id)
            && matches!(
                r.kind,
                Kind::Layer | Kind::Wave | Kind::Release | Kind::Change
            )
    }) {
        scope.extend(container.refs.iter().cloned());
    }
    records
        .iter()
        .filter(|r| {
            r.kind == Kind::Story
                && (scope.contains(&r.id)
                    || [&r.layer, &r.wave, &r.release]
                        .into_iter()
                        .flatten()
                        .any(|id| scope.contains(id))
                    || r.change_ids().iter().any(|id| scope.contains(*id)))
        })
        .map(|r| r.id.clone())
        .collect()
}
pub fn gate_scope(gate: &Record, records: &[Record], config: &Config) -> String {
    let members = scope_stories(&gate.refs, records);
    let mut subject_ids: BTreeSet<&str> = gate
        .refs
        .iter()
        .chain(&members)
        .map(String::as_str)
        .collect();
    let mut pending: Vec<&str> = subject_ids.iter().copied().collect();
    let mut seen = BTreeSet::new();
    while let Some(id) = pending.pop() {
        if !seen.insert(id) {
            continue;
        }
        if let Some(record) = records.iter().find(|r| r.id == id) {
            for link in record
                .refs
                .iter()
                .map(String::as_str)
                .chain(record.change_ids())
            {
                if records.iter().any(|r| {
                    r.id == link
                        && matches!(
                            r.kind,
                            Kind::Change
                                | Kind::Decision
                                | Kind::Roadmap
                                | Kind::Rule
                                | Kind::Lesson
                        )
                }) {
                    subject_ids.insert(link);
                    pending.push(link);
                }
            }
        }
    }
    let mut criteria = gate.data.clone();
    for key in ["evidence", "missing", "scope_digest"] {
        criteria.remove(key);
    }
    let subjects: Vec<_> = records
        .iter()
        .filter(|r| {
            subject_ids.contains(r.id.as_str())
                || (matches!(
                    r.kind,
                    Kind::Evidence | Kind::Review | Kind::Delivery | Kind::Attempt
                ) && r.refs.iter().any(|id| subject_ids.contains(id.as_str())))
        })
        .map(|r| (&r.id, r.revision()))
        .collect();
    let gates: Vec<_> = gate
        .required_gates
        .iter()
        .map(|id| {
            (
                id,
                records.iter().find(|r| &r.id == id).map(Record::revision),
            )
        })
        .collect();
    let evidence: Vec<_> = gate
        .data
        .get("evidence")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(|id| {
            (
                id,
                records.iter().find(|r| r.id == id).map(Record::revision),
            )
        })
        .collect();
    digest(
        serde_json::to_vec(&(
            subjects,
            gates,
            evidence,
            &gate.required_checks,
            &gate.description,
            &gate.owner,
            &criteria,
            &config.checks,
            &config.policy,
            &config.context_revision,
        ))
        .unwrap(),
    )
}
pub fn gate_current(gate: &Record, records: &[Record], config: &Config) -> bool {
    fn visit(g: &Record, records: &[Record], config: &Config, seen: &mut BTreeSet<String>) -> bool {
        if !seen.insert(g.id.clone())
            || g.kind != Kind::Gate
            || g.status != "pass"
            || g.text("scope_digest") != Some(gate_scope(g, records, config).as_str())
        {
            return false;
        }
        let receipts = g.data.get("evidence").and_then(Value::as_array);
        if !receipts.is_some_and(|ids| {
            !ids.is_empty()
                && ids.iter().all(|id| {
                    id.as_str()
                        .and_then(|id| records.iter().find(|r| r.id == id))
                        .is_some_and(|r| {
                            matches!(r.kind, Kind::Evidence | Kind::Review)
                                && ["pass", "pass_with_notes"].contains(&r.status.as_str())
                        })
                })
        }) {
            return false;
        }
        let valid = g.required_gates.iter().all(|id| {
            records
                .iter()
                .find(|r| &r.id == id)
                .is_some_and(|r| visit(r, records, config, seen))
        });
        seen.remove(&g.id);
        valid
    }
    visit(gate, records, config, &mut BTreeSet::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dependency_containers_keep_gates_and_unverified_integrations_blocked() {
        let config = Config::default();
        let mut a = Record::new(Kind::Story, "A", "Integrated work");
        a.status = "integrated".into();
        a.set("delivery", "D");
        let mut delivery = Record::new(Kind::Delivery, "D", "Integration");
        delivery.status = "integrated".into();
        delivery.refs = vec!["A".into()];
        delivery.set("head", "abc");
        let mut layer = Record::new(Kind::Layer, "L", "Prerequisite layer");
        layer.refs = vec!["A".into()];
        layer.required_gates = vec!["G".into()];
        let mut gate = Record::new(Kind::Gate, "G", "Required gate");
        gate.status = "fail".into();
        let mut change = Record::new(Kind::Change, "C", "Contract");
        change.status = "accepted".into();
        let mut b = Record::new(Kind::Story, "B", "Next work");
        b.depends_on = vec!["L".into()];
        b.change = Some("C".into());
        let mut records = vec![a, delivery, layer, gate, change, b.clone()];
        assert!(!readiness(&b, &records, &config).ready);
        records[2].required_gates.clear();
        assert!(readiness(&b, &records, &config).ready);
        records[1].set("tree_verified", false);
        assert!(!readiness(&b, &records, &config).ready);
        assert!(
            validate(&records, &config).is_empty(),
            "Actual integration remains a valid recorded fact"
        );
    }
    #[test]
    fn gate_proof_tracks_members_all_contracts_and_failed_receipts() {
        let config = Config::default();
        let layer = Record::new(Kind::Layer, "L", "Scope");
        let mut story = Record::new(Kind::Story, "S", "Work");
        story.layer = Some("L".into());
        story.changes = vec!["C1".into(), "C2".into()];
        let c1 = Record::new(Kind::Change, "C1", "First contract");
        let c2 = Record::new(Kind::Change, "C2", "Second contract");
        let mut evidence = Record::new(Kind::Evidence, "E", "Check");
        evidence.status = "pass".into();
        evidence.refs = vec!["S".into()];
        let mut gate = Record::new(Kind::Gate, "G", "Gate");
        gate.status = "pass".into();
        gate.refs = vec!["L".into()];
        gate.set("evidence", serde_json::json!(["E"]));
        let mut records = vec![layer, story, c1, c2, evidence];
        gate.set("scope_digest", gate_scope(&gate, &records, &config));
        assert!(gate_current(&gate, &records, &config));
        records[3].description = "Changed obligation".into();
        assert!(!gate_current(&gate, &records, &config));
        records[3].description.clear();
        let mut next = Record::new(Kind::Story, "S2", "New member");
        next.layer = Some("L".into());
        records.push(next);
        assert!(!gate_current(&gate, &records, &config));
        records.pop();
        let mut failed = Record::new(Kind::Evidence, "E2", "Regression");
        failed.status = "fail".into();
        failed.refs = vec!["S".into()];
        records.push(failed);
        assert!(!gate_current(&gate, &records, &config));
    }
    #[test]
    fn cycles_and_missing_links_are_errors() {
        let mut a = Record::new(Kind::Story, "A", "A");
        let mut b = Record::new(Kind::Story, "B", "B");
        a.depends_on = vec!["B".into(), "missing".into()];
        b.depends_on = vec!["A".into()];
        let d = validate(&[a, b], &Config::default());
        assert!(d.iter().any(|x| x.code == "dependency_cycle"));
        assert!(d.iter().any(|x| x.code == "missing_reference"));
    }
    #[test]
    fn completion_label_does_not_release_dependency() {
        let mut a = Record::new(Kind::Story, "A", "A");
        a.status = "integrated".into();
        let mut b = Record::new(Kind::Story, "B", "B");
        b.depends_on = vec!["A".into()];
        assert!(!readiness(&b, &[a.clone(), b.clone()], &Config::default()).ready);
        assert!(
            validate(&[a, b], &Config::default())
                .iter()
                .any(|d| d.code == "missing_delivery")
        );
    }
    #[test]
    fn paths_are_component_scoped() {
        assert!(path_overlap("apps/web", "apps/web/src"));
        assert!(!path_overlap("apps/web", "apps/webapp"));
        assert!(!valid_id("../outside"));
        assert!(path_overlap(".", "src/main.rs"));
        assert!(path_overlap("./src/", "src/main.rs"));
        assert!(normalize_scope("../outside").is_none());
    }
    #[test]
    fn parent_gate_tracks_child_and_policy() {
        let config = Config::default();
        let mut evidence = Record::new(Kind::Evidence, "E", "check");
        evidence.status = "pass".into();
        let mut child = Record::new(Kind::Gate, "G-child", "child");
        child.status = "pass".into();
        child.set("evidence", serde_json::json!(["E"]));
        child.set(
            "scope_digest",
            gate_scope(&child, &[evidence.clone()], &config),
        );
        let mut parent = Record::new(Kind::Gate, "G-parent", "parent");
        parent.status = "pass".into();
        parent.required_gates = vec![child.id.clone()];
        parent.set("evidence", serde_json::json!(["E"]));
        parent.set(
            "scope_digest",
            gate_scope(&parent, &[child.clone(), evidence.clone()], &config),
        );
        assert!(gate_current(
            &parent,
            &[child.clone(), evidence.clone()],
            &config
        ));
        child.status = "fail".into();
        assert!(!gate_current(&parent, &[child, evidence], &config));
    }
}
