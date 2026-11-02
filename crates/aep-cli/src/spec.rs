use crate::{
    cli::{Cli, Spec},
    commands::{Outcome, save, subject},
    workflow,
};
use aep_core::{Error, Kind, Record, Result, digest, valid_id};
use aep_store::{Snapshot, Store, contained};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Contract {
    pub capability: String,
    pub path: String,
    pub baseline: Option<String>,
}
#[derive(Debug, PartialEq)]
enum Mode {
    Add,
    Modify,
    Remove,
    Rename,
}
#[derive(Debug)]
struct Delta {
    mode: Mode,
    name: String,
    text: String,
    to: Option<String>,
}
struct RequirementDocument {
    prefix: String,
    blocks: Vec<(String, String)>,
    suffix: String,
}
fn visible_lines(text: &str) -> Vec<(usize, &str)> {
    let mut result = vec![];
    let mut fence: Option<(char, usize)> = None;
    let mut offset = 0;
    for line in text.split_inclusive('\n') {
        let trimmed = line.trim();
        let marker = trimmed.chars().next().filter(|c| *c == '`' || *c == '~');
        let length = marker.map_or(0, |c| trimmed.chars().take_while(|x| *x == c).count());
        if let Some((c, n)) = fence {
            if marker == Some(c) && length >= n && trimmed[length..].trim().is_empty() {
                fence = None;
            }
        } else if length >= 3 {
            fence = Some((marker.unwrap(), length));
        } else {
            result.push((offset, line.trim_end_matches(['\n', '\r'])));
        }
        offset += line.len();
    }
    result
}
fn document(text: &str) -> Result<RequirementDocument> {
    let lines = visible_lines(text);
    let mut starts = vec![];
    let mut suffix = text.len();
    let mut ended = false;
    let mut names = BTreeSet::new();
    for (offset, line) in lines {
        if let Some(name) = line.strip_prefix("### Requirement: ") {
            if ended {
                return Err(Error::unsupported(
                    "Requirements in multiple sections are unsupported",
                ));
            }
            let name = name.trim();
            if name.is_empty() || !names.insert(name.to_string()) {
                return Err(Error::input("Empty or duplicate requirement name"));
            }
            starts.push((offset, name.to_string()));
        } else if !starts.is_empty()
            && (line.starts_with("## ") || line.starts_with("# "))
            && !ended
        {
            suffix = offset;
            ended = true;
        }
    }
    let prefix_end = starts.first().map_or(text.len(), |(i, _)| *i);
    let blocks = starts
        .iter()
        .enumerate()
        .map(|(index, (start, name))| {
            let end = starts.get(index + 1).map_or(suffix, |(i, _)| *i);
            (
                name.clone(),
                text[*start..end].trim_end().to_string() + "\n",
            )
        })
        .collect();
    Ok(RequirementDocument {
        prefix: text[..prefix_end].into(),
        blocks,
        suffix: text[suffix..].into(),
    })
}
fn requirements(text: &str) -> Result<BTreeMap<String, String>> {
    Ok(document(text)?.blocks.into_iter().collect())
}
fn scenario_names(text: &str) -> Result<BTreeSet<String>> {
    let mut names = BTreeSet::new();
    let mut current: Option<String> = None;
    let mut steps = BTreeSet::new();
    fn finish(name: &Option<String>, steps: &BTreeSet<String>) -> Result<()> {
        if name.is_some() && (!steps.contains("WHEN") || !steps.contains("THEN")) {
            return Err(Error::input(
                "Each scenario requires explicit WHEN and THEN steps; add GIVEN when setup is relevant",
            ));
        }
        Ok(())
    }
    for (_, line) in visible_lines(text) {
        if let Some(name) = line.strip_prefix("#### Scenario: ") {
            finish(&current, &steps)?;
            steps.clear();
            let name = name.trim().to_string();
            if name.is_empty() || !names.insert(name.clone()) {
                return Err(Error::input("Empty or duplicate scenario identity"));
            }
            current = Some(name);
        } else if line.starts_with('#') {
            finish(&current, &steps)?;
            current = None;
            steps.clear();
        } else if current.is_some() {
            let step = line
                .trim()
                .trim_start_matches("- ")
                .trim_start_matches("* ")
                .replace("**", "");
            if let Some(word) = step.split_whitespace().next()
                && ["GIVEN", "WHEN", "THEN", "AND"].contains(&word.to_uppercase().as_str())
                && step.split_whitespace().count() > 1
            {
                steps.insert(word.to_uppercase());
            }
        }
    }
    finish(&current, &steps)?;
    if names.is_empty() {
        return Err(Error::input("Requirement has no unfenced BDD scenario"));
    }
    Ok(names)
}
fn scenarios(text: &str) -> Result<()> {
    let requirement = visible_lines(text)
        .into_iter()
        .skip(1)
        .take_while(|(_, line)| !line.starts_with("#### Scenario: "))
        .map(|(_, line)| line)
        .collect::<Vec<_>>()
        .join("\n");
    if !requirement
        .split(|c: char| !c.is_alphabetic())
        .any(|word| ["SHALL", "MUST"].contains(&word))
    {
        return Err(Error::input(
            "Requirement body needs an explicit SHALL or MUST obligation",
        ));
    }
    scenario_names(text).map(|_| ())
}
fn parse_delta(text: &str) -> Result<Vec<Delta>> {
    let mut sections: Vec<(Mode, String)> = vec![];
    let visible: BTreeSet<usize> = visible_lines(text)
        .into_iter()
        .map(|(offset, _)| offset)
        .collect();
    let mut offset = 0;
    for raw in text.split_inclusive('\n') {
        let line = raw.trim_end_matches(['\n', '\r']);
        let exposed = visible.contains(&offset);
        offset += raw.len();
        let mode = match line.trim() {
            "## ADDED Requirements" => Some(Mode::Add),
            "## MODIFIED Requirements" => Some(Mode::Modify),
            "## REMOVED Requirements" => Some(Mode::Remove),
            "## RENAMED Requirements" => Some(Mode::Rename),
            _ => None,
        };
        if let Some(mode) = mode.filter(|_| exposed) {
            sections.push((mode, String::new()));
        } else if exposed && line.starts_with("## ") {
            return Err(Error::unsupported(format!(
                "Unsupported delta section: {line}"
            )));
        } else if let Some((_, body)) = sections.last_mut() {
            body.push_str(line);
            body.push('\n');
        } else if !line.trim().is_empty() && !line.starts_with("# ") {
            return Err(Error::unsupported(
                "Delta content outside a supported section",
            ));
        }
    }
    let mut result = vec![];
    let mut names = BTreeSet::new();
    for (mode, body) in sections {
        if mode == Mode::Rename {
            let mut from = None;
            for line in body.lines().map(str::trim).filter(|l| !l.is_empty()) {
                if let Some(name) = line.strip_prefix("- FROM: ") {
                    if from.is_some() {
                        return Err(Error::input("Rename FROM has no TO"));
                    }
                    from = Some(
                        name.trim_matches('`')
                            .trim_start_matches("### Requirement: ")
                            .to_string(),
                    );
                } else if let Some(name) = line.strip_prefix("- TO: ") {
                    let old = from
                        .take()
                        .ok_or_else(|| Error::input("Rename TO has no FROM"))?;
                    let new = name
                        .trim_matches('`')
                        .trim_start_matches("### Requirement: ")
                        .to_string();
                    if old.is_empty() || new.is_empty() || !names.insert(old.clone()) {
                        return Err(Error::input("Empty or duplicate rename"));
                    }
                    result.push(Delta {
                        mode: Mode::Rename,
                        name: old,
                        text: String::new(),
                        to: Some(new),
                    });
                } else {
                    return Err(Error::unsupported(format!(
                        "Unsupported rename content: {line}"
                    )));
                }
            }
            if from.is_some() {
                return Err(Error::input("Rename FROM has no TO"));
            }
        } else {
            let reqs = requirements(&body)?;
            if reqs.is_empty() {
                return Err(Error::input("Delta section has no requirements"));
            }
            for (name, text) in reqs {
                if !names.insert(name.clone()) {
                    return Err(Error::input(format!("Conflicting deltas for {name}")));
                }
                if mode == Mode::Add || mode == Mode::Modify {
                    scenarios(&text)?;
                }
                result.push(Delta {
                    mode: match mode {
                        Mode::Add => Mode::Add,
                        Mode::Modify => Mode::Modify,
                        _ => Mode::Remove,
                    },
                    name,
                    text,
                    to: None,
                });
            }
        }
    }
    if result.is_empty() {
        return Err(Error::input("No supported specification deltas"));
    }
    Ok(result)
}
fn apply_delta(baseline: Option<&str>, delta: &str) -> Result<String> {
    let mut doc = document(baseline.unwrap_or("# Requirements\n\n"))?;
    let mut deltas = parse_delta(delta)?;
    deltas.sort_by_key(|d| match d.mode {
        Mode::Rename => 0,
        Mode::Remove => 1,
        Mode::Modify => 2,
        Mode::Add => 3,
    });
    let original = doc.blocks.clone();
    for delta in deltas {
        let index = doc.blocks.iter().position(|(name, _)| name == &delta.name);
        match delta.mode {
            Mode::Add => {
                if let Some(i) = index {
                    if doc.blocks[i].1.trim() != delta.text.trim() {
                        return Err(Error::conflict(format!(
                            "Requirement already exists with different content: {}",
                            delta.name
                        )));
                    }
                } else {
                    doc.blocks.push((delta.name, delta.text));
                }
            }
            Mode::Modify => {
                let i = index.ok_or_else(|| {
                    Error::conflict(format!("Modified requirement is absent: {}", delta.name))
                })?;
                let before = scenario_names(&doc.blocks[i].1)?;
                let after = scenario_names(&delta.text)?;
                if !before.is_subset(&after) {
                    return Err(Error::conflict(
                        "MODIFIED must preserve existing scenarios; make intentional removals explicit in the design",
                    ));
                }
                doc.blocks[i].1 = delta.text;
            }
            Mode::Remove => {
                if let Some(i) = index {
                    doc.blocks.remove(i);
                }
            }
            Mode::Rename => {
                let to = delta.to.unwrap();
                let target = doc.blocks.iter().position(|(n, _)| n == &to);
                match (index, target) {
                    (None, Some(_)) => {}
                    (Some(i), None) => {
                        doc.blocks[i].1 = doc.blocks[i].1.replacen(
                            &format!("### Requirement: {}", delta.name),
                            &format!("### Requirement: {to}"),
                            1,
                        );
                        doc.blocks[i].0 = to;
                    }
                    _ => return Err(Error::conflict("Rename source/target conflict")),
                }
            }
        }
    }
    if doc.blocks.is_empty() {
        return Err(Error::unsupported(
            "Retiring the final requirement needs an explicit capability retirement design; the common profile cannot publish an empty specification",
        ));
    }
    if doc.blocks == original {
        return Ok(baseline.unwrap_or("# Requirements\n\n").into());
    }
    let blocks = doc
        .blocks
        .iter()
        .map(|(_, b)| b.trim_end())
        .collect::<Vec<_>>()
        .join("\n\n");
    let separator = if doc.suffix.is_empty() { "\n" } else { "\n\n" };
    Ok(format!("{}{blocks}{separator}{}", doc.prefix, doc.suffix))
}
pub fn contracts(change: &Record) -> Result<Vec<Contract>> {
    serde_json::from_value(change.data.get("specs").cloned().unwrap_or(json!([])))
        .map_err(Error::from)
}
fn candidates(s: &Store, snap: &Snapshot, change: &Record) -> Result<BTreeMap<String, String>> {
    if change
        .data
        .get("unmapped_artifacts")
        .and_then(Value::as_array)
        .is_some_and(|a| !a.is_empty())
    {
        return Err(Error::unsupported(
            "Change has unmapped custom artifacts; resolve them before acceptance/publication",
        ));
    }
    let mut output = BTreeMap::new();
    for contract in contracts(change)? {
        if !valid_id(&contract.capability) {
            return Err(Error::input("Invalid capability ID"));
        }
        let target = format!(
            "{}/specs/{}/spec.md",
            s.config.stores.roadmap, contract.capability
        );
        if output.contains_key(&target) {
            return Err(Error::input("Duplicate capability delta"));
        }
        let baseline = snap.files.get(&target);
        if baseline.map(digest) != contract.baseline {
            return Err(Error::conflict(format!(
                "Specification baseline changed: {}",
                contract.capability
            )));
        }
        contained(&s.root, &contract.path)?;
        let delta = snap
            .files
            .get(&contract.path)
            .ok_or_else(|| Error::input("Delta must be in a configured context store"))?;
        let initial = format!(
            "# {} Specification\n\n## Purpose\n{}\n\n## Requirements\n\n",
            contract.capability,
            change.description.trim()
        );
        output.insert(
            target,
            apply_delta(
                Some(baseline.map_or(initial.as_str(), String::as_str)),
                delta,
            )?,
        );
    }
    Ok(output)
}
pub fn validate_change(s: &Store, snap: &Snapshot, change: &Record) -> Result<()> {
    let result = candidates(s, snap, change)?;
    if result.is_empty() && change.data.get("documentation_only") != Some(&Value::Bool(true)) {
        return Err(Error::blocked(
            "Behavior changes require BDD deltas; documentation-only work must declare that scope",
        ));
    }
    Ok(())
}
pub fn check_all(s: &Store, snap: &Snapshot) -> Result<Vec<Value>> {
    let mut out = vec![];
    for r in snap
        .records
        .iter()
        .filter(|r| r.kind == Kind::Change && r.status == "accepted")
    {
        if let Err(e) = validate_change(s, snap, r) {
            out.push(json!({"code":e.code,"record":r.id,"message":e.message}));
        }
    }
    let prefix = format!("{}/specs/", s.config.stores.roadmap);
    for (path, text) in snap
        .files
        .iter()
        .filter(|(p, _)| p.starts_with(&prefix) && p.ends_with("/spec.md"))
    {
        match requirements(text) {
            Ok(reqs) => {
                if reqs.is_empty() {
                    out.push(json!({"code":"spec","path":path,"message":"Formal specification has no requirements"}));
                }
                for (name, text) in reqs {
                    if let Err(e) = scenarios(&text) {
                        out.push(json!({"code":"bdd","path":path,"requirement":name,"message":e.message}));
                    }
                }
            }
            Err(e) => out.push(json!({"code":"spec","path":path,"message":e.message})),
        }
    }
    Ok(out)
}
pub fn run(args: &Cli, s: &Store, snap: &Snapshot, command: &Spec) -> Result<Outcome> {
    match command {
        Spec::Check { change } => {
            let diagnostics = if let Some(id) = change {
                let r = subject(snap, id, Kind::Change)?;
                match validate_change(s, snap, &r) {
                    Ok(()) => vec![],
                    Err(e) => vec![json!(e)],
                }
            } else {
                check_all(s, snap)?
            };
            let mut out =
                Outcome::ok(json!({"pass":diagnostics.is_empty(),"diagnostics":diagnostics}));
            if !diagnostics.is_empty() {
                out.exit_code = 1;
                out.diagnostics = diagnostics;
            }
            Ok(out)
        }
        Spec::Diff { change } => {
            let r = subject(snap, change, Kind::Change)?;
            let after = candidates(s, snap, &r)?;
            Ok(Outcome::ok(
                json!({"change":change,"files":after.iter().map(|(p,a)|json!({"path":p,"before":snap.files.get(p),"after":a,"digest":digest(a)})).collect::<Vec<_>>()}),
            ))
        }
        Spec::Publish { change } => {
            let mut r = subject(snap, change, Kind::Change)?;
            if r.status != "accepted" {
                return Err(Error::blocked(
                    "Publish requires an accepted, unpublished change",
                ));
            }
            let stories: Vec<_> = snap
                .records
                .iter()
                .filter(|r| r.kind == Kind::Story && r.change_ids().contains(&change.as_str()))
                .collect();
            if stories.is_empty()
                || stories
                    .iter()
                    .any(|r| !["integrated", "released"].contains(&r.status.as_str()))
            {
                return Err(Error::blocked(
                    "All linked stories must be integrated before publication",
                ));
            }
            let mut receipts = vec![];
            for story in stories {
                let delivery = story
                    .text("delivery")
                    .and_then(|id| snap.get(id).ok())
                    .ok_or_else(|| Error::blocked("Missing integration receipt"))?;
                let (plan, missing, evidence) = workflow::evidence_requirements(s, snap, story)?;
                if !missing.is_empty() {
                    return Err(Error::blocked(format!(
                        "Publication requires current verification for {}: {}. Run aep verify run --story {} and resolve required review before retrying publication.",
                        story.id,
                        missing.join("; "),
                        story.id
                    )));
                }
                // Integration is a historical fact; verification is a current-context
                // assertion. Recheck the exact Git trees rather than requiring a new
                // merge just because context arrived in the shared store afterward.
                let integration = delivery
                    .text("integration_head")
                    .ok_or_else(|| Error::blocked("Delivery has no integration revision"))?;
                let integrated_tree = aep_store::git(&s.root, &["rev-parse", "--verify", &format!("{integration}^{{tree}}")])
                    .map_err(|_| Error::blocked("Integration tree is unavailable; restore the recorded commit before publication"))?;
                let candidate_tree = aep_store::git(
                    &s.root,
                    &["rev-parse", "--verify", &format!("{}^{{tree}}", plan.head)],
                )?;
                if integrated_tree != candidate_tree {
                    return Err(Error::blocked(
                        "Integrated tree differs from the verified candidate; inspect and reopen the story for current validation",
                    ));
                }
                receipts.push(json!({"story":story.id,"delivery":delivery.id,"implementation_head":plan.head,"integration_head":integration,
                    "integration_fingerprint":delivery.text("fingerprint"),"verification_fingerprint":plan.fingerprint,
                    "verification_evidence":evidence,"producer":plan.producer,
                    "environment":"local/CI only; release evidence is separate"}));
            }
            let outputs = candidates(s, snap, &r)?;
            let mut writes: BTreeMap<String, Option<String>> = outputs
                .iter()
                .map(|(p, s)| (p.clone(), Some(s.clone())))
                .collect();
            for (p, content) in outputs {
                writes.insert(p.replace("/spec.md","/evidence.yaml"),Some(aep_store::yaml(&json!({"change":change,"spec_digest":digest(&content),"evidence":receipts}))?));
            }
            r.status = "published".into();
            r.set("publication_evidence", json!(receipts));
            save(s, snap, vec![r], writes, args.dry_run)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    const ADD: &str = "## ADDED Requirements\n### Requirement: A\nThe system SHALL work.\n#### Scenario: success\nGiven input\nWhen run\nThen result\n";
    #[test]
    fn delta_baseline_conflicts_and_bdd() {
        let first = apply_delta(None, ADD).unwrap();
        assert!(first.ends_with("Then result\n"));
        assert!(!first.ends_with("\n\n"));
        assert_eq!(apply_delta(Some(&first), ADD).unwrap(), first);
        let second = apply_delta(
            Some(&first),
            &ADD.replace("Requirement: A", "Requirement: B"),
        )
        .unwrap();
        assert!(second.contains("Then result\n\n### Requirement: B"));
        assert!(!second.ends_with("\n\n"));
        // A no-op delta preserves existing source bytes, including old formatting.
        let historical = format!("{first}\n");
        assert_eq!(apply_delta(Some(&historical), ADD).unwrap(), historical);
        assert!(parse_delta("## ADDED Requirements\n### Requirement: A\nNo scenario").is_err());
        let renamed = apply_delta(
            Some(&first),
            "## RENAMED Requirements\n- FROM: A\n- TO: B\n",
        )
        .unwrap();
        assert!(renamed.contains("### Requirement: B"));
        assert!(!renamed.contains("### Requirement: A"));
    }
    #[test]
    fn unknown_sections_are_not_guessed() {
        assert!(parse_delta("## CUSTOM Requirements\ntext").is_err());
    }
    #[test]
    fn rename_precedes_modify_and_preserves_document_context() {
        let first = apply_delta(None, ADD).unwrap();
        let baseline = first.replacen(
            "# Requirements",
            "# Result\n\n## Purpose\nRetain results.\n\n## Requirements",
            1,
        ) + "## Operational notes\nKeep this context.\n";
        let delta = ADD
            .replace("ADDED", "MODIFIED")
            .replace("Requirement: A", "Requirement: B")
            .replace("SHALL work", "SHALL complete work")
            + "\n## RENAMED Requirements\n- FROM: A\n- TO: B\n";
        let result = apply_delta(Some(&baseline), &delta).unwrap();
        assert!(result.starts_with("# Result\n\n## Purpose\nRetain results."));
        assert!(result.ends_with("## Operational notes\nKeep this context.\n"));
        assert!(result.contains("### Requirement: B\nThe system SHALL complete work."));
        assert!(!result.contains("### Requirement: A"));
    }
    #[test]
    fn prose_and_fenced_examples_cannot_satisfy_bdd() {
        let prose = "## ADDED Requirements\n### Requirement: A\nThe system SHALL work.\n#### Scenario: example\nExplain when and then in prose.\n";
        assert!(parse_delta(prose).is_err());
        assert!(
            parse_delta(&(prose.to_string() + "```gherkin\nWhen run\nThen result\n```\n")).is_err()
        );
        assert!(parse_delta(&format!("```markdown\n{ADD}```\n")).is_err());
        let baseline = apply_delta(None, ADD).unwrap();
        let removed = ADD
            .replace("ADDED", "MODIFIED")
            .replace("Scenario: success", "Scenario: replacement");
        assert!(apply_delta(Some(&baseline), &removed).is_err());
    }
}
