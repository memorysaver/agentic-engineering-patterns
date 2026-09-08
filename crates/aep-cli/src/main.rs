mod cli;
mod commands;
mod guidance;
mod migration;
mod spec;
mod workflow;

use clap::{CommandFactory, FromArgMatches};
use serde_json::json;
fn main() {
    let raw: Vec<_> = std::env::args_os().collect();
    let wants_json = raw.iter().any(|a| a == "--json");
    let matches = match cli::Cli::command().try_get_matches_from(raw) {
        Ok(matches) => matches,
        Err(e) => {
            let code = e.exit_code();
            if wants_json {
                println!(
                    "{}",
                    json!({"schema_version":1,"cli_version":aep_core::VERSION,"operation":"parse","ok":code==0,"exit_code":code,"side_effects":false,"data":if code==0{json!({"help":e.to_string()})}else{serde_json::Value::Null},"diagnostics":if code==0{json!([])}else{json!([{"code":"usage","message":e.to_string()}])}})
                );
            } else {
                let _ = e.print();
            }
            std::process::exit(code);
        }
    };
    let mut commands = vec![];
    let mut current = &matches;
    while let Some((name, next)) = current.subcommand() {
        commands.push(name);
        current = next;
    }
    let operation = commands.join(".");
    let args = cli::Cli::from_arg_matches(&matches).expect("validated CLI arguments");
    workflow::install_interrupt_handler();
    match commands::run(&args) {
        Ok(out) => {
            if args.json {
                println!(
                    "{}",
                    json!({"schema_version":1,"cli_version":aep_core::VERSION,"operation":operation,"ok":out.exit_code==0,"exit_code":out.exit_code,"side_effects":out.changed,"data":out.data,"diagnostics":out.diagnostics})
                );
            } else if let Some(text) = out.text {
                println!("{text}");
            } else {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&out.data).expect("JSON output")
                );
                for d in out.diagnostics {
                    eprintln!("{d}");
                }
            }
            std::process::exit(out.exit_code);
        }
        Err(e) => {
            if args.json {
                println!(
                    "{}",
                    json!({"schema_version":1,"cli_version":aep_core::VERSION,"operation":operation,"ok":false,"exit_code":e.exit_code,"side_effects":e.side_effects,"data":null,"diagnostics":[e]})
                );
            } else {
                eprintln!("{e}");
            }
            std::process::exit(e.exit_code);
        }
    }
}
