use crate::client::{N8nClient, PaginatedResponse};
use crate::error::Result;
use crate::models::{ExecuteRequest, Execution, ExecutionDetail};
use serde::Serialize;

/// Query parameters for listing executions
#[derive(Debug, Default, Serialize)]
pub struct ExecutionListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "workflowId")]
    pub workflow_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeData")]
    pub include_data: Option<bool>,
}

impl N8nClient {
    /// List executions with optional filters
    pub async fn list_executions(
        &self,
        params: &ExecutionListParams,
    ) -> Result<PaginatedResponse<Execution>> {
        self.get_with_query("/executions", params).await
    }

    /// Get execution details
    pub async fn get_execution(&self, id: &str, include_data: bool) -> Result<ExecutionDetail> {
        if include_data {
            self.get_with_query(
                &format!("/executions/{}", id),
                &[("includeData", "true")],
            )
            .await
        } else {
            self.get(&format!("/executions/{}", id)).await
        }
    }

    /// Delete an execution
    pub async fn delete_execution(&self, id: &str) -> Result<()> {
        self.delete(&format!("/executions/{}", id)).await
    }

    /// Retry a failed execution
    pub async fn retry_execution(&self, id: &str) -> Result<ExecutionDetail> {
        self.post_empty(&format!("/executions/{}/retry", id)).await
    }

    /// Execute a workflow
    pub async fn execute_workflow(
        &self,
        workflow_id: &str,
        data: Option<serde_json::Value>,
    ) -> Result<ExecutionDetail> {
        let request = ExecuteRequest { data };
        self.post(&format!("/workflows/{}/execute", workflow_id), &request)
            .await
    }
}
