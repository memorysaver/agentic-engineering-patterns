mod cli;
mod commands;
mod guidance;
mod migration;
mod output;
mod spec;
mod workflow;

use clap::FromArgMatches;
use serde_json::json;
use std::io::{self, Write};

fn emit(text: &str, stderr: bool) {
    let result = if stderr {
        io::stderr().lock().write_all(text.as_bytes())
    } else {
        io::stdout().lock().write_all(text.as_bytes())
    };
    if let Err(error) = result {
        if error.kind() == io::ErrorKind::BrokenPipe {
            std::process::exit(0);
        }
        eprintln!("Cannot write output: {error}");
        std::process::exit(1);
    }
}
fn parse_error(error: clap::Error, wants_json: bool) -> ! {
    let help = matches!(
        error.kind(),
        clap::error::ErrorKind::DisplayHelp
            | clap::error::ErrorKind::DisplayVersion
            | clap::error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
    );
    let code = if help { 0 } else { error.exit_code() };
    if wants_json {
        emit(
            &format!(
                "{}\n",
                json!({"schema_version":1,"cli_version":aep_core::VERSION,"operation":"parse","ok":code==0,"exit_code":code,"side_effects":false,"data":if help {json!({"help":error.to_string()})} else {serde_json::Value::Null},"diagnostics":if help {json!([])} else {json!([{"code":"usage","message":error.to_string()}])}})
            ),
            false,
        );
    } else {
        emit(&error.to_string(), !help);
    }
    std::process::exit(code);
}
fn main() {
    let raw: Vec<_> = std::env::args_os().collect();
    let wants_json = raw.iter().any(|a| a == "--json");
    let matches = cli::command()
        .try_get_matches_from(raw)
        .unwrap_or_else(|e| parse_error(e, wants_json));
    let args = cli::Cli::from_arg_matches(&matches).expect("validated CLI arguments");
    if args.skill.is_some() && args.command.is_some() {
        parse_error(
            cli::command().error(
                clap::error::ErrorKind::ArgumentConflict,
                "Use --skill on its own; it cannot be combined with an operation.",
            ),
            args.json,
        );
    }
    if args.skill.is_none() && args.command.is_none() {
        let help = cli::command().render_long_help().to_string();
        if args.json {
            emit(
                &format!(
                    "{}\n",
                    json!({"schema_version":1,"cli_version":aep_core::VERSION,"operation":"help","ok":true,"exit_code":0,"side_effects":false,"data":{"help":help},"diagnostics":[]})
                ),
                false,
            );
        } else {
            emit(&help, false);
        }
        return;
    }
    let mut commands = vec![];
    let mut current = &matches;
    while let Some((name, next)) = current.subcommand() {
        commands.push(name);
        current = next;
    }
    let operation = if args.skill.is_some() {
        "skill".into()
    } else {
        commands.join(".")
    };
    workflow::install_interrupt_handler();
    match commands::run(&args) {
        Ok(out) => {
            let rendered = if args.json {
                format!(
                    "{}\n",
                    json!({"schema_version":1,"cli_version":aep_core::VERSION,"operation":operation,"ok":out.exit_code==0,"exit_code":out.exit_code,"side_effects":out.changed,"data":out.data,"diagnostics":out.diagnostics})
                )
            } else if let Some(ref text) = out.text {
                text.clone()
            } else {
                output::render(&operation, &out)
            };
            emit(&rendered, false);
            std::process::exit(out.exit_code);
        }
        Err(e) => {
            if args.json {
                emit(
                    &format!(
                        "{}\n",
                        json!({"schema_version":1,"cli_version":aep_core::VERSION,"operation":operation,"ok":false,"exit_code":e.exit_code,"side_effects":e.side_effects,"data":null,"diagnostics":[e]})
                    ),
                    false,
                );
            } else {
                emit(&output::error(&e), true);
            }
            std::process::exit(e.exit_code);
        }
    }
}
