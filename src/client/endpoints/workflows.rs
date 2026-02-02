use crate::client::{N8nClient, PaginatedResponse};
use crate::error::Result;
use crate::models::{Workflow, WorkflowDefinition, WorkflowDetail};
use serde::Serialize;

/// Query parameters for listing workflows
#[derive(Debug, Default, Serialize)]
pub struct WorkflowListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl N8nClient {
    /// List workflows with optional filters
    pub async fn list_workflows(
        &self,
        params: &WorkflowListParams,
    ) -> Result<PaginatedResponse<Workflow>> {
        self.get_with_query("/workflows", params).await
    }

    /// Get a single workflow by ID
    pub async fn get_workflow(&self, id: &str) -> Result<WorkflowDetail> {
        self.get(&format!("/workflows/{}", id)).await
    }

    /// Create a new workflow
    pub async fn create_workflow(&self, workflow: &WorkflowDefinition) -> Result<WorkflowDetail> {
        self.post("/workflows", workflow).await
    }

    /// Update an existing workflow
    pub async fn update_workflow(
        &self,
        id: &str,
        workflow: &WorkflowDefinition,
    ) -> Result<WorkflowDetail> {
        self.put(&format!("/workflows/{}", id), workflow).await
    }

    /// Delete a workflow
    pub async fn delete_workflow(&self, id: &str) -> Result<()> {
        self.delete(&format!("/workflows/{}", id)).await
    }

    /// Activate a workflow
    pub async fn activate_workflow(&self, id: &str) -> Result<WorkflowDetail> {
        self.post_empty(&format!("/workflows/{}/activate", id))
            .await
    }

    /// Deactivate a workflow
    pub async fn deactivate_workflow(&self, id: &str) -> Result<WorkflowDetail> {
        self.post_empty(&format!("/workflows/{}/deactivate", id))
            .await
    }

    /// List all workflows (auto-paginate)
    pub async fn list_all_workflows(
        &self,
        mut params: WorkflowListParams,
    ) -> Result<Vec<Workflow>> {
        let mut all_workflows = Vec::new();
        let mut cursor: Option<String> = None;

        loop {
            params.cursor = cursor;
            let response = self.list_workflows(&params).await?;
            all_workflows.extend(response.data);

            match response.next_cursor {
                Some(next) => cursor = Some(next),
                None => break,
            }
        }

        Ok(all_workflows)
    }
}
