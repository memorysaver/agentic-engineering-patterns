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
            source: format!("aep:{}:{path}", aep_core::VERSION),
        });
    }
    Ok(skills)
}
fn local(root: &Path, dir: &str) -> Result<Skill> {
    let body = std::fs::read_to_string(contained(root, &format!("{dir}/SKILL.md"))?)?;
    let (name, description) = metadata(&body)?;
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
        source: format!("project:{dir}/SKILL.md"),
    })
}
pub fn run(args: &Cli, name: &str, reference: Option<&str>) -> Result<Outcome> {
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
    match name {
        "aep" => {
            if reference.is_some() {
                return Err(Error::input(
                    "Select a procedure before using --ref; see aep --skill",
                ));
            }
            let entries:Vec<_>=skills.iter().map(|s|json!({"name":s.name,"description":s.description,"read_command":format!("aep --skill {}",s.name),"source":s.source,"content_digest":digest(&s.body)})).collect();
            let mut text = asset("SKILL.md")?;
            text.push_str("\n## Available procedures\n\nRead only the procedure needed for your task. Project procedures retain their own rules and verification requirements.\n\n");
            for (project, heading) in [(false, "Built-in procedures"), (true, "Project procedures")]
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
                        s.name, s.description, s.name
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
                json!({"metadata":common,"name":name,"reference":reference,"source":skill.source,"content_digest":digest(content),"content":content,"references":refs}),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn embedded_catalog_has_complete_procedures_and_references() {
        let skills = builtin().unwrap();
        let names: std::collections::BTreeSet<_> = skills.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names.len(), skills.len());
        assert_eq!(names.len(), 8);
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
