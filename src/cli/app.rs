use crate::output::OutputFormat;
use clap::{Parser, Subcommand};

use super::{
    CredentialsCommand, ExecutionsCommand, HealthCommand, TagsCommand, WorkflowsCommand,
};

/// n8n CLI - Manage n8n workflows from the command line
#[derive(Parser)]
#[command(name = "n8n")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Configuration profile to use
    #[arg(long, short = 'p', global = true, env = "N8N_PROFILE")]
    pub profile: Option<String>,

    /// n8n instance base URL
    #[arg(long, global = true, env = "N8N_BASE_URL")]
    pub url: Option<String>,

    /// API key (prefer N8N_API_KEY env var for security)
    #[arg(long, global = true, env = "N8N_API_KEY", hide_env_values = true)]
    pub api_key: Option<String>,

    /// Output format
    #[arg(long, short = 'o', global = true, default_value = "table", value_enum)]
    pub output: OutputFormat,

    /// Enable verbose output
    #[arg(long, short = 'v', global = true)]
    pub verbose: bool,

    /// Suppress non-essential output
    #[arg(long, short = 'q', global = true)]
    pub quiet: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage workflows
    #[command(alias = "wf")]
    Workflows(WorkflowsCommand),

    /// Manage executions
    #[command(alias = "exec")]
    Executions(ExecutionsCommand),

    /// Manage credentials
    #[command(alias = "cred")]
    Credentials(CredentialsCommand),

    /// Manage tags
    Tags(TagsCommand),

    /// Health checks
    Health(HealthCommand),

    /// Show current configuration
    Config,

    /// Install Claude Code skill for n8n workflow development
    #[command(name = "install-claude-skill")]
    InstallClaudeSkill {
        /// Overwrite existing skill without prompting
        #[arg(long, short = 'f')]
        force: bool,
    },
}
