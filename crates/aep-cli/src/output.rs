//! Human presentation only. JSON responses retain their complete domain data.
use crate::commands::Outcome;
use serde_json::Value;
use std::{collections::BTreeMap, fmt::Write};

fn text(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => "—".into(),
        _ => v.to_string(),
    }
}
fn label(key: &str) -> String {
    let s = key.replace('_', " ");
    let mut chars = s.chars();
    chars
        .next()
        .map(|c| c.to_uppercase().to_string() + chars.as_str())
        .unwrap_or_default()
}
fn field(out: &mut String, key: &str, value: &Value) {
    if !value.is_null() {
        let _ = writeln!(out, "  {}: {}", label(key), text(value));
    }
}
fn strings(out: &mut String, heading: &str, values: &Value) {
    if let Some(values) = values.as_array().filter(|v| !v.is_empty()) {
        let _ = writeln!(out, "\n{heading}:");
        for v in values {
            let _ = writeln!(out, "  - {}", text(v));
        }
    }
}
fn diagnostics(out: &mut String, values: &Value) {
    if let Some(values) = values.as_array().filter(|v| !v.is_empty()) {
        out.push_str("\nIssues:\n");
        for v in values {
            let message = v.get("message").map(text).unwrap_or_else(|| text(v));
            let prefix = v
                .get("record")
                .or_else(|| v.get("path"))
                .map(|v| format!("{}: ", text(v)))
                .unwrap_or_default();
            let _ = writeln!(out, "  - {prefix}{message}");
        }
    }
}
fn record(v: &Value) -> &Value {
    v.get("record").unwrap_or(v)
}
fn record_list(out: &mut String, values: &[Value], limit: usize) {
    if values.is_empty() {
        out.push_str("  No records.\n");
        return;
    }
    for value in values.iter().take(limit) {
        let r = record(value);
        if r["id"].is_null() {
            render_value(out, "result", r, 1);
        } else {
            let _ = writeln!(
                out,
                "  {}  [{}]  {}",
                text(&r["id"]),
                text(&r["status"]),
                r["title"].as_str().unwrap_or("")
            );
        }
    }
    if values.len() > limit {
        let _ = writeln!(
            out,
            "  … showing {limit} of {}. Use --json for every record.",
            values.len()
        );
    }
}
fn record_detail(out: &mut String, value: &Value) {
    let r = record(value);
    let _ = writeln!(out, "{} — {}\n", text(&r["id"]), text(&r["title"]));
    for k in [
        "kind", "status", "owner", "risk", "layer", "wave", "change", "release",
    ] {
        field(out, k, &r[k]);
    }
    if let Some(description) = r["description"].as_str().filter(|s| !s.is_empty()) {
        let _ = writeln!(out, "\n{description}");
    }
    strings(out, "Dependencies", &r["depends_on"]);
    strings(out, "Scope", &r["paths"]);
    strings(out, "Checks", &r["required_checks"]);
    strings(out, "Gates", &r["required_gates"]);
    strings(out, "References", &r["refs"]);
    for key in ["blockers", "missing", "findings"] {
        if let Some(v) = r.get("data").and_then(|d| d.get(key)) {
            render_value(out, key, v, 0);
        }
    }
    if let Some(revision) = value.get("revision") {
        field(out, "revision", revision);
    }
    out.push_str("\nUse --json for complete metadata and provenance.\n");
}
/// Bounded structural fallback for less common operations. Never emit raw JSON as the default UI.
fn render_value(out: &mut String, key: &str, v: &Value, depth: usize) {
    let pad = "  ".repeat(depth);
    match v {
        Value::Null => {}
        Value::Object(map) => {
            let _ = writeln!(out, "{pad}{}:", label(key));
            if depth >= 5 {
                out.push_str(&format!("{pad}  Details available with --json.\n"));
                return;
            }
            for (k, v) in map {
                if matches!(
                    k.as_str(),
                    "schema_version"
                        | "fingerprint"
                        | "source_revision"
                        | "bundle_digest"
                        | "content_digest"
                        | "guidance_digest"
                        | "source_hashes"
                        | "consumer_source_hashes"
                ) {
                    continue;
                }
                render_value(out, k, v, depth + 1);
            }
        }
        Value::Array(values) => {
            if values.is_empty() {
                let _ = writeln!(out, "{pad}{}: none", label(key));
                return;
            }
            let _ = writeln!(out, "{pad}{} ({}):", label(key), values.len());
            for value in values.iter().take(20) {
                if value.is_object() || value.is_array() {
                    render_value(out, "item", value, depth + 1);
                } else {
                    let _ = writeln!(out, "{pad}  - {}", text(value));
                }
            }
            if values.len() > 20 {
                let _ = writeln!(
                    out,
                    "{pad}  … {} more; use --json for all details.",
                    values.len() - 20
                );
            }
        }
        _ => {
            let _ = writeln!(out, "{pad}{}: {}", label(key), text(v));
        }
    }
}

pub fn render(operation: &str, result: &Outcome) -> String {
    let d = &result.data;
    let mut out = String::new();
    match operation {
        "status" => {
            let stories = d["stories"]
                .as_array()
                .map(Vec::as_slice)
                .unwrap_or_default();
            let mut counts = BTreeMap::new();
            for story in stories {
                *counts.entry(text(&story["status"])).or_insert(0usize) += 1;
            }
            let ready = stories
                .iter()
                .filter(|s| s["readiness"]["ready"] == true)
                .count();
            let _ = writeln!(
                out,
                "Project status\n\n  {} records · {} stories · {ready} ready to start",
                text(&d["records"]),
                stories.len()
            );
            for (status, count) in counts {
                let _ = writeln!(out, "  {status:<14} {count}");
            }
            let active: Vec<_> = stories
                .iter()
                .filter(|s| {
                    !matches!(
                        s["status"].as_str().unwrap_or(""),
                        "imported"
                            | "integrated"
                            | "released"
                            | "superseded"
                            | "cancelled"
                            | "deferred"
                    )
                })
                .collect();
            if !active.is_empty() {
                out.push_str("\nCurrent work:\n");
                for story in active.iter().take(12) {
                    let _ = writeln!(
                        out,
                        "  {}  [{}]",
                        text(&story["id"]),
                        text(&story["status"])
                    );
                    if let Some(reasons) = story["readiness"]["reasons"].as_array() {
                        for reason in reasons.iter().take(2) {
                            let _ = writeln!(out, "    {}", text(reason));
                        }
                        if reasons.len() > 2 {
                            let _ = writeln!(
                                out,
                                "    … {} more readiness conditions; inspect this story.",
                                reasons.len() - 2
                            );
                        }
                    }
                }
                if active.len() > 12 {
                    let _ = writeln!(out, "  … showing 12 of {} current stories.", active.len());
                }
            }
            diagnostics(&mut out, &d["diagnostics"]);
            out.push_str("\nInspect: aep context STORY-ID\nBrowse:  aep query --kind story --status pending\nFull readiness details: aep status --json\n");
        }
        "check" | "migrate.verify" | "spec.check" | "openspec.check" => {
            let pass = result.exit_code == 0 && d["pass"] != false;
            let _ = writeln!(
                out,
                "{} — {}",
                operation.replace('.', " "),
                if pass { "PASS" } else { "FAIL" }
            );
            field(&mut out, "records", &d["records"]);
            diagnostics(&mut out, &d["diagnostics"]);
            if !d["reference"].is_null() {
                render_value(&mut out, "reference validator", &d["reference"], 0);
            }
            if let Some(receipts) = d["migration_receipts"].as_array() {
                let _ = writeln!(out, "  Migration receipts: {}", receipts.len());
            }
            if let Some(note) = d["context_review_required"].as_str() {
                let _ = writeln!(out, "\n{note}.");
            }
            out.push_str("\nThese are context checks; product behavior needs the project's verification procedure.\n");
        }
        "doctor" => {
            out.push_str("AEP environment\n\n");
            field(&mut out, "version", &d["cli_version"]);
            let _ = writeln!(
                out,
                "  Platform: {} / {}",
                text(&d["platform"]["os"]),
                text(&d["platform"]["arch"])
            );
            field(&mut out, "git", &d["git"]);
            let project = &d["project"];
            if project["root"].is_null() {
                out.push_str("\nProject setup is unavailable.\n");
                field(&mut out, "reason", &project["diagnostic"]["message"]);
                out.push_str("\nSetup guidance: aep --skill project\n");
            } else {
                field(&mut out, "project", &project["root"]);
                field(&mut out, "compatible", &project["compatible"]);
                field(&mut out, "recovery_required", &project["recovery_required"]);
            }
            out.push_str("\nCapabilities:\n");
            if let Some(caps) = d["capabilities"].as_object() {
                for (name, c) in caps {
                    let state = match (c["available"].as_bool(), c["executable_present"].as_bool())
                    {
                        (Some(true), _) => "available",
                        (Some(false), _) => "setup needed",
                        (_, Some(true)) => "tool found; access not checked",
                        _ => "optional tool not found",
                    };
                    let _ = writeln!(out, "  {}: {state}", label(name));
                }
            }
        }
        "context" => {
            let values = d["records"]
                .as_array()
                .map(Vec::as_slice)
                .unwrap_or_default();
            let _ = writeln!(out, "Context: {}", text(&d["subject"]));
            field(&mut out, "repository", &d["repository"]);
            out.push('\n');
            if let Some(subject) = values.iter().find(|v| record(v)["id"] == d["subject"]) {
                record_detail(&mut out, subject);
            }
            let linked: Vec<_> = values
                .iter()
                .filter(|v| record(v)["id"] != d["subject"])
                .cloned()
                .collect();
            let _ = writeln!(out, "\nLinked context ({}):", linked.len());
            record_list(&mut out, &linked, 20);
            strings(&mut out, "Missing references", &d["missing_references"]);
            if let Some(sources) = d["sources"].as_array() {
                for source in sources {
                    let _ = writeln!(
                        out,
                        "\nSource: {}\n{}",
                        text(&source["path"]),
                        text(&source["content"])
                    );
                }
            }
            out.push_str("\nThis follows recorded links and incoming accepted decisions. Use --json for their full content; reconcile intent with the request and actual behavior.\n");
        }
        "query" => {
            let values = d["records"]
                .as_array()
                .map(Vec::as_slice)
                .unwrap_or_default();
            let _ = writeln!(out, "Records ({})\n", values.len());
            record_list(&mut out, values, 30);
            out.push_str("\nFilter: aep query --kind story --status pending\nInspect: aep context RECORD-ID\n");
        }
        "timeline" => {
            let values = d["events"]
                .as_array()
                .map(Vec::as_slice)
                .unwrap_or_default();
            let _ = writeln!(out, "Timeline ({} recorded events)\n", values.len());
            for event in values.iter().rev().take(30) {
                let _ = writeln!(
                    out,
                    "  {}  {}  {}",
                    text(&event["created_at"]),
                    text(&event["id"]),
                    text(&event["title"])
                );
            }
            if values.is_empty() {
                out.push_str("  No events recorded.\n");
            }
            if values.len() > 30 {
                out.push_str("  Showing the latest 30; use --json for the complete timeline.\n");
            }
            field(&mut out, "ordering", &d["ordering"]);
        }
        "deliver.plan" | "deliver.pr" | "deliver.merge" if d.get("eligible").is_some() => {
            out.push_str("Delivery readiness\n\n");
            let state = if d["eligible"] == true {
                "Ready for the authorized delivery action"
            } else {
                "Requirements remain"
            };
            let _ = writeln!(out, "  Candidate: {state}");
            for key in ["story", "attempt", "head"] {
                field(&mut out, key, &d["verification"][key]);
            }
            strings(&mut out, "Missing requirements", &d["missing"]);
            render_value(&mut out, "evidence", &d["evidence"], 0);
            out.push_str("\nThis inspection performs no delivery or cleanup.\n");
            if let Some(guidance) = d["guidance"].as_str() {
                let _ = writeln!(out, "{guidance}");
            }
        }
        "migrate.plan" => {
            let p = &d["plan"];
            out.push_str("Migration plan\n\n");
            for k in ["source", "base_commit"] {
                field(&mut out, k, &p[k]);
            }
            let records = p["records"].as_array().map_or(0, Vec::len);
            let writes = p["writes"].as_object().map_or(0, |m| m.len());
            let _ = writeln!(
                out,
                "  Records to import: {records}\n  Context files to write: {writes}"
            );
            field(&mut out, "saved_plan", &d["output"]);
            diagnostics(&mut out, &p["diagnostics"]);
            out.push_str("\nReview the saved plan and resolve its diagnostics before applying.\n");
            if d["output"].is_null() {
                out.push_str("Save: aep migrate plan --output migration.json\nFull plan: aep migrate plan --json\n");
            } else {
                out.push_str("Apply after review: aep migrate apply --plan <saved-plan>\n");
            }
        }
        "config.show" => {
            out.push_str("Project configuration\n\n");
            render_value(&mut out, "configuration", &d["config"], 0);
            field(&mut out, "revision", &d["revision"]);
        }
        _ if d.get("record").is_some() => record_detail(&mut out, d),
        _ if d.is_array() => {
            let rows = d.as_array().unwrap();
            let _ = writeln!(out, "{} ({})\n", operation.replace('.', " "), rows.len());
            record_list(&mut out, rows, 30);
        }
        _ if d.get("operation_id").is_some() => {
            let state = if result.exit_code != 0 {
                if result.changed {
                    "FAIL — evidence saved"
                } else {
                    "FAIL"
                }
            } else if d["dry_run"] == true {
                "Preview — nothing applied"
            } else if result.changed {
                "Saved"
            } else {
                "No changes needed"
            };
            let _ = writeln!(out, "{} — {state}", operation.replace('.', " "));
            if let Some(checks) = d["check_results"].as_array() {
                for check in checks {
                    let data = &check["data"];
                    let _ = writeln!(
                        out,
                        "\nCheck id: {} [{}]",
                        text(&data["check_id"]),
                        text(&check["status"])
                    );
                    for key in ["stale_reason", "missing_environment", "execution_error"] {
                        render_value(&mut out, key, &data[key], 1);
                    }
                    if check["status"] != "pass" {
                        render_value(&mut out, "result", &data["result"], 1);
                    }
                }
            }
            strings(&mut out, "Records", &d["records"]);
            if let Some(files) = d["files"].as_array() {
                let _ = writeln!(out, "\nFiles ({}):", files.len());
                for path in files.iter().take(20) {
                    let _ = writeln!(out, "  {}", text(path));
                }
                if files.len() > 20 {
                    let _ = writeln!(
                        out,
                        "  … {} more; use --json for all paths.",
                        files.len() - 20
                    );
                }
            }
            if let Some(obj) = d.as_object() {
                for (k, v) in obj {
                    if !matches!(
                        k.as_str(),
                        "operation_id"
                            | "source_revision"
                            | "records"
                            | "files"
                            | "dry_run"
                            | "check_results"
                    ) {
                        render_value(&mut out, k, v, 0);
                    }
                }
            }
        }
        _ => render_value(&mut out, &operation.replace('.', " "), d, 0),
    }
    if d.get("diagnostics") != Some(&Value::Array(result.diagnostics.clone())) {
        diagnostics(&mut out, &Value::Array(result.diagnostics.clone()));
    }
    out
}

pub fn error(error: &aep_core::Error) -> String {
    let mut out = format!("Error: {}\n", error.message);
    if let Ok(details) = serde_json::from_str::<Value>(&error.message) {
        out = "Error: project validation failed\n".into();
        diagnostics(&mut out, &details);
    }
    if error.side_effects {
        out.push_str("\nSome changes were applied. Inspect the recorded state before retrying.\n");
    }
    if error.code == "conflict" {
        out.push_str("\nRead the current record/configuration revision before retrying.\n");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn failed_reference_validator_retains_the_reason() {
        let mut result = Outcome::ok(json!({"diagnostics":[],"reference":{
            "code":1,"stdout":"Checked feature-a", "stderr":"Missing scenario: retry"
        }}));
        result.exit_code = 1;
        let output = render("openspec.check", &result);
        assert!(output.contains("FAIL"));
        assert!(output.contains("Missing scenario: retry"));
        assert!(output.contains("Checked feature-a"));
    }
}
