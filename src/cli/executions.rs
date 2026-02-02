use clap::{Args, Subcommand};

#[derive(Args)]
pub struct ExecutionsCommand {
    #[command(subcommand)]
    pub action: ExecutionsAction,
}

#[derive(Subcommand)]
pub enum ExecutionsAction {
    /// List executions
    List {
        /// Filter by workflow ID
        #[arg(long, short = 'w')]
        workflow_id: Option<String>,

        /// Filter by status (running, success, error, waiting, canceled)
        #[arg(long, short = 's')]
        status: Option<String>,

        /// Include full execution data
        #[arg(long)]
        include_data: bool,

        /// Maximum results to return
        #[arg(long, default_value = "100")]
        limit: u32,

        /// Pagination cursor
        #[arg(long)]
        cursor: Option<String>,
    },

    /// Get execution details
    Get {
        /// Execution ID
        id: String,

        /// Include full execution data
        #[arg(long)]
        include_data: bool,
    },

    /// Delete an execution
    Delete {
        /// Execution ID
        id: String,
    },

    /// Retry a failed execution
    Retry {
        /// Execution ID
        id: String,
    },

    /// Execute a workflow
    Run {
        /// Workflow ID to execute
        workflow_id: String,

        /// Wait for execution to complete
        #[arg(long)]
        wait: bool,

        /// Input data as JSON string
        #[arg(long, short = 'd')]
        data: Option<String>,
    },
}
