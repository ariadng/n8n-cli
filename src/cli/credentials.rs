use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Args)]
pub struct CredentialsCommand {
    #[command(subcommand)]
    pub action: CredentialsAction,
}

#[derive(Subcommand)]
pub enum CredentialsAction {
    /// List all credentials
    List {
        /// Filter by credential type
        #[arg(long, short = 't')]
        r#type: Option<String>,
    },

    /// Get the schema for a credential type
    Schema {
        /// Credential type name (e.g., openAiApi, httpBasicAuth)
        type_name: String,
    },

    /// Create a new credential
    Create {
        /// Path to credential JSON file (use - for stdin)
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },

    /// Update an existing credential
    Update {
        /// Credential ID
        id: String,

        /// Path to credential JSON file (use - for stdin)
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },

    /// Delete a credential
    Delete {
        /// Credential ID
        id: String,

        /// Skip confirmation prompt
        #[arg(long, short = 'f')]
        force: bool,
    },
}
