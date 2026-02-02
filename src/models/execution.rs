use crate::models::common::format_timestamp_str;
use crate::output::Outputable;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Execution status values
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionStatus {
    Running,
    Success,
    Error,
    Waiting,
    Canceled,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for ExecutionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Running => write!(f, "running"),
            Self::Success => write!(f, "success"),
            Self::Error => write!(f, "error"),
            Self::Waiting => write!(f, "waiting"),
            Self::Canceled => write!(f, "canceled"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

/// Execution summary
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Execution {
    pub id: String,
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
    pub status: ExecutionStatus,
    #[serde(rename = "startedAt")]
    pub started_at: String,
    #[serde(rename = "stoppedAt")]
    pub stopped_at: Option<String>,
    #[serde(default)]
    pub finished: bool,
    #[serde(default)]
    pub mode: String,
}

impl Outputable for Execution {
    fn headers() -> Vec<&'static str> {
        vec!["ID", "WORKFLOW", "STATUS", "MODE", "STARTED", "STOPPED"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.workflow_id.clone(),
            self.status.to_string(),
            self.mode.clone(),
            format_timestamp_str(&self.started_at),
            self.stopped_at
                .as_ref()
                .map(|s| format_timestamp_str(s))
                .unwrap_or_else(|| "-".to_string()),
        ]
    }
}

/// Execution detail with full data
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExecutionDetail {
    pub id: String,
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
    pub status: ExecutionStatus,
    #[serde(rename = "startedAt")]
    pub started_at: String,
    #[serde(rename = "stoppedAt")]
    pub stopped_at: Option<String>,
    pub finished: bool,
    pub mode: String,
    #[serde(default)]
    pub data: Option<Value>,
}

/// Request body for executing a workflow
#[derive(Debug, Clone, Serialize)]
pub struct ExecuteRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}
