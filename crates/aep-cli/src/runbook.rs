use crate::cli::Cli;
use crate::cli::Runbook;
use crate::commands::Outcome;
use crate::guidance;
use aep_core::{Error, Kind, Result};
use aep_store::{Snapshot, Store, git, parse_yaml};
use serde_json::{Value, json};
use std::collections::BTreeSet;

pub fn run(args: &Cli, s: &Store, snap: &Snapshot, command: &Runbook) -> Result<Outcome> {
    let Runbook::Check { file } = command;
    let text = std::fs::read_to_string(file)?;
    let progress: Value = parse_yaml(&text)?;
    let name = progress["runbook"]
        .as_str()
        .ok_or_else(|| Error::input("A progress file needs `runbook`"))?;
    let attempt = progress["attempt"]
        .as_str()
        .ok_or_else(|| Error::input("A progress file needs `attempt`"))?;
    let steps = guidance::runbook_steps_named(name)?;
    if !snap
        .records
        .iter()
        .any(|r| r.kind == Kind::Attempt && r.id == attempt)
    {
        return Err(Error::input(format!(
            "Attempt {attempt} is not in the store"
        )));
    }
    let recorded = progress["steps"]
        .as_object()
        .ok_or_else(|| Error::input("A progress file needs a steps mapping"))?;
    for id in recorded.keys() {
        if !steps.iter().any(|step| step["id"] == id.as_str()) {
            return Err(Error::input(format!("Step {id} is not in runbook {name}")));
        }
    }
    let reads = principle_reads(&args.root, attempt)?;
    let mut report = vec![];
    let mut previous = None;
    for step in &steps {
        let id = step["id"].as_str().unwrap_or("");
        let entry = recorded
            .get(id)
            .ok_or_else(|| refuse(name, id, "is missing from the progress file"))?;
        match entry["status"].as_str() {
            Some("skipped") => {
                if entry["reason"].as_str().is_none_or(|r| r.trim().is_empty()) {
                    return Err(refuse(name, id, "is skipped without a reason"));
                }
                report.push(json!({"step": id, "status": "skipped"}));
            }
            Some("done") => {
                let at = entry["at"]
                    .as_str()
                    .and_then(|at| chrono::DateTime::parse_from_rfc3339(at).ok())
                    .ok_or_else(|| refuse(name, id, "needs an RFC3339 at time"))?;
                if previous.is_some_and(|prior| at < prior) {
                    return Err(refuse(name, id, "precedes the previous completed step"));
                }
                previous = Some(at);
                check_done(name, step, entry, snap, s, &reads)?;
                report.push(json!({"step": id, "status": "done"}));
            }
            _ => return Err(refuse(name, id, "needs status done or skipped")),
        }
    }
    Ok(Outcome::ok(json!({
        "runbook": name,
        "attempt": attempt,
        "pass": true,
        "steps": report,
    })))
}

fn refuse(name: &str, step: &str, problem: &str) -> Error {
    Error::input(format!("Runbook {name}, step {step} {problem}"))
}

/// Checks one done step: its time, its evidence, its principles and their read order.
fn check_done(
    name: &str,
    step: &Value,
    entry: &Value,
    snap: &Snapshot,
    s: &Store,
    reads: &[(String, String)],
) -> Result<()> {
    let id = step["id"].as_str().unwrap_or("");
    let at = entry["at"]
        .as_str()
        .ok_or_else(|| refuse(name, id, "is done without an at time"))?;
    let declared: Vec<&str> = step["evidence"]
        .as_array()
        .map(|kinds| kinds.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();
    let given = entry["evidence"].as_object();
    for kind in &declared {
        let value = given
            .and_then(|map| map.get(*kind))
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|v| !v.is_empty())
            .ok_or_else(|| refuse(name, id, &format!("is done without {kind} evidence")))?;
        match *kind {
            "check-id" => {
                if !s.config.checks.iter().any(|c| c.id == value) {
                    return Err(refuse(
                        name,
                        id,
                        &format!("names no configured check: {value}"),
                    ));
                }
            }
            "record-id" if snap.get(value).is_err() => {
                return Err(refuse(name, id, &format!("names no record: {value}")));
            }
            _ => {}
        }
    }
    if let Some(map) = given {
        for key in map.keys() {
            if !declared.contains(&key.as_str()) {
                return Err(refuse(
                    name,
                    id,
                    &format!("has evidence {key} the step does not declare"),
                ));
            }
        }
    }
    let cited: BTreeSet<&str> = step["principles"]
        .as_array()
        .map(|list| list.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();
    let read: BTreeSet<&str> = entry["principles_read"]
        .as_array()
        .map(|list| list.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();
    if read != cited {
        return Err(refuse(
            name,
            id,
            "lists principles_read that differ from its principles",
        ));
    }
    for principle in &cited {
        let read_before = reads.iter().any(|(reference, when)| {
            reference == principle
                && chrono::DateTime::parse_from_rfc3339(when)
                    .ok()
                    .zip(chrono::DateTime::parse_from_rfc3339(at).ok())
                    .is_some_and(|(read, done)| read <= done)
        });
        if !read_before {
            return Err(refuse(
                name,
                id,
                &format!("cites {principle}, which was not read in the attempt before its at time"),
            ));
        }
    }
    Ok(())
}

/// The principle reads the attempt recorded in the read log: (reference, at).
fn principle_reads(root: &std::path::Path, attempt: &str) -> Result<Vec<(String, String)>> {
    let common = git(
        root,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )?;
    let path = std::path::Path::new(&common)
        .join("aep")
        .join(guidance::READ_LOG);
    let Ok(text) = std::fs::read_to_string(path) else {
        return Ok(vec![]);
    };
    let mut reads = vec![];
    for line in text.lines() {
        let Ok(value) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        if value["source"] != "agent"
            || value["procedure"] != "principles"
            || value["attempt"] != attempt
            || value["guidance_digest"] != guidance::bundle_digest()
        {
            continue;
        }
        let reference = value["reference"]
            .as_str()
            .unwrap_or("")
            .trim_start_matches("references/")
            .trim_end_matches(".md")
            .to_string();
        let at = value["at"].as_str().unwrap_or("").to_string();
        reads.push((reference, at));
    }
    Ok(reads)
}
