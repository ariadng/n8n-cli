use clap::{Args, Subcommand};

#[derive(Args)]
pub struct TagsCommand {
    #[command(subcommand)]
    pub action: TagsAction,
}

#[derive(Subcommand)]
pub enum TagsAction {
    /// List all tags
    List,

    /// Create a new tag
    Create {
        /// Tag name
        name: String,
    },

    /// Update a tag
    Update {
        /// Tag ID
        id: String,

        /// New tag name
        #[arg(long)]
        name: String,
    },

    /// Delete a tag
    Delete {
        /// Tag ID
        id: String,
    },

    /// Assign tags to a workflow
    Assign {
        /// Workflow ID
        workflow_id: String,

        /// Tags to assign (comma-separated)
        #[arg(long, short = 't', value_delimiter = ',')]
        tags: Vec<String>,
    },
}
