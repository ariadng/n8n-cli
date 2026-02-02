use clap::{Args, Subcommand};

#[derive(Args)]
pub struct HealthCommand {
    #[command(subcommand)]
    pub action: HealthAction,
}

#[derive(Subcommand)]
pub enum HealthAction {
    /// Basic health check (/healthz)
    Check,

    /// Readiness check (/healthz/readiness)
    Ready,
}
