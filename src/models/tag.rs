use crate::models::common::format_timestamp_str;
use crate::output::Outputable;
use serde::{Deserialize, Serialize};

/// Tag
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    #[serde(rename = "createdAt", default)]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default)]
    pub updated_at: Option<String>,
}

impl Outputable for Tag {
    fn headers() -> Vec<&'static str> {
        vec!["ID", "NAME", "UPDATED"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.name.clone(),
            self.updated_at
                .as_ref()
                .map(|s| format_timestamp_str(s))
                .unwrap_or_else(|| "-".to_string()),
        ]
    }
}

/// Tag creation request
#[derive(Debug, Clone, Serialize)]
pub struct TagCreate {
    pub name: String,
}

/// Tag update request
#[derive(Debug, Clone, Serialize)]
pub struct TagUpdate {
    pub name: String,
}

/// Workflow tag assignment request
#[derive(Debug, Clone, Serialize)]
pub struct TagAssignment {
    pub tags: Vec<TagId>,
}

/// Tag ID for assignment
#[derive(Debug, Clone, Serialize)]
pub struct TagId {
    pub id: String,
}
