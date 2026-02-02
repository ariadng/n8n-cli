use serde::{Deserialize, Serialize};

/// Paginated response wrapper matching n8n API format
#[derive(Debug, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    #[serde(rename = "nextCursor")]
    pub next_cursor: Option<String>,
}

/// Query parameters for paginated endpoints
#[derive(Debug, Default, Serialize)]
pub struct PaginationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl PaginationParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit.min(250)); // API max is 250
        self
    }

    pub fn with_cursor(mut self, cursor: Option<String>) -> Self {
        self.cursor = cursor;
        self
    }
}
