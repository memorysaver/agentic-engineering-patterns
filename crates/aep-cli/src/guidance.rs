use crate::{cli::Cli, commands::Outcome};
use aep_core::{Error, Result, digest, valid_id};
use aep_store::{contained, frontmatter, parse_yaml, read_optional};
use serde_json::{Value, json};
use std::{collections::BTreeMap, path::Path};

const EMBEDDED: &str = include_str!(concat!(env!("OUT_DIR"), "/skills.json"));
fn files() -> BTreeMap<String, String> {
    serde_json::from_str(EMBEDDED).expect("build-produced skill bundle")
}
pub fn asset(path: &str) -> Result<String> {
    files()
        .remove(path)
        .ok_or_else(|| Error::input(format!("Unknown bundled resource {path}")))
}
pub fn bundle_digest() -> String {
    digest(EMBEDDED)
}
struct Skill {
    name: String,
    description: String,
    body: String,
    refs: BTreeMap<String, String>,
    source: String,
    steps: Option<Vec<Value>>,
}
/// The evidence kinds a runbook step may declare.
const EVIDENCE_KINDS: [&str; 7] = [
    "check-id",
    "file-content",
    "record-id",
    "pr-or-commit",
    "review-result",
    "command-output",
    "git-ref",
];
/// The steps of a runbook, or `None` for a procedure. A procedure has no `kind`, or `kind: procedure`.
fn runbook_steps(body: &str) -> Result<Option<Vec<Value>>> {
    let (front, _) = frontmatter(body)?;
    let data: Value = parse_yaml(front)?;
    match data["kind"].as_str() {
        None if data.get("kind").is_some() => Err(Error::input("Skill kind must be a string")),
        None | Some("procedure") => Ok(None),
        Some("runbook") => data["steps"]
            .as_array()
            .cloned()
            .map(Some)
            .ok_or_else(|| Error::input("A runbook needs a steps list")),
        Some(other) => Err(Error::input(format!(
            "Unknown skill kind {other}; use procedure or runbook"
        ))),
    }
}
fn runbook_error(name: &str, step: Option<&str>, problem: &str) -> Error {
    Error::input(match step {
        Some(step) => format!("Runbook {name}, step {step} {problem}"),
        None => format!("Runbook {name} {problem}"),
    })
}
/// Checks each step of a runbook against the evidence kinds and the admitted principles.
fn check_steps(name: &str, steps: &[Value], principles: &[String]) -> Result<()> {
    if steps.is_empty() {
        return Err(runbook_error(name, None, "has no steps"));
    }
    let mut seen = std::collections::BTreeSet::new();
    for step in steps {
        let object = step
            .as_object()
            .ok_or_else(|| runbook_error(name, None, "has a step that is not a mapping"))?;
        let id = object
            .get("id")
            .and_then(Value::as_str)
            .filter(|id| valid_id(id))
            .ok_or_else(|| runbook_error(name, None, "has a step without a valid id"))?;
        if !seen.insert(id) {
            return Err(runbook_error(name, Some(id), "is listed twice"));
        }
        for key in object.keys() {
            if !["id", "do", "evidence", "principles"].contains(&key.as_str()) {
                return Err(runbook_error(
                    name,
                    Some(id),
                    &format!("has an unknown field {key}"),
                ));
            }
        }
        if object
            .get("do")
            .and_then(Value::as_str)
            .is_none_or(|text| text.trim().is_empty())
        {
            return Err(runbook_error(name, Some(id), "needs a non-empty do"));
        }
        let evidence = object
            .get("evidence")
            .and_then(Value::as_array)
            .filter(|kinds| !kinds.is_empty())
            .ok_or_else(|| runbook_error(name, Some(id), "needs a non-empty evidence list"))?;
        for kind in evidence {
            let kind = kind.as_str().unwrap_or("");
            if !EVIDENCE_KINDS.contains(&kind) {
                return Err(runbook_error(
                    name,
                    Some(id),
                    &format!("has evidence {kind} outside the fixed set"),
                ));
            }
        }
        if let Some(cited) = object.get("principles") {
            let cited = cited
                .as_array()
                .ok_or_else(|| runbook_error(name, Some(id), "lists principles as a non-list"))?;
            for principle in cited {
                let principle = principle.as_str().unwrap_or("");
                if !principles.iter().any(|admitted| admitted == principle) {
                    return Err(runbook_error(
                        name,
                        Some(id),
                        &format!("cites {principle}, which is not an admitted principle"),
                    ));
                }
            }
        }
    }
    Ok(())
}
/// The steps of a bundled runbook, by name.
pub(crate) fn runbook_steps_named(name: &str) -> Result<Vec<Value>> {
    builtin()?
        .into_iter()
        .find(|skill| skill.name == name)
        .and_then(|skill| skill.steps)
        .ok_or_else(|| Error::input(format!("No bundled runbook named {name}")))
}
/// Checks every runbook against the admitted principles, which are the references of the
/// bundled `principles` procedure.
fn validate_runbooks(skills: &[Skill]) -> Result<()> {
    let principles: Vec<String> = skills
        .iter()
        .find(|s| s.name == "principles")
        .map(|s| s.refs.keys().cloned().collect())
        .unwrap_or_default();
    for skill in skills {
        if let Some(steps) = &skill.steps {
            check_steps(&skill.name, steps, &principles)?;
        }
    }
    Ok(())
}
fn metadata(body: &str) -> Result<(String, String)> {
    let (front, _) = frontmatter(body)?;
    let data: Value = parse_yaml(front)?;
    let name = data["name"]
        .as_str()
        .filter(|s| valid_id(s))
        .ok_or_else(|| Error::input("Skill needs a valid name"))?;
    let description = data["description"]
        .as_str()
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| Error::input("Skill needs an applicability description"))?;
    Ok((name.into(), description.into()))
}
fn builtin() -> Result<Vec<Skill>> {
    let files = files();
    let mut skills = vec![];
    for (path, body) in &files {
        let Some(dir) = path.strip_suffix("/SKILL.md") else {
            continue;
        };
        let (name, description) = metadata(body)?;
        let steps = runbook_steps(body)?;
        let prefix = format!("{dir}/references/");
        let refs = files
            .iter()
            .filter_map(|(p, text)| {
                p.strip_prefix(&prefix)
                    .and_then(|p| p.strip_suffix(".md"))
                    .map(|id| (id.into(), text.clone()))
            })
            .collect();
        skills.push(Skill {
            name,
            description,
            body: body.clone(),
            refs,
            steps,
            source: format!("aep:{}:{path}", aep_core::VERSION),
        });
    }
    validate_runbooks(&skills)?;
    Ok(skills)
}
fn local(root: &Path, dir: &str) -> Result<Skill> {
    let body = std::fs::read_to_string(contained(root, &format!("{dir}/SKILL.md"))?)?;
    let (name, description) = metadata(&body)?;
    let steps = runbook_steps(&body)?;
    let mut refs = BTreeMap::new();
    let reference_dir = contained(root, &format!("{dir}/references"))?;
    if reference_dir.exists() {
        for entry in std::fs::read_dir(reference_dir)? {
            let entry = entry?;
            let filename = entry.file_name().to_string_lossy().to_string();
            if let Some(id) = filename.strip_suffix(".md")
                && valid_id(id)
            {
                let text = std::fs::read_to_string(contained(
                    root,
                    &format!("{dir}/references/{filename}"),
                )?)?;
                refs.insert(id.into(), text);
            }
        }
    }
    Ok(Skill {
        name,
        description,
        body,
        refs,
        steps,
        source: format!("project:{dir}/SKILL.md"),
    })
}
/// File name of the per-checkout guidance read log under the Git common directory.
pub const READ_LOG: &str = "guidance-reads.jsonl";
/// Appends one line per guidance read beside the transaction journal, outside the
/// project tree, so eval can see whether a procedure was opened before acting.
/// Reading guidance must never fail because the log cannot be written.
fn record_read(root: &Path, name: &str, reference: Option<&str>, attempt: Option<&str>) {
    let Ok(common) = aep_store::git(
        root,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    ) else {
        return;
    };
    let dir = Path::new(&common).join("aep");
    let procedure = if ["aep", "aep"].contains(&name) {
        "catalog"
    } else {
        name
    };
    let source = if std::env::var_os("AEP_CHECK_RUN").is_some() {
        "check"
    } else {
        "agent"
    };
    let mut line = json!({"at": aep_core::now(), "procedure": procedure, "reference": reference, "source": source, "guidance_digest": bundle_digest()});
    if let Some(attempt) = attempt {
        line["attempt"] = json!(attempt);
    }
    let _ = std::fs::create_dir_all(&dir).and_then(|_| {
        use std::io::Write;
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(dir.join(READ_LOG))
            .and_then(|mut f| writeln!(f, "{line}"))
    });
}
pub fn run(args: &Cli, name: &str, reference: Option<&str>) -> Result<Outcome> {
    if args
        .attempt
        .as_ref()
        .is_some_and(|id| !id.starts_with("attempt-") || !valid_id(id))
    {
        return Err(Error::input("--attempt needs a valid attempt- identifier"));
    }
    let mut skills = builtin()?;
    let requested = std::fs::canonicalize(&args.root)?;
    // Match repository-root discovery for workflow commands, without invoking Git.
    // Without a repository boundary, the nearest explicit config defines standalone guidance.
    let root = requested
        .ancestors()
        .find(|root| root.join(".git").exists())
        .or_else(|| {
            requested
                .ancestors()
                .find(|root| root.join(".aep/config.toml").exists())
        });
    if let Some(root) = root
        && let Some(content) = read_optional(&contained(root, ".aep/config.toml")?)?
    {
        let mut config: aep_core::Config =
            toml::from_str(&content).map_err(|e| Error::input(e.to_string()))?;
        config.stores.rules = aep_core::normalize_scope(&config.stores.rules)
            .ok_or_else(|| Error::input("Invalid project rules path"))?;
        let mut paths = config
            .skill_paths
            .iter()
            .map(|path| {
                aep_core::normalize_scope(path)
                    .ok_or_else(|| Error::input("Invalid project skill path"))
            })
            .collect::<Result<Vec<_>>>()?;
        let default = format!("{}/skills", config.stores.rules);
        let dir = contained(root, &default)?;
        if dir.exists() {
            for entry in std::fs::read_dir(dir)? {
                let entry = entry?;
                let name = entry.file_name().to_string_lossy().to_string();
                let path = format!("{default}/{name}");
                let target = contained(root, &path)?;
                if target.is_dir() && contained(root, &format!("{path}/SKILL.md"))?.exists() {
                    paths.push(path);
                }
            }
        }
        paths.sort();
        paths.dedup();
        for path in paths {
            skills.push(local(root, &path)?);
        }
    }
    skills.sort_by(|a, b| a.name.cmp(&b.name));
    if skills.iter().any(|s| s.name == "aep") || skills.windows(2).any(|s| s[0].name == s[1].name) {
        return Err(Error::input(
            "Duplicate or reserved skill name; project procedures cannot shadow bundled skills or aep",
        ));
    }
    let common =
        json!({"release":aep_core::VERSION,"bundle_digest":bundle_digest(),"complete":true});
    let answer = (|| {
        match name {
            "aep" => {
                if reference.is_some() {
                    return Err(Error::input(
                        "Select a procedure before using --ref; see aep --skill",
                    ));
                }
                let entries:Vec<_>=skills.iter().map(|s|json!({"name":s.name,"description":s.description,"kind":if s.steps.is_some(){"runbook"}else{"procedure"},"read_command":format!("aep --skill {}",s.name),"source":s.source,"content_digest":digest(&s.body)})).collect();
                let mut text = asset("SKILL.md")?;
                text.push_str("\n## Available procedures\n\nRead only the procedure needed for your task. Project procedures retain their own rules and verification requirements.\n\n");
                for (project, heading) in
                    [(false, "Built-in procedures"), (true, "Project procedures")]
                {
                    let group: Vec<_> = skills
                        .iter()
                        .filter(|s| s.source.starts_with("project:") == project)
                        .collect();
                    if group.is_empty() {
                        continue;
                    }
                    text.push_str(&format!("### {heading}\n\n"));
                    for s in group {
                        text.push_str(&format!(
                            "- {}: {}\n  aep --skill {}\n",
                            if s.steps.is_some() {
                                format!("{} (runbook)", s.name)
                            } else {
                                s.name.clone()
                            },
                            s.description,
                            s.name
                        ));
                        if let Some(path) = s.source.strip_prefix("project:") {
                            text.push_str(&format!("  Source: `{path}`\n"));
                        }
                    }
                    text.push('\n');
                }
                Ok(Outcome::text(
                    text.clone(),
                    json!({"metadata":common,"skills":entries,"content":text}),
                ))
            }
            name => {
                let skill = skills.iter().find(|s| s.name == name).ok_or_else(|| {
                    Error::input(format!("Unknown skill {name}; inspect aep --skill"))
                })?;
                let content = if let Some(id) = reference {
                    // Resolve only catalogued resources; a Markdown link is an alias,
                    // never an arbitrary filesystem path.
                    skill
                        .refs
                        .get(id)
                        .or_else(|| {
                            skill.refs.iter().find_map(|(key, body)| {
                                (id == format!("references/{key}.md")).then_some(body)
                            })
                        })
                        .ok_or_else(|| {
                            Error::input(format!(
                                "Unknown reference {id}; read an available resource with:\n{}",
                                skill
                                    .refs
                                    .keys()
                                    .map(|key| format!("  aep --skill {name} --ref {key}"))
                                    .collect::<Vec<_>>()
                                    .join("\n")
                            ))
                        })?
                } else {
                    &skill.body
                };
                let refs:Vec<_>=skill.refs.keys().map(|id|json!({"name":id,"path":format!("references/{id}.md"),"read_command":format!("aep --skill {} --ref {id}",skill.name)})).collect();
                Ok(Outcome::text(
                    content.clone(),
                    json!({"metadata":common,"name":name,"kind":if skill.steps.is_some(){"runbook"}else{"procedure"},"steps":skill.steps,"reference":reference,"source":skill.source,"content_digest":digest(content),"content":content,"references":refs}),
                ))
            }
        }
    })();
    if answer.is_ok()
        && !args.dry_run
        && args.attempt.is_some()
        && let Some(root) = root
    {
        record_read(root, name, reference, args.attempt.as_deref());
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn embedded_catalog_has_complete_procedures_and_references() {
        let skills = builtin().unwrap();
        let names: std::collections::BTreeSet<_> = skills.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names.len(), skills.len());
        assert_eq!(names.len(), 11);
        assert!(names.contains("eval"));
        for skill in &skills {
            assert!(skill.body.lines().count() <= 400);
            assert!(!skill.description.trim().is_empty());
            for (name, text) in &skill.refs {
                assert!(valid_id(name));
                assert!(!text.trim().is_empty());
            }
        }
        let design = skills.iter().find(|s| s.name == "design").unwrap();
        assert!(design.refs.contains_key("bdd"));
        assert!(design.refs.contains_key("records"));
        assert!(!skills.iter().any(|s| s.name == "route"));
    }
}

#[cfg(test)]
mod runbook_tests {
    use super::*;

    fn procedure(front: &str) -> String {
        format!("---\nname: sample\ndescription: A sample.\n{front}---\n\n# Sample\n")
    }
    fn steps(text: &str) -> Vec<Value> {
        let body = procedure(&format!("kind: runbook\nsteps:\n{text}"));
        runbook_steps(&body).unwrap().unwrap()
    }
    fn principles() -> Vec<String> {
        vec!["prove-it".into(), "encode-lessons".into()]
    }

    #[test]
    fn a_procedure_without_kind_has_no_steps() {
        assert!(runbook_steps(&procedure("")).unwrap().is_none());
        assert!(
            runbook_steps(&procedure("kind: procedure\n"))
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn an_unknown_kind_is_refused() {
        let err = runbook_steps(&procedure("kind: recipe\n")).unwrap_err();
        assert!(err.message.contains("recipe"), "{}", err.message);
    }

    #[test]
    fn a_runbook_without_steps_is_refused() {
        let err = runbook_steps(&procedure("kind: runbook\n")).unwrap_err();
        assert!(err.message.contains("steps"), "{}", err.message);
    }

    #[test]
    fn a_valid_runbook_passes_the_check() {
        let list = steps(
            "  - id: reproduce\n    do: Reproduce it.\n    evidence: [command-output]\n    principles: [prove-it]\n",
        );
        assert!(check_steps("sample", &list, &principles()).is_ok());
    }

    #[test]
    fn an_evidence_kind_outside_the_fixed_set_is_refused_naming_the_step() {
        let list = steps("  - id: shot\n    do: Look.\n    evidence: [screenshot]\n");
        let err = check_steps("sample", &list, &principles()).unwrap_err();
        assert!(
            err.message.contains("shot") && err.message.contains("screenshot"),
            "{}",
            err.message
        );
    }

    #[test]
    fn an_unadmitted_principle_is_refused() {
        let list = steps(
            "  - id: go\n    do: Go.\n    evidence: [git-ref]\n    principles: [attack-the-premise]\n",
        );
        let err = check_steps("sample", &list, &principles()).unwrap_err();
        assert!(
            err.message.contains("attack-the-premise"),
            "{}",
            err.message
        );
    }

    #[test]
    fn a_duplicate_step_or_an_unknown_field_is_refused() {
        let dup = steps(
            "  - id: a\n    do: One.\n    evidence: [git-ref]\n  - id: a\n    do: Two.\n    evidence: [git-ref]\n",
        );
        assert!(
            check_steps("sample", &dup, &principles())
                .unwrap_err()
                .message
                .contains("twice")
        );
        let extra = steps("  - id: a\n    do: One.\n    evidence: [git-ref]\n    when: always\n");
        assert!(
            check_steps("sample", &extra, &principles())
                .unwrap_err()
                .message
                .contains("when")
        );
    }
}
