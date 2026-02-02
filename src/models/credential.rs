use crate::models::common::{format_timestamp_str, truncate};
use crate::output::Outputable;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Credential summary
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Credential {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub credential_type: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl Outputable for Credential {
    fn headers() -> Vec<&'static str> {
        vec!["ID", "NAME", "TYPE", "UPDATED"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            truncate(&self.name, 30),
            self.credential_type.clone(),
            format_timestamp_str(&self.updated_at),
        ]
    }
}

/// Credential creation request
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CredentialCreate {
    pub name: String,
    #[serde(rename = "type")]
    pub credential_type: String,
    pub data: Value,
}

/// Credential schema response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CredentialSchema {
    #[serde(flatten)]
    pub schema: Value,
}
