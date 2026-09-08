use crate::{
    cli::{Cli, Migrate},
    commands::{Outcome, initial_files, input, save},
    spec,
};
use aep_core::{Error, Kind, Record, Result, digest, valid_id};
use aep_store::{Store, contained, git, parse_yaml};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Plan {
    schema_version: u32,
    repository: PathBuf,
    base_commit: String,
    source_revision: String,
    sources: BTreeMap<String, String>,
    records: Vec<Record>,
    writes: BTreeMap<String, Option<String>>,
    diagnostics: Vec<String>,
}
fn text(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        _ => value.to_string(),
    }
}
fn list(value: &Value) -> Vec<Value> {
    value.as_array().cloned().unwrap_or_default()
}
fn source(s: &Store, path: &str, sources: &mut BTreeMap<String, String>) -> Result<Option<Value>> {
    let file = contained(&s.root, path)?;
    if !file.exists() {
        return Ok(None);
    }
    let bytes = std::fs::read_to_string(file)?;
    sources.insert(path.into(), digest(&bytes));
    Ok(Some(parse_yaml(&bytes)?))
}
fn insert_record(records: &mut BTreeMap<String, Record>, record: Record) -> Result<()> {
    if records.insert(record.id.clone(), record).is_some() {
        return Err(Error::conflict(
            "Migration ID collision across generated and imported records",
        ));
    }
    Ok(())
}
fn bind_tree(
    s: &Store,
    path: &str,
    sources: &mut BTreeMap<String, String>,
) -> Result<BTreeMap<String, String>> {
    fn walk(
        s: &Store,
        path: &str,
        sources: &mut BTreeMap<String, String>,
        files: &mut BTreeMap<String, String>,
    ) -> Result<()> {
        for entry in std::fs::read_dir(contained(&s.root, path)?)? {
            let entry = entry?;
            let rel = format!("{path}/{}", entry.file_name().to_string_lossy());
            let file = contained(&s.root, &rel)?;
            if entry.file_type()?.is_dir() {
                walk(s, &rel, sources, files)?;
            } else {
                let content = std::fs::read_to_string(file)?;
                sources.insert(rel.clone(), digest(&content));
                files.insert(rel, content);
            }
        }
        Ok(())
    }
    let mut files = BTreeMap::new();
    if s.root.join(path).exists() {
        walk(s, path, sources, &mut files)?;
    }
    Ok(files)
}
pub fn run(args: &Cli, command: &Migrate) -> Result<Outcome> {
    let s = match Store::open(&args.root) {
        Ok(s) => s,
        Err(e) if e.code == "prerequisite" => Store::uninitialized(&args.root)?,
        Err(e) => return Err(e),
    };
    let snap = s.snapshot()?;
    match command {
        Migrate::Plan {
            source: input_source,
            story,
            output,
        } => {
            let source_path = input_source
                .as_ref()
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or_else(|| "product-context.yaml".into());
            let mut sources = BTreeMap::new();
            let mut legacy = source(&s, &source_path, &mut sources)?
                .ok_or_else(|| Error::input(format!("No legacy source {source_path}")))?;
            let product =
                source(&s, "product/index.yaml", &mut sources)?.unwrap_or_else(|| legacy.clone());
            let mut records = BTreeMap::new();
            let mut diagnostics = vec![];
            let mut writes = initial_files(&s, &snap, s.root.join("CLAUDE.md").exists())?;
            let mut all = BTreeMap::new();
            for key in ["stories", "waves", "layer_gates"] {
                if legacy.get(key).is_some_and(|v| !v.is_array()) {
                    return Err(Error::input(format!("Legacy {key} must be an array")));
                }
            }
            for collection in ["stories", "waves", "layer_gates"] {
                if let Some(items) = legacy.get_mut(collection).and_then(Value::as_array_mut) {
                    for item in items {
                        for field in ["layer", "wave"] {
                            if let Some(value) = item.get_mut(field) {
                                if let Some(label) = value.as_str() {
                                    let number: serde_json::Number = serde_json::from_str(label).map_err(|_| Error::input(format!("Legacy {field} label {label} has no declared numeric ordering")))?;
                                    *value = Value::Number(number);
                                } else if !value.is_number() && !value.is_null() {
                                    return Err(Error::input(format!(
                                        "Legacy {field} must have numeric ordering"
                                    )));
                                }
                            }
                        }
                    }
                }
            }
            for old in list(&legacy["stories"]) {
                for key in ["dependencies", "files_affected", "paths"] {
                    if old.get(key).is_some_and(|v| !v.is_array()) {
                        return Err(Error::input(format!(
                            "Legacy story {}: {key} must be an array",
                            text(&old["id"])
                        )));
                    }
                }
            }
            for wave in list(&legacy["waves"]) {
                if !wave["stories"].is_array() {
                    return Err(Error::input("Legacy wave stories must be an array"));
                }
            }

            for old in list(&legacy["stories"]) {
                let id = text(&old["id"]);
                if !valid_id(&id) || all.insert(id.clone(), old).is_some() {
                    return Err(Error::input(format!(
                        "Invalid or duplicate legacy story ID {id}"
                    )));
                }
            }
            let mut selected: BTreeSet<String> = if story.is_empty() {
                all.iter()
                    .filter(|(_, r)| {
                        !["completed", "done", "cancelled", "archived"]
                            .contains(&text(&r["status"]).as_str())
                    })
                    .map(|(id, _)| id.clone())
                    .collect()
            } else {
                story.iter().cloned().collect()
            };
            let mut barriers: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
            let mut waves = list(&legacy["waves"]);
            waves.sort_by(|a, b| {
                text(&a["layer"]).cmp(&text(&b["layer"])).then_with(|| {
                    a["wave"]
                        .as_f64()
                        .partial_cmp(&b["wave"].as_f64())
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
            });
            let mut prior: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
            for wave in &waves {
                let layer = text(&wave["layer"]);
                let before = prior.entry(layer).or_default();
                let members: Vec<String> = list(&wave["stories"]).iter().map(text).collect();
                for id in &members {
                    barriers
                        .entry(id.clone())
                        .or_default()
                        .extend(before.clone());
                }
                before.extend(members);
            }
            let mut queue: Vec<_> = selected.iter().cloned().collect();
            while let Some(id) = queue.pop() {
                let old = all.get(&id).ok_or_else(|| {
                    Error::input(format!("Selected dependency/story is absent: {id}"))
                })?;
                let dependencies: BTreeSet<String> = list(&old["dependencies"])
                    .iter()
                    .map(text)
                    .chain(barriers.get(&id).into_iter().flatten().cloned())
                    .collect();
                for dep in dependencies {
                    if selected.insert(dep.clone()) {
                        queue.push(dep);
                    }
                }
                if let Some(layer) = old["layer"].as_f64() {
                    for gate in list(&legacy["layer_gates"]) {
                        if let Some(prior) = gate["layer"].as_f64().filter(|prior| *prior < layer) {
                            for (dep, candidate) in &all {
                                if candidate["layer"].as_f64() == Some(prior)
                                    && selected.insert(dep.clone())
                                {
                                    queue.push(dep.clone());
                                }
                            }
                        }
                    }
                }
            }
            let already: BTreeSet<String> = snap
                .records
                .iter()
                .filter(|r| {
                    r.kind == Kind::Story
                        && r.data.get("legacy_source").and_then(|v| v["path"].as_str())
                            == Some(source_path.as_str())
                })
                .map(|r| r.id.clone())
                .collect();
            if selected.is_subset(&already)
                && snap
                    .records
                    .iter()
                    .any(|r| r.kind == Kind::Import && r.text("migration") == Some("v4-context"))
            {
                return Ok(Outcome::ok(
                    json!({"already_migrated":true,"selected_stories":selected,"files":[]}),
                ));
            }
            for id in &selected {
                let old = &all[id];
                let status = text(&old["status"]);
                if ["in_progress", "running", "in_review", "dispatched"].contains(&status.as_str())
                {
                    diagnostics.push(format!(
                        "{id}: finish or explicitly checkpoint/restart the active legacy attempt"
                    ));
                }
                let mut r = Record::new(Kind::Story, id, old["title"].as_str().unwrap_or(id));
                r.description = text(&old["description"]);
                r.created_at = None;
                r.updated_at = None;
                r.depends_on = list(&old["dependencies"])
                    .iter()
                    .map(text)
                    .chain(barriers.get(id).into_iter().flatten().cloned())
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect();
                r.paths = list(
                    old.get("files_affected")
                        .or_else(|| old.get("paths"))
                        .unwrap_or(&Value::Null),
                )
                .iter()
                .filter_map(Value::as_str)
                .map(String::from)
                .collect();
                r.owner = old["owner"].as_str().map(String::from);
                r.set("legacy_status", status.clone());
                r.set("legacy_source", json!({"path":source_path,"record":id}));
                if ["completed", "done"].contains(&status.as_str()) {
                    r.status = "imported".into();
                    r.set("implementation_evidence","unknown; resolve against Git/provider evidence before using as a dependency");
                }
                if let Some(criteria) = old
                    .get("acceptance_criteria")
                    .or_else(|| old.get("acceptance_criteria_sketch"))
                {
                    r.set("legacy_acceptance", criteria.clone());
                }
                let layer = text(&old["layer"]);
                if !layer.is_empty() {
                    let key = format!("layer-{layer}");
                    if !valid_id(&key) {
                        return Err(Error::input(format!("Unsupported layer label {layer}")));
                    }
                    let mut container = Record::new(Kind::Layer, &key, &format!("Layer {layer}"));
                    container.created_at = None;
                    container.set("legacy_label", layer);
                    if records
                        .get(&key)
                        .is_some_and(|r: &Record| r.kind != Kind::Layer)
                    {
                        return Err(Error::conflict("Legacy layer/story ID collision"));
                    }
                    records.entry(key.clone()).or_insert(container);
                    r.layer = Some(key);
                }
                r.set("legacy_metadata", old.clone());
                insert_record(&mut records, r)?;
            }
            for old in list(&legacy["waves"]) {
                let members: Vec<_> = list(&old["stories"])
                    .iter()
                    .map(text)
                    .filter(|id| selected.contains(id))
                    .collect();
                if members.is_empty() {
                    continue;
                }
                let label = text(
                    old.get("wave")
                        .or_else(|| old.get("id"))
                        .unwrap_or(&Value::Null),
                );
                let layer = text(&old["layer"]);
                let id = format!("wave-{layer}-{label}");
                if !valid_id(&id) {
                    diagnostics.push("A selected wave needs an explicit ID mapping".into());
                    continue;
                }
                let mut wave = Record::new(Kind::Wave, &id, &format!("Wave {label}"));
                wave.created_at = None;
                wave.refs = members.clone();
                wave.set("legacy_metadata", old);
                for story in members {
                    if let Some(r) = records.get_mut(&story) {
                        if r.wave.is_some() {
                            diagnostics.push(format!(
                                "{story}: multiple legacy wave memberships need explicit mapping"
                            ));
                        }
                        r.wave = Some(id.clone());
                    }
                }
                insert_record(&mut records, wave)?;
            }
            for (n, old) in list(&legacy["layer_gates"]).into_iter().enumerate() {
                let layer = format!("layer-{}", text(&old["layer"]));
                if !records.contains_key(&layer) {
                    continue;
                }
                let mut gate = Record::new(
                    Kind::Gate,
                    &format!("gate-{layer}-{}", n + 1),
                    &format!("Imported gate occurrence {}", n + 1),
                );
                gate.created_at = None;
                gate.refs = vec![layer.clone()];
                gate.set("legacy_metadata", old.clone());
                gate.set("evidence_class", "imported_claim");
                // Historical gate occurrences retain scope but cannot certify a fresh attempt.
                let gate_id = gate.id.clone();
                let gate_layer = old["layer"].as_f64();
                for story in records.values_mut().filter(|r| r.kind == Kind::Story) {
                    let label = story
                        .data
                        .get("legacy_metadata")
                        .and_then(|v| v["layer"].as_f64());
                    if matches!((label, gate_layer), (Some(a), Some(b)) if a > b) {
                        story.required_gates.push(gate_id.clone());
                    }
                }
                insert_record(&mut records, gate)?;
            }
            let mut roadmap = Record::new(
                Kind::Roadmap,
                "imported-product-direction",
                "Imported product direction",
            );
            roadmap.created_at = None;
            roadmap.description="Current product direction imported for review; code and runtime evidence establish implementation.".into();
            let mut direction = product.clone();
            if let Some(object) = direction.as_object_mut() {
                for key in ["stories", "waves", "layer_gates"] {
                    object.remove(key);
                }
            }
            roadmap.set("product", direction);
            if let Some(architecture) = legacy.get("architecture") {
                roadmap.set("architecture", architecture.clone());
            }
            let mut mapped_sources = vec![];
            for (path, content) in bind_tree(&s, "product", &mut sources)? {
                if path == "product/index.yaml" || path.contains("/archive/") {
                    continue;
                }
                let target = path.replacen(
                    "product/",
                    &format!("{}/product/", s.config.stores.roadmap),
                    1,
                );
                mapped_sources.push(target.clone());
                writes.insert(target, Some(content));
            }
            roadmap.paths = mapped_sources;
            insert_record(&mut records, roadmap)?;
            let mut decisions = vec![];
            for value in [
                product.pointer("/product/decisions"),
                product.get("decisions"),
                legacy.pointer("/architecture/decisions"),
                legacy.pointer("/architecture/adrs"),
            ] {
                for item in value.into_iter().flat_map(list) {
                    if !decisions.contains(&item) {
                        decisions.push(item);
                    }
                }
            }
            for (n, old) in decisions.into_iter().enumerate() {
                let id = old["id"]
                    .as_str()
                    .map(String::from)
                    .unwrap_or_else(|| format!("decision-import-{}", n + 1));
                let mut r = Record::new(
                    Kind::Decision,
                    &id,
                    old["title"].as_str().unwrap_or("Imported decision"),
                );
                r.created_at = None;
                r.description = text(&old);
                r.set("legacy_source", json!({"path":source_path,"index":n}));
                insert_record(&mut records, r)?;
            }
            for (path, content) in bind_tree(&s, "project-convention", &mut sources)? {
                let target = path.replacen(
                    "project-convention/",
                    &format!("{}/", s.config.stores.rules),
                    1,
                );
                if snap.files.get(&target).is_some_and(|old| old != &content) {
                    diagnostics.push(format!("Rule target already differs: {target}"));
                }
                writes.insert(
                    target,
                    Some(content.replace(
                        "project-convention/",
                        &format!("{}/", s.config.stores.rules),
                    )),
                );
                writes.insert(path, None);
            }
            // Keep relative note/evidence links intact. Legacy prose is searchable
            // through lesson find; new observations use typed lesson records.
            for (path, content) in bind_tree(&s, "lessons-learned", &mut sources)? {
                let target = path.replacen(
                    "lessons-learned/",
                    &format!("{}/", s.config.stores.lessons),
                    1,
                );
                if snap.files.get(&target).is_some_and(|old| old != &content) {
                    return Err(Error::conflict("Imported lesson path already differs"));
                }
                if s.config.stores.lessons.contains('/')
                    || target
                        .strip_prefix(&format!("{}/observations/", s.config.stores.lessons))
                        .is_some_and(|rest| rest.ends_with(".md") && !rest.contains('/'))
                {
                    diagnostics.push(format!("{path}: map legacy lesson links and reserved observation paths explicitly for this store layout"));
                }
                writes.insert(target, Some(content));
                writes.insert(path, None);
            }
            // The new entrypoint becomes the writer route. Preserve project-specific old instructions as an explicitly indexed source for review.
            if let Some(old) = snap
                .files
                .get("AGENTS.md")
                .filter(|text| !text.contains("<!-- aep-cli-entrypoint: 5.0 -->"))
            {
                sources.insert("AGENTS.md".into(), digest(old));
                let path = format!("{}/legacy-entrypoint.md", s.config.stores.rules);
                writes.insert(path, Some(format!("# Imported project instructions\n\nHistorical AEP commands are superseded by `aep skills`. Review project-specific constraints below before implementation.\n\n{}", old.replace("project-convention/", &format!("{}/", s.config.stores.rules)))));
                writes.insert(
                    "AGENTS.md".into(),
                    Some(crate::guidance::asset("templates/AGENTS.md")?),
                );
                let index = format!("{}/README.md", s.config.stores.rules);
                let mut body = writes
                    .get(&index)
                    .and_then(Option::as_ref)
                    .or_else(|| snap.files.get(&index))
                    .cloned()
                    .unwrap_or_default();
                body.push_str("\nRead [legacy-entrypoint.md](legacy-entrypoint.md) for imported project-specific constraints. AEP 5.0 owns workflow state.\n");
                writes.insert(index, Some(body));
            }
            for key in [
                "operational_truth",
                "governance",
                "operational-truth",
                "preflight",
                "project_memory",
                "topology",
                "release_gates",
                "execution_slices",
                "releases",
                "routing",
                "max_concurrent_agents",
            ] {
                if legacy.get(key).is_some() {
                    diagnostics.push(format!("Project-specific {key} must be mapped to active rules/checks before cutover"));
                }
            }
            if s.root.join("openspec").exists() {
                let imported = spec::import_bundle(&s, &snap, &s.root.join("openspec"))?;
                for r in imported.0 {
                    insert_record(&mut records, r)?;
                }
                writes.extend(imported.1);
                // The importer writes a source digest into its receipt; bind all copied files in this plan too.
                fn bind(
                    root: &Path,
                    path: &Path,
                    out: &mut BTreeMap<String, String>,
                ) -> Result<()> {
                    for entry in std::fs::read_dir(path)? {
                        let entry = entry?;
                        let rel = entry
                            .path()
                            .strip_prefix(root)
                            .unwrap()
                            .to_string_lossy()
                            .to_string();
                        let path = contained(root, &rel)?;
                        if entry.file_type()?.is_dir() {
                            bind(root, &path, out)?;
                        } else {
                            out.insert(rel, digest(std::fs::read(path)?));
                        }
                    }
                    Ok(())
                }
                bind(&s.root, &s.root.join("openspec"), &mut sources)?;
            }
            // Link only explicit legacy change references; semantic matches belong to the agent.
            let change_ids: BTreeSet<_> = records
                .values()
                .chain(snap.records.iter())
                .filter(|r| r.kind == Kind::Change)
                .map(|r| r.id.clone())
                .collect();
            for id in &selected {
                let old = &all[id];
                if let Some(change) = old
                    .get("change_id")
                    .or_else(|| old.get("openspec_change"))
                    .and_then(Value::as_str)
                {
                    if change_ids.contains(change) {
                        records.get_mut(id).unwrap().change = Some(change.into());
                    } else {
                        diagnostics.push(format!("{id}: referenced change {change} is absent"));
                    }
                }
            }
            let base_commit = git(&s.root, &["rev-parse", "HEAD"])?;
            let mut receipt = Record::new(
                Kind::Import,
                &format!(
                    "migration-{}",
                    &digest(format!("{base_commit}:{source_path}:{selected:?}"))[..16]
                ),
                "Current context migration",
            );
            receipt.set("migration", "v4-context");
            receipt.set("source_commit", base_commit.clone());
            receipt.set("source_paths", json!(sources.keys().collect::<Vec<_>>()));
            receipt.set("source_hashes", json!(sources));
            for (path, hash) in &sources {
                if aep_store::git_bytes(&s.root, &["show", &format!("{base_commit}:{path}")])
                    .map(digest)
                    .as_ref()
                    .ok()
                    != Some(hash)
                {
                    diagnostics.push(format!(
                        "Commit the exact migration source before cutover: {path}"
                    ));
                }
            }
            receipt.set("selected_stories", json!(selected));
            receipt.set(
                "history",
                "Retrieve older records from the source Git commit/path",
            );
            insert_record(&mut records, receipt)?;
            for (id, record) in &records {
                if let Ok(existing) = snap.get(id) {
                    let compatible_import = existing.kind == record.kind
                        && (already.contains(id)
                            || (existing.kind != Kind::Story && existing.created_at.is_none()));
                    if !compatible_import {
                        return Err(Error::conflict(format!(
                            "Existing native record collides with migration ID: {id}"
                        )));
                    }
                }
            }
            records.retain(|id, _| snap.get(id).is_err());
            for record in records.values_mut().filter(|r| r.created_at.is_none()) {
                record.updated_at = None;
            }
            let plan = Plan {
                schema_version: 1,
                repository: s.root.clone(),
                base_commit,
                source_revision: snap.revision,
                sources,
                records: records.into_values().collect(),
                writes,
                diagnostics,
            };
            if let Some(output) = output
                && !args.dry_run
            {
                let mut file = std::fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(output)?;
                file.write_all(serde_json::to_string_pretty(&plan)?.as_bytes())?;
                file.sync_all()?;
            }
            let mut out = Outcome::ok(json!({"plan":plan,"output":output}));
            out.changed = output.is_some() && !args.dry_run;
            Ok(out)
        }
        Migrate::Apply { plan } => {
            let plan: Plan = input(plan)?;
            if plan.schema_version != 1 || plan.repository != s.root {
                return Err(Error::input(
                    "Migration plan belongs to another project/schema",
                ));
            }
            let receipt_ids: Vec<_> = plan
                .records
                .iter()
                .filter(|r| r.kind == Kind::Import && r.text("migration") == Some("v4-context"))
                .map(|r| &r.id)
                .collect();
            if !receipt_ids.is_empty()
                && receipt_ids
                    .iter()
                    .all(|id| snap.records.iter().any(|r| &r.id == *id))
            {
                return Ok(Outcome::ok(json!({"already_migrated":true,"files":[]})));
            }
            if !plan.diagnostics.is_empty() {
                return Err(Error::blocked(plan.diagnostics.join("; ")));
            }
            if plan.source_revision != snap.revision
                || plan.base_commit != git(&s.root, &["rev-parse", "HEAD"])?
            {
                return Err(Error::conflict(
                    "Migration base changed; generate a new plan",
                ));
            }
            for (path, hash) in &plan.sources {
                if digest(std::fs::read(contained(&s.root, path)?)?) != *hash {
                    return Err(Error::conflict(format!("Migration source changed: {path}")));
                }
            }
            let mut checked = snap.clone();
            for path in plan.sources.keys() {
                checked.files.insert(
                    path.clone(),
                    std::fs::read_to_string(contained(&s.root, path)?)?,
                );
            }
            save(&s, &checked, plan.records, plan.writes, args.dry_run)
        }
        Migrate::Verify => {
            let mut out = crate::commands::check(&s, &snap)?;
            let receipts: Vec<_> = snap
                .records
                .iter()
                .filter(|r| r.kind == Kind::Import && r.text("migration") == Some("v4-context"))
                .collect();
            if receipts.is_empty() {
                return Err(Error::blocked("No migration receipt exists"));
            }
            for receipt in &receipts {
                let base = receipt
                    .text("source_commit")
                    .ok_or_else(|| Error::input("Migration receipt has no Git base"))?;
                git(&s.root, &["cat-file", "-e", &format!("{base}^{{commit}}")])?;
                let hashes: BTreeMap<String, String> = serde_json::from_value(
                    receipt
                        .data
                        .get("source_hashes")
                        .cloned()
                        .ok_or_else(|| Error::input("Migration receipt lacks source hashes"))?,
                )?;
                for (path, hash) in hashes {
                    if digest(aep_store::git_bytes(
                        &s.root,
                        &["show", &format!("{base}:{path}")],
                    )?) != hash
                    {
                        return Err(Error::conflict("Migration Git source digest differs"));
                    }
                }
                for id in list(&receipt.data["selected_stories"]).iter().map(text) {
                    if snap.get(&id)?.kind != Kind::Story {
                        return Err(Error::input("Migrated story was lost"));
                    }
                }
                if !snap
                    .files
                    .get("AGENTS.md")
                    .is_some_and(|s| s.contains("aep-cli-entrypoint: 5.0"))
                    || !snap
                        .files
                        .contains_key(&format!("{}/README.md", s.config.stores.rules))
                {
                    return Err(Error::blocked(
                        "Migration entrypoint or project rules index is absent",
                    ));
                }
            }
            out.data["migration_receipts"] = json!(receipts);
            out.data["context_review_required"] = json!(
                "Agent reviews current intent, rule applicability, and unresolved legacy evidence"
            );
            Ok(out)
        }
    }
}
