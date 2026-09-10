use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "aep",
    version,
    about = "Keep project context, work, and verification together",
    disable_help_subcommand = true
)]
pub struct Cli {
    /// Project directory (defaults to the current directory).
    #[arg(long, global = true, default_value = ".")]
    pub root: PathBuf,
    /// Print structured JSON instead of the human-readable result.
    #[arg(long, global = true)]
    pub json: bool,
    /// Preview supported writes without applying them.
    #[arg(long, global = true)]
    pub dry_run: bool,
    /// Print the agent skill, or a named procedure, as Markdown.
    #[arg(long, num_args = 0..=1, default_missing_value = "aep", value_name = "NAME")]
    pub skill: Option<String>,
    /// Read a supporting reference from the selected procedure.
    #[arg(long = "ref", requires = "skill", value_name = "NAME")]
    pub reference: Option<String>,
    #[command(subcommand)]
    pub command: Option<Command>,
}
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Create the CLI pin and minimal instruction entrypoints in a Git project.
    Init {
        #[arg(long)]
        claude: bool,
    },
    /// Inspect or update project configuration.
    Config {
        #[command(subcommand)]
        command: ConfigAction,
    },
    /// Check installation, project setup, and available tools.
    Doctor,
    /// See project progress and which stories are ready or blocked.
    Status,
    /// Find records by kind or status.
    Query {
        #[arg(long)]
        kind: Option<String>,
        #[arg(long)]
        status: Option<String>,
    },
    /// Read a subject and its linked project context.
    Context {
        id: String,
        #[arg(long)]
        source: Vec<String>,
    },
    /// Show recorded events, optionally for one subject.
    Timeline { id: Option<String> },
    /// Manage work items and their dependencies.
    Story(Records),
    /// Group work into product layers.
    Layer(Records),
    /// Group work into execution waves.
    Wave(Records),
    /// Track releases and environment promotion.
    Release(Records),
    /// Manage change contracts and acceptance.
    Change(Records),
    /// Record and supersede decisions.
    Decision(Records),
    /// Maintain accepted product direction.
    Roadmap(Records),
    /// Record and find execution lessons.
    Lesson(Records),
    /// Propose and adopt evidence-backed project rules.
    Rule(Records),
    /// Evaluate acceptance gates against current evidence.
    Gate(Records),
    /// Propose a project rule from observed evidence.
    Reflect {
        #[command(subcommand)]
        command: Reflect,
    },
    /// Validate project records, links, and specifications.
    Check,
    /// Plan or run the project checks for a story.
    Verify {
        #[command(subcommand)]
        command: Verify,
    },
    /// Check readiness and start an isolated implementation attempt.
    Dispatch {
        #[command(subcommand)]
        command: Dispatch,
    },
    /// Inspect an attempt and its Git worktree.
    Worktree {
        #[command(subcommand)]
        command: Worktree,
    },
    /// Inspect, update, or recover an implementation attempt.
    Attempt {
        #[command(subcommand)]
        command: Attempt,
    },
    /// Request or record an independent review.
    Review {
        #[command(subcommand)]
        command: Review,
    },
    /// Plan delivery, open a PR, or record authorized integration.
    Deliver {
        #[command(subcommand)]
        command: Deliver,
    },
    /// Check, compare, and publish change specifications.
    Spec {
        #[command(subcommand)]
        command: Spec,
    },
    /// Import, map, or export OpenSpec context.
    Openspec {
        #[command(subcommand)]
        command: Openspec,
    },
    /// Plan, apply, or verify a legacy project migration.
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
    /// Return a completed or cancelled story to pending work.
    Reopen { id: String },
    /// Create a draft record from a YAML or JSON file.
    New(Input),
    /// Record structured input from a YAML or JSON file.
    Record(Input),
    /// Update a draft using its expected revision.
    Update {
        id: String,
        #[arg(long)]
        file: PathBuf,
        #[arg(long)]
        expect: String,
    },
    /// Read the current value and revision.
    #[command(alias = "status")]
    Show { id: String },
    /// List records of this kind.
    List,
    /// Accept a draft contract or decision with attribution.
    Accept {
        id: String,
        #[arg(long)]
        by: String,
    },
    /// Replace an accepted decision with another accepted decision.
    Supersede {
        id: String,
        #[arg(long)]
        by: String,
    },
    /// Close a published change whose stories are integrated.
    Close { id: String },
    /// Record release promotion to an environment.
    Promote {
        id: String,
        #[arg(long)]
        environment: String,
        #[arg(long)]
        by: String,
    },
    /// Adopt a rule using integrated verification evidence.
    Adopt {
        #[arg(long)]
        evidence: Vec<String>,
        id: String,
        #[arg(long)]
        by: String,
    },
    /// Retire an adopted project rule.
    Retire { id: String },
    /// Evaluate a gate using current checks and reviews.
    Evaluate { id: String },
    /// Search titles and descriptions for text.
    Find { query: String },
    /// Validate project records, links, and specifications.
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
    /// Record a proposed rule and its supporting observations.
    Propose(Input),
}
#[derive(Debug, Subcommand)]
pub enum Verify {
    /// Inspect prerequisites and the proposed work without applying it.
    Plan {
        #[arg(long)]
        story: String,
    },
    /// Execute configured checks in the bound candidate worktree.
    Run {
        #[arg(long)]
        story: String,
        #[arg(long)]
        check: Vec<String>,
    },
}
#[derive(Debug, Subcommand)]
pub enum Dispatch {
    /// Inspect prerequisites and the proposed work without applying it.
    Plan {
        #[arg(long)]
        story: Option<String>,
    },
    /// Create an implementation attempt and isolated Git worktree.
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
    /// Inspect the recorded worktree and current Git state.
    Inspect {
        #[arg(long)]
        attempt: String,
    },
}
#[derive(Debug, Subcommand)]
pub enum Attempt {
    /// Inspect an attempt's current status and recorded worktree.
    Status { id: String },
    /// Update the execution status of an existing attempt.
    Record {
        id: String,
        #[arg(long, value_enum)]
        status: AttemptStatus,
    },
    /// Reconcile an interrupted attempt, or cancel it explicitly.
    Recover {
        id: String,
        #[arg(long)]
        cancel: bool,
    },
}
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum AttemptStatus {
    Running,
    /// Implementation is ready for review.
    Review,
    Failed,
    Cancelled,
}
#[derive(Debug, Subcommand)]
pub enum Review {
    /// Create a review request bound to the current candidate.
    Request {
        #[arg(long)]
        story: String,
    },
    /// Record structured input from a YAML or JSON file.
    Record(Input),
}
#[derive(Debug, Subcommand)]
pub enum Deliver {
    /// Reconcile a recorded integration intent after an interrupted provider response.
    Reconcile {
        #[arg(long)]
        story: String,
    },
    /// Inspect prerequisites and the proposed work without applying it.
    Plan {
        #[arg(long)]
        story: String,
    },
    /// Open a GitHub pull request for a verified story.
    Pr {
        #[arg(long)]
        story: String,
        #[arg(long)]
        base: String,
    },
    /// Integrate a verified story using the selected delivery adapter.
    Merge {
        #[arg(long)]
        story: String,
        #[arg(long)]
        local: bool,
    },
    /// Inspect the recorded delivery state of a story.
    Status {
        #[arg(long)]
        story: String,
    },
}
#[derive(Debug, Subcommand)]
pub enum Spec {
    /// Validate project records, links, and specifications.
    Check {
        #[arg(long)]
        change: Option<String>,
    },
    /// Compare a change against the published specifications.
    Diff {
        #[arg(long)]
        change: String,
    },
    /// Publish the specifications of an accepted change.
    Publish {
        #[arg(long)]
        change: String,
    },
}
#[derive(Debug, Subcommand)]
pub enum Openspec {
    /// Bind custom imported context to explicit active project rules or configuration.
    Map(Input),
    /// Import context from an OpenSpec directory.
    Import {
        #[arg(long)]
        source: PathBuf,
    },
    /// Export native change context to an OpenSpec directory.
    Export {
        #[arg(long)]
        output: PathBuf,
    },
    /// Validate project records, links, and specifications.
    Check {
        #[arg(long)]
        reference_cli: bool,
    },
}
#[derive(Debug, Subcommand)]
pub enum Migrate {
    /// Inspect prerequisites and the proposed work without applying it.
    Plan {
        #[arg(long)]
        source: Option<PathBuf>,
        #[arg(long)]
        story: Vec<String>,
        #[arg(long)]
        output: Option<PathBuf>,
        /// Tracked review of legacy host hooks, instructions, and scheduled writers.
        #[arg(long)]
        consumer_review: Option<PathBuf>,
    },
    /// Apply a reviewed migration plan.
    Apply {
        #[arg(long)]
        plan: PathBuf,
    },
    /// Check migrated context and retained source provenance.
    Verify,
}

#[derive(Debug, Subcommand)]
pub enum ConfigAction {
    /// Read the current value and revision.
    Show,
    /// Update a draft using its expected revision.
    Update {
        #[arg(long)]
        file: PathBuf,
        #[arg(long)]
        expect: String,
    },
}

/// One clap schema drives parsing, nested help and the grouped front page.
pub fn command() -> clap::Command {
    use clap::CommandFactory;
    let mut command = Cli::command().mut_subcommands(|sub| {
        let example = match sub.get_name() {
            "story" => "Examples:\n  aep story list\n  aep story show STORY-ID\n  aep story new --file story.yaml",
            "status" => "Next:\n  aep context STORY-ID          Read a story and its dependencies\n  aep query --kind story --status blocked",
            "context" => "Examples:\n  aep context STORY-ID\n  aep context STORY-ID --source README.md\n  aep context STORY-ID --json   Read the complete structured context",
            "query" => "Examples:\n  aep query --kind story\n  aep query --kind story --status pending",
            "verify" => "Examples:\n  aep verify plan --story STORY-ID\n  aep verify run --story STORY-ID\n\nChecks run in the story's bound worktree and retain their actual results.",
            "migrate" => "Examples:\n  aep migrate plan --output migration.json\n  aep migrate apply --plan migration.json\n  aep migrate verify\n\nInspect the plan and resolve its diagnostics before applying it.",
            "config" => "Examples:\n  aep config show\n  aep config show --json\n  aep config update --file config.toml --expect REVISION",
            "dispatch" => "Examples:\n  aep dispatch plan\n  aep dispatch plan --story STORY-ID\n  aep dispatch start --story STORY-ID --base main --owner developer",
            _ => "Use --json for the complete structured result.",
        };
        sub.after_help(example)
    });
    let groups: &[(&str, &[&str])] = &[
        (
            "Start here",
            &["init", "doctor", "status", "context", "check"],
        ),
        (
            "Browse and plan",
            &[
                "query", "timeline", "story", "roadmap", "change", "decision", "layer", "wave",
            ],
        ),
        (
            "Implement and verify",
            &[
                "dispatch", "worktree", "attempt", "verify", "review", "gate",
            ],
        ),
        (
            "Deliver and learn",
            &["deliver", "release", "spec", "lesson", "reflect", "rule"],
        ),
        (
            "Project maintenance",
            &["config", "migrate", "openspec", "recover"],
        ),
    ];
    let mut help = format!(
        "aep — {}\n\nUsage: aep [OPTIONS] <COMMAND>\n       aep --skill [NAME] [--ref NAME]\n",
        command.get_about().unwrap()
    );
    for (title, names) in groups {
        help.push_str(&format!("\n{title}:\n"));
        for name in *names {
            let sub = command.find_subcommand(name).expect("documented command");
            help.push_str(&format!("  {name:<12} {}\n", sub.get_about().unwrap()));
        }
    }
    help.push_str("\nOptions:\n  --root <PATH>    Use a project directory (default: current directory)\n  --json           Print structured JSON\n  --dry-run        Preview supported writes without applying them\n  --skill [NAME]   Print the agent skill or a named procedure as Markdown\n  --ref <NAME>     Read a reference with --skill <NAME>\n  -h, --help       Show help\n  -V, --version    Show version\n\nExamples:\n  aep status\n  aep context STORY-ID\n  aep verify --help\n\nFor coding agents:\n  aep --skill      Read the bundled SKILL.md and discover project procedures\n");
    command = command.override_help(help);
    command
}
