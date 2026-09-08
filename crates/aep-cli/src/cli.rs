use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "aep",
    version,
    about = "Project records, checks, and agent-readable skills"
)]
pub struct Cli {
    #[arg(long, global = true, default_value = ".")]
    pub root: PathBuf,
    #[arg(long, global = true)]
    pub json: bool,
    #[arg(long, global = true)]
    pub dry_run: bool,
    #[command(subcommand)]
    pub command: Command,
}
#[derive(Debug, Subcommand)]
pub enum Command {
    /// List skills or read a procedure; the working agent chooses what applies.
    Skills {
        #[command(subcommand)]
        command: Option<Skills>,
    },
    /// Create the CLI pin and minimal instruction entrypoints in a Git project.
    Init {
        #[arg(long)]
        claude: bool,
    },
    Config {
        #[command(subcommand)]
        command: ConfigAction,
    },
    Doctor,
    Status,
    Query {
        #[arg(long)]
        kind: Option<String>,
        #[arg(long)]
        status: Option<String>,
    },
    Context {
        id: String,
        #[arg(long)]
        source: Vec<String>,
    },
    Timeline {
        id: Option<String>,
    },
    Story(Records),
    Layer(Records),
    Wave(Records),
    Release(Records),
    Change(Records),
    Decision(Records),
    Roadmap(Records),
    Lesson(Records),
    Rule(Records),
    Gate(Records),
    Reflect {
        #[command(subcommand)]
        command: Reflect,
    },
    Check,
    Verify {
        #[command(subcommand)]
        command: Verify,
    },
    Dispatch {
        #[command(subcommand)]
        command: Dispatch,
    },
    Worktree {
        #[command(subcommand)]
        command: Worktree,
    },
    Attempt {
        #[command(subcommand)]
        command: Attempt,
    },
    Review {
        #[command(subcommand)]
        command: Review,
    },
    Deliver {
        #[command(subcommand)]
        command: Deliver,
    },
    Spec {
        #[command(subcommand)]
        command: Spec,
    },
    Openspec {
        #[command(subcommand)]
        command: Openspec,
    },
    Migrate {
        #[command(subcommand)]
        command: Migrate,
    },
    /// Inspect an interrupted transaction; use --apply or --rollback explicitly.
    Recover {
        #[arg(long, conflicts_with = "rollback")]
        apply: bool,
        #[arg(long)]
        rollback: bool,
    },
    /// JSON views for consumers; readiness stays in Rust.
    Dashboard,
}
#[derive(Debug, Subcommand)]
pub enum Skills {
    Show {
        name: String,
        #[arg(long)]
        r#ref: Option<String>,
    },
}
#[derive(Debug, Args)]
pub struct Records {
    #[command(subcommand)]
    pub action: RecordAction,
}
#[derive(Debug, Subcommand)]
pub enum RecordAction {
    /// Resolve an imported completion claim against an inspected Git commit.
    Reconcile {
        id: String,
        #[arg(long)]
        commit: String,
        #[arg(long)]
        by: String,
    },
    Reopen {
        id: String,
    },
    New(Input),
    Record(Input),
    Update {
        id: String,
        #[arg(long)]
        file: PathBuf,
        #[arg(long)]
        expect: String,
    },
    #[command(alias = "status")]
    Show {
        id: String,
    },
    List,
    Accept {
        id: String,
        #[arg(long)]
        by: String,
    },
    Supersede {
        id: String,
        #[arg(long)]
        by: String,
    },
    Close {
        id: String,
    },
    Promote {
        id: String,
        #[arg(long)]
        environment: String,
        #[arg(long)]
        by: String,
    },
    Adopt {
        #[arg(long)]
        evidence: Vec<String>,
        id: String,
        #[arg(long)]
        by: String,
    },
    Retire {
        id: String,
    },
    Evaluate {
        id: String,
    },
    Find {
        query: String,
    },
    Check,
}
#[derive(Debug, Args)]
pub struct Input {
    /// YAML/JSON file; use - for stdin.
    #[arg(long)]
    pub file: PathBuf,
}
#[derive(Debug, Subcommand)]
pub enum Reflect {
    Propose(Input),
}
#[derive(Debug, Subcommand)]
pub enum Verify {
    Plan {
        #[arg(long)]
        story: String,
    },
    Run {
        #[arg(long)]
        story: String,
        #[arg(long)]
        check: Vec<String>,
    },
}
#[derive(Debug, Subcommand)]
pub enum Dispatch {
    Plan {
        #[arg(long)]
        story: Option<String>,
    },
    Start {
        #[arg(long)]
        story: String,
        #[arg(long)]
        base: String,
        #[arg(long)]
        owner: String,
        #[arg(long)]
        worktree: Option<PathBuf>,
    },
}
#[derive(Debug, Subcommand)]
pub enum Worktree {
    Inspect {
        #[arg(long)]
        attempt: String,
    },
}
#[derive(Debug, Subcommand)]
pub enum Attempt {
    Status {
        id: String,
    },
    Record {
        id: String,
        #[arg(long, value_enum)]
        status: AttemptStatus,
    },
    Recover {
        id: String,
        #[arg(long)]
        cancel: bool,
    },
}
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum AttemptStatus {
    Running,
    Review,
    Failed,
    Cancelled,
}
#[derive(Debug, Subcommand)]
pub enum Review {
    Request {
        #[arg(long)]
        story: String,
    },
    Record(Input),
}
#[derive(Debug, Subcommand)]
pub enum Deliver {
    /// Reconcile a recorded integration intent after an interrupted provider response.
    Reconcile {
        #[arg(long)]
        story: String,
    },
    Plan {
        #[arg(long)]
        story: String,
    },
    Pr {
        #[arg(long)]
        story: String,
        #[arg(long)]
        base: String,
    },
    Merge {
        #[arg(long)]
        story: String,
        #[arg(long)]
        local: bool,
    },
    Status {
        #[arg(long)]
        story: String,
    },
}
#[derive(Debug, Subcommand)]
pub enum Spec {
    Check {
        #[arg(long)]
        change: Option<String>,
    },
    Diff {
        #[arg(long)]
        change: String,
    },
    Publish {
        #[arg(long)]
        change: String,
    },
}
#[derive(Debug, Subcommand)]
pub enum Openspec {
    /// Bind custom imported context to explicit active project rules or configuration.
    Map(Input),
    Import {
        #[arg(long)]
        source: PathBuf,
    },
    Export {
        #[arg(long)]
        output: PathBuf,
    },
    Check {
        #[arg(long)]
        reference_cli: bool,
    },
}
#[derive(Debug, Subcommand)]
pub enum Migrate {
    Plan {
        #[arg(long)]
        source: Option<PathBuf>,
        #[arg(long)]
        story: Vec<String>,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    Apply {
        #[arg(long)]
        plan: PathBuf,
    },
    Verify,
}

#[derive(Debug, Subcommand)]
pub enum ConfigAction {
    Show,
    Update {
        #[arg(long)]
        file: PathBuf,
        #[arg(long)]
        expect: String,
    },
}
