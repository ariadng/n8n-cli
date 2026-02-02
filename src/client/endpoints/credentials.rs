use crate::client::{N8nClient, PaginatedResponse};
use crate::error::Result;
use crate::models::{Credential, CredentialCreate, CredentialSchema};
use serde::Serialize;

/// Query parameters for listing credentials
#[derive(Debug, Default, Serialize)]
pub struct CredentialListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub credential_type: Option<String>,
}

impl N8nClient {
    /// List credentials with optional filters
    pub async fn list_credentials(
        &self,
        params: &CredentialListParams,
    ) -> Result<PaginatedResponse<Credential>> {
        self.get_with_query("/credentials", params).await
    }

    /// Get credential schema for a type
    pub async fn get_credential_schema(&self, type_name: &str) -> Result<CredentialSchema> {
        self.get(&format!("/credentials/schema/{}", type_name))
            .await
    }

    /// Create a new credential
    pub async fn create_credential(&self, credential: &CredentialCreate) -> Result<Credential> {
        self.post("/credentials", credential).await
    }

    /// Update an existing credential
    pub async fn update_credential(
        &self,
        id: &str,
        credential: &CredentialCreate,
    ) -> Result<Credential> {
        self.put(&format!("/credentials/{}", id), credential).await
    }

    /// Delete a credential
    pub async fn delete_credential(&self, id: &str) -> Result<()> {
        self.delete(&format!("/credentials/{}", id)).await
    }
}
