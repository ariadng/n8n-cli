use crate::client::N8nClient;
use crate::error::Result;
use crate::models::{Tag, TagAssignment, TagCreate, TagId, TagUpdate};

impl N8nClient {
    /// List all tags
    pub async fn list_tags(&self) -> Result<Vec<Tag>> {
        self.get("/tags").await
    }

    /// Create a new tag
    pub async fn create_tag(&self, name: &str) -> Result<Tag> {
        let request = TagCreate {
            name: name.to_string(),
        };
        self.post("/tags", &request).await
    }

    /// Update a tag
    pub async fn update_tag(&self, id: &str, name: &str) -> Result<Tag> {
        let request = TagUpdate {
            name: name.to_string(),
        };
        self.put(&format!("/tags/{}", id), &request).await
    }

    /// Delete a tag
    pub async fn delete_tag(&self, id: &str) -> Result<()> {
        self.delete(&format!("/tags/{}", id)).await
    }

    /// Assign tags to a workflow
    pub async fn assign_tags(&self, workflow_id: &str, tag_ids: Vec<String>) -> Result<()> {
        let request = TagAssignment {
            tags: tag_ids.into_iter().map(|id| TagId { id }).collect(),
        };
        // This endpoint returns the workflow, but we don't need it
        let _: serde_json::Value = self
            .put(&format!("/workflows/{}/tags", workflow_id), &request)
            .await?;
        Ok(())
    }
}
