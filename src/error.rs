use serde::Deserialize;
use thiserror::Error;

/// All possible errors in the n8n CLI
#[derive(Debug, Error)]
pub enum N8nError {
    // Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Invalid API key format")]
    InvalidApiKey,

    #[error("API key not configured. Set N8N_API_KEY environment variable or use --api-key")]
    MissingApiKey,

    #[error("Base URL not configured. Set N8N_BASE_URL environment variable or use --url")]
    MissingBaseUrl,

    #[error("Profile '{0}' not found in configuration")]
    ProfileNotFound(String),

    #[error("Failed to read config file: {0}")]
    ConfigFileRead(#[source] std::io::Error),

    #[error("Failed to parse config file: {0}")]
    ConfigFileParse(#[source] toml::de::Error),

    // HTTP/Network errors
    #[error("HTTP client error: {0}")]
    HttpClient(#[source] reqwest::Error),

    #[error("Request failed: {0}")]
    Request(#[source] reqwest::Error),

    #[error("Connection failed to {url}: {message}\n\nHint: Check that n8n is running and accessible")]
    ConnectionFailed { url: String, message: String },

    // API errors
    #[error("{}", .0.user_message())]
    Api(ApiErrorResponse),

    // Serialization
    #[error("Failed to parse response: {0}")]
    Deserialize(#[source] reqwest::Error),

    #[error("Failed to parse input: {0}")]
    InvalidInput(#[source] serde_json::Error),

    #[error("Failed to serialize data: {0}")]
    Serialize(#[source] serde_json::Error),

    // Resource errors
    #[error("Workflow '{0}' not found")]
    WorkflowNotFound(String),

    #[error("Execution '{0}' not found")]
    ExecutionNotFound(String),

    #[error("Credential '{0}' not found")]
    CredentialNotFound(String),

    // I/O errors
    #[error("Failed to read file '{path}': {source}")]
    FileRead {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to write file '{path}': {source}")]
    FileWrite {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to read from stdin: {0}")]
    StdinRead(#[source] std::io::Error),

    // User interaction
    #[error("Operation cancelled by user")]
    Cancelled,

    // Workflow editing errors
    #[error("Node '{0}' not found in workflow")]
    NodeNotFound(String),

    #[error("Connection not found: {from} -> {to}")]
    ConnectionNotFound { from: String, to: String },

    #[error("Validation failed:\n{0}")]
    ValidationFailed(String),

    #[error("Editor failed: {0}")]
    EditorFailed(String),

    #[error("No changes detected")]
    NoChanges,
}

/// API error response from n8n
#[derive(Debug, Deserialize, Clone)]
pub struct ApiErrorResponse {
    pub code: u16,
    pub message: String,
    pub hint: Option<String>,
}

impl ApiErrorResponse {
    /// Format error for user display
    pub fn user_message(&self) -> String {
        match &self.hint {
            Some(hint) => format!("{}\n\nHint: {}", self.message, hint),
            None => self.message.clone(),
        }
    }

    /// Create a generic error for unknown status codes
    pub fn unknown(status: reqwest::StatusCode) -> Self {
        Self {
            code: status.as_u16(),
            message: format!("Request failed with status {}", status),
            hint: None,
        }
    }
}

impl N8nError {
    /// Convert errors to exit codes following Unix conventions
    pub fn exit_code(&self) -> i32 {
        match self {
            // Configuration errors (EX_CONFIG = 78)
            Self::Config(_)
            | Self::ProfileNotFound(_)
            | Self::ConfigFileRead(_)
            | Self::ConfigFileParse(_)
            | Self::MissingBaseUrl => 78,

            // Permission errors (EX_NOPERM = 77)
            Self::InvalidApiKey | Self::MissingApiKey => 77,
            Self::Api(e) if e.code == 401 || e.code == 403 => 77,

            // Resource unavailable (EX_UNAVAILABLE = 69)
            Self::Api(e) if e.code == 404 => 69,
            Self::WorkflowNotFound(_)
            | Self::ExecutionNotFound(_)
            | Self::CredentialNotFound(_) => 69,
            Self::ConnectionFailed { .. } | Self::Request(_) | Self::HttpClient(_) => 69,

            // I/O errors (EX_IOERR = 74)
            Self::FileRead { .. } | Self::FileWrite { .. } | Self::StdinRead(_) => 74,

            // Data errors (EX_DATAERR = 65)
            Self::InvalidInput(_) | Self::Serialize(_) | Self::Deserialize(_) => 65,

            // User cancelled
            Self::Cancelled => 130, // Standard for Ctrl+C

            // Workflow editing errors
            Self::NodeNotFound(_) | Self::ConnectionNotFound { .. } => 69, // EX_UNAVAILABLE
            Self::ValidationFailed(_) => 65,                               // EX_DATAERR
            Self::EditorFailed(_) => 74,                                   // EX_IOERR
            Self::NoChanges => 0,                                          // Not an error

            // Generic failure
            Self::Api(_) => 1,
        }
    }
}

/// Result type alias for n8n operations
pub type Result<T> = std::result::Result<T, N8nError>;
