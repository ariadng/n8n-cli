use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Args)]
pub struct WorkflowsCommand {
    #[command(subcommand)]
    pub action: WorkflowsAction,
}

#[derive(Subcommand)]
pub enum WorkflowsAction {
    /// List all workflows
    List {
        /// Filter by active status
        #[arg(long, short = 'a')]
        active: Option<bool>,

        /// Filter by tags (comma-separated)
        #[arg(long, short = 't', value_delimiter = ',')]
        tags: Option<Vec<String>>,

        /// Filter by name (partial match)
        #[arg(long, short = 'n')]
        name: Option<String>,

        /// Maximum results to return
        #[arg(long, default_value = "100")]
        limit: u32,

        /// Pagination cursor from previous response
        #[arg(long)]
        cursor: Option<String>,

        /// Fetch all pages automatically
        #[arg(long)]
        all: bool,
    },

    /// Get a single workflow by ID
    Get {
        /// Workflow ID
        id: String,
    },

    /// Create a workflow from JSON file
    Create {
        /// Path to workflow JSON file (use - for stdin)
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Activate workflow after creation
        #[arg(long)]
        activate: bool,
    },

    /// Update an existing workflow
    Update {
        /// Workflow ID
        id: String,

        /// Path to workflow JSON file (use - for stdin)
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },

    /// Delete a workflow
    Delete {
        /// Workflow ID
        id: String,

        /// Skip confirmation prompt
        #[arg(long, short = 'f')]
        force: bool,
    },

    /// Activate a workflow
    Activate {
        /// Workflow ID
        id: String,
    },

    /// Deactivate a workflow
    Deactivate {
        /// Workflow ID
        id: String,
    },

    /// Manage workflow nodes
    Nodes(NodesCommand),

    /// Manage workflow connections
    Connections(ConnectionsCommand),

    /// Open workflow in external editor
    Edit {
        /// Workflow ID
        id: String,

        /// Editor to use (defaults to $EDITOR or $VISUAL)
        #[arg(long, env = "EDITOR")]
        editor: Option<String>,

        /// Skip validation before upload
        #[arg(long)]
        no_validate: bool,
    },

    /// Compare workflows
    Diff {
        /// First workflow ID
        id: String,

        /// Second workflow ID (if comparing two workflows)
        #[arg(long, conflicts_with = "file")]
        with: Option<String>,

        /// Local file to compare against (if comparing with file)
        #[arg(long, conflicts_with = "with")]
        file: Option<PathBuf>,

        /// Show full diff (not just summary)
        #[arg(long, short)]
        full: bool,
    },

    /// Export workflow to file
    Export {
        /// Workflow ID
        id: String,

        /// Output file (defaults to stdout)
        #[arg(long, short)]
        file: Option<PathBuf>,

        /// Pretty-print JSON output
        #[arg(long)]
        pretty: bool,
    },

    /// Clone/duplicate a workflow
    Clone {
        /// Source workflow ID
        id: String,

        /// Name for the new workflow
        #[arg(long, short)]
        name: String,

        /// Activate the cloned workflow
        #[arg(long)]
        activate: bool,
    },

    /// Validate workflow structure
    Validate {
        /// Workflow ID (mutually exclusive with --file)
        #[arg(conflicts_with = "file")]
        id: Option<String>,

        /// Local file to validate
        #[arg(long, conflicts_with = "id")]
        file: Option<PathBuf>,

        /// Show warnings (not just errors)
        #[arg(long)]
        warnings: bool,
    },

    /// Run/trigger a workflow (webhook workflows only)
    Run {
        /// Workflow ID
        id: String,

        /// Input data as JSON
        #[arg(long, short = 'd')]
        data: Option<String>,

        /// Input data from file
        #[arg(long, conflicts_with = "data")]
        data_file: Option<PathBuf>,

        /// HTTP method for webhook (GET, POST, etc.)
        #[arg(long, short = 'm', default_value = "POST")]
        method: String,

        /// Don't wait for execution to complete
        #[arg(long)]
        no_wait: bool,
    },
}

#[derive(Args)]
pub struct NodesCommand {
    #[command(subcommand)]
    pub action: NodesAction,
}

#[derive(Subcommand)]
pub enum NodesAction {
    /// List all nodes in a workflow
    List {
        /// Workflow ID
        workflow_id: String,
    },

    /// Get a single node
    Get {
        /// Workflow ID
        workflow_id: String,

        /// Node ID or name
        node_id: String,
    },

    /// Add a new node to a workflow
    Add {
        /// Workflow ID
        workflow_id: String,

        /// n8n node type (e.g., "n8n-nodes-base.httpRequest")
        #[arg(long, short = 't')]
        r#type: String,

        /// Node display name
        #[arg(long, short)]
        name: String,

        /// Position as "x,y" (e.g., "200,300")
        #[arg(long, value_parser = parse_position)]
        position: Option<(i32, i32)>,

        /// Node configuration as JSON
        #[arg(long, short)]
        config: Option<String>,

        /// Node configuration from file
        #[arg(long, conflicts_with = "config")]
        config_file: Option<PathBuf>,

        /// Disable the node
        #[arg(long)]
        disabled: bool,
    },

    /// Remove a node from a workflow
    Remove {
        /// Workflow ID
        workflow_id: String,

        /// Node ID or name
        node_id: String,

        /// Skip confirmation prompt
        #[arg(long, short = 'f')]
        force: bool,
    },

    /// Update a node's configuration
    Update {
        /// Workflow ID
        workflow_id: String,

        /// Node ID or name
        node_id: String,

        /// New node name
        #[arg(long)]
        name: Option<String>,

        /// New position as "x,y"
        #[arg(long, value_parser = parse_position)]
        position: Option<(i32, i32)>,

        /// Node configuration as JSON (merges with existing)
        #[arg(long, short)]
        config: Option<String>,

        /// Replace entire configuration (instead of merge)
        #[arg(long)]
        replace: bool,

        /// Enable/disable the node
        #[arg(long)]
        disabled: Option<bool>,
    },

    /// Move a node to a new position
    Move {
        /// Workflow ID
        workflow_id: String,

        /// Node ID or name
        node_id: String,

        /// New position as "x,y"
        #[arg(value_parser = parse_position)]
        position: (i32, i32),
    },
}

#[derive(Args)]
pub struct ConnectionsCommand {
    #[command(subcommand)]
    pub action: ConnectionsAction,
}

#[derive(Subcommand)]
pub enum ConnectionsAction {
    /// List all connections in a workflow
    List {
        /// Workflow ID
        workflow_id: String,

        /// Filter by source node
        #[arg(long)]
        from: Option<String>,

        /// Filter by target node
        #[arg(long)]
        to: Option<String>,
    },

    /// Add a connection between nodes
    Add {
        /// Workflow ID
        workflow_id: String,

        /// Source node ID or name
        #[arg(long)]
        from: String,

        /// Target node ID or name
        #[arg(long)]
        to: String,

        /// Source output index (default: 0)
        #[arg(long = "output-index", default_value = "0")]
        output_index: u32,

        /// Target input index (default: 0)
        #[arg(long = "input-index", default_value = "0")]
        input_index: u32,

        /// Connection type (default: "main")
        #[arg(long, default_value = "main")]
        r#type: String,
    },

    /// Remove a connection
    Remove {
        /// Workflow ID
        workflow_id: String,

        /// Source node ID or name
        #[arg(long)]
        from: String,

        /// Target node ID or name
        #[arg(long)]
        to: String,

        /// Skip confirmation prompt
        #[arg(long, short = 'f')]
        force: bool,
    },
}

/// Parse position string "x,y" into (i32, i32)
fn parse_position(s: &str) -> Result<(i32, i32), String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err("Position must be in format 'x,y'".to_string());
    }
    let x = parts[0]
        .trim()
        .parse::<i32>()
        .map_err(|_| "Invalid x coordinate")?;
    let y = parts[1]
        .trim()
        .parse::<i32>()
        .map_err(|_| "Invalid y coordinate")?;
    Ok((x, y))
}
