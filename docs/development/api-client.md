# API Client

Documentation for the HTTP client implementation and endpoint patterns.

## Overview

The `N8nClient` struct provides a typed interface to the n8n REST API, handling authentication, serialization, and error mapping.

## N8nClient Structure

```rust
pub struct N8nClient {
    client: reqwest::Client,
    base_url: String,
}
```

### Construction

```rust
impl N8nClient {
    pub fn new(base_url: &str, api_key: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();

        // Authentication header
        headers.insert(
            "X-N8N-API-KEY",
            HeaderValue::from_str(api_key)
                .map_err(|_| N8nError::InvalidApiKey)?,
        );

        // Content type headers
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(N8nError::HttpClient)?;

        // Normalize URL (remove trailing slash, add /api/v1)
        let base_url = format!("{}/api/v1", base_url.trim_end_matches('/'));

        Ok(Self { client, base_url })
    }
}
```

## HTTP Methods

### GET

```rust
pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
    let url = format!("{}{}", self.base_url, path);

    let response = self.client
        .get(&url)
        .send()
        .await
        .map_err(|e| self.map_request_error(e, &url))?;

    self.handle_response(response).await
}
```

### GET with Query Parameters

```rust
pub async fn get_with_query<T, Q>(&self, path: &str, query: &Q) -> Result<T>
where
    T: DeserializeOwned,
    Q: Serialize,
{
    let url = format!("{}{}", self.base_url, path);

    let response = self.client
        .get(&url)
        .query(query)
        .send()
        .await
        .map_err(|e| self.map_request_error(e, &url))?;

    self.handle_response(response).await
}
```

### POST

```rust
pub async fn post<T, B>(&self, path: &str, body: &B) -> Result<T>
where
    T: DeserializeOwned,
    B: Serialize,
{
    let url = format!("{}{}", self.base_url, path);

    let response = self.client
        .post(&url)
        .json(body)
        .send()
        .await
        .map_err(|e| self.map_request_error(e, &url))?;

    self.handle_response(response).await
}
```

### POST (Empty Body)

```rust
pub async fn post_empty<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
    let url = format!("{}{}", self.base_url, path);

    let response = self.client
        .post(&url)
        .send()
        .await
        .map_err(|e| self.map_request_error(e, &url))?;

    self.handle_response(response).await
}
```

### PUT

```rust
pub async fn put<T, B>(&self, path: &str, body: &B) -> Result<T>
where
    T: DeserializeOwned,
    B: Serialize,
{
    let url = format!("{}{}", self.base_url, path);

    let response = self.client
        .put(&url)
        .json(body)
        .send()
        .await
        .map_err(|e| self.map_request_error(e, &url))?;

    self.handle_response(response).await
}
```

### DELETE

```rust
pub async fn delete(&self, path: &str) -> Result<()> {
    let url = format!("{}{}", self.base_url, path);

    let response = self.client
        .delete(&url)
        .send()
        .await
        .map_err(|e| self.map_request_error(e, &url))?;

    // DELETE typically returns no body
    if response.status().is_success() {
        Ok(())
    } else {
        Err(self.extract_api_error(response).await)
    }
}
```

## Error Handling

### Request Error Mapping

```rust
fn map_request_error(&self, error: reqwest::Error, url: &str) -> N8nError {
    if error.is_connect() {
        N8nError::ConnectionFailed {
            url: url.to_string(),
            message: "Connection refused".to_string(),
        }
    } else if error.is_timeout() {
        N8nError::ConnectionFailed {
            url: url.to_string(),
            message: "Request timed out".to_string(),
        }
    } else {
        N8nError::Request(error)
    }
}
```

### Response Handling

```rust
async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
    if response.status().is_success() {
        response.json::<T>().await.map_err(N8nError::Deserialize)
    } else {
        Err(self.extract_api_error(response).await)
    }
}

async fn extract_api_error(&self, response: Response) -> N8nError {
    let status = response.status();

    // Try to parse n8n error response
    match response.json::<ApiErrorResponse>().await {
        Ok(error) => N8nError::Api(error),
        Err(_) => N8nError::Api(ApiErrorResponse::unknown(status)),
    }
}
```

## Pagination

### PaginatedResponse Type

```rust
#[derive(Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    #[serde(rename = "nextCursor")]
    pub next_cursor: Option<String>,
}
```

### Auto-Pagination Pattern

```rust
impl N8nClient {
    pub async fn list_workflows_all(&self, params: &WorkflowListParams) -> Result<Vec<Workflow>> {
        let mut all_workflows = Vec::new();
        let mut cursor = None;

        loop {
            let mut query_params = params.clone();
            query_params.cursor = cursor;

            let response: PaginatedResponse<Workflow> = self
                .get_with_query("/workflows", &query_params)
                .await?;

            all_workflows.extend(response.data);

            match response.next_cursor {
                Some(c) => cursor = Some(c),
                None => break,
            }
        }

        Ok(all_workflows)
    }
}
```

## Endpoint Implementation Pattern

### Basic CRUD Endpoints

```rust
// In src/client/endpoints/workflows.rs

impl N8nClient {
    /// List workflows with optional filters
    pub async fn list_workflows(&self, params: &WorkflowListParams) -> Result<Vec<Workflow>> {
        let response: PaginatedResponse<Workflow> = self
            .get_with_query("/workflows", params)
            .await?;
        Ok(response.data)
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
        self.post_empty(&format!("/workflows/{}/activate", id)).await
    }

    /// Deactivate a workflow
    pub async fn deactivate_workflow(&self, id: &str) -> Result<WorkflowDetail> {
        self.post_empty(&format!("/workflows/{}/deactivate", id)).await
    }
}
```

### Query Parameters

```rust
#[derive(Debug, Clone, Serialize)]
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
```

## API Endpoints Reference

### Workflows

| Method | Path | Description |
|--------|------|-------------|
| GET | `/workflows` | List workflows |
| GET | `/workflows/{id}` | Get workflow |
| POST | `/workflows` | Create workflow |
| PUT | `/workflows/{id}` | Update workflow |
| DELETE | `/workflows/{id}` | Delete workflow |
| POST | `/workflows/{id}/activate` | Activate |
| POST | `/workflows/{id}/deactivate` | Deactivate |

### Executions

| Method | Path | Description |
|--------|------|-------------|
| GET | `/executions` | List executions |
| GET | `/executions/{id}` | Get execution |
| DELETE | `/executions/{id}` | Delete execution |
| POST | `/executions/{id}/retry` | Retry execution |
| GET | `/workflows/{id}/execute` | Execute workflow |

### Credentials

| Method | Path | Description |
|--------|------|-------------|
| GET | `/credentials` | List credentials |
| GET | `/credentials/schema/{type}` | Get schema |
| POST | `/credentials` | Create credential |
| PUT | `/credentials/{id}` | Update credential |
| DELETE | `/credentials/{id}` | Delete credential |

### Tags

| Method | Path | Description |
|--------|------|-------------|
| GET | `/tags` | List tags |
| POST | `/tags` | Create tag |
| PUT | `/tags/{id}` | Update tag |
| DELETE | `/tags/{id}` | Delete tag |
| PUT | `/workflows/{id}/tags` | Assign tags |

### Health

| Method | Path | Description |
|--------|------|-------------|
| GET | `/healthz` | Health check |
| GET | `/healthz/readiness` | Readiness check |

## Adding New Endpoints

### 1. Create Endpoint File

```rust
// src/client/endpoints/new_resource.rs

use crate::error::Result;
use crate::models::NewResource;
use super::super::N8nClient;

impl N8nClient {
    pub async fn list_new_resources(&self) -> Result<Vec<NewResource>> {
        self.get("/new-resources").await
    }

    pub async fn get_new_resource(&self, id: &str) -> Result<NewResource> {
        self.get(&format!("/new-resources/{}", id)).await
    }

    // ... more methods
}
```

### 2. Export Module

```rust
// src/client/endpoints/mod.rs

mod credentials;
mod executions;
mod health;
mod new_resource;  // Add this
mod tags;
mod workflows;
```

### 3. Use in Handler

```rust
// src/main.rs

let resources = client.list_new_resources().await?;
```

## Best Practices

### URL Construction

```rust
// Always use format! for path construction
self.get(&format!("/workflows/{}", id)).await

// Never concatenate user input directly
// BAD: self.get(&("/workflows/".to_string() + id)).await
```

### Error Context

```rust
// Add context when appropriate
let workflow = client.get_workflow(&id).await
    .map_err(|e| match e {
        N8nError::Api(ref api_err) if api_err.code == 404 => {
            N8nError::WorkflowNotFound(id.clone())
        }
        _ => e,
    })?;
```

### Query Parameters

```rust
// Use skip_serializing_if for optional params
#[serde(skip_serializing_if = "Option::is_none")]
pub limit: Option<u32>,
```

### Response Types

```rust
// Use specific types for different responses
pub async fn get_workflow(&self, id: &str) -> Result<WorkflowDetail>;  // Full detail
pub async fn list_workflows(&self, ...) -> Result<Vec<Workflow>>;       // Summary
```

---

## See Also

- [Architecture](./architecture.md) - System design
- [Models](./models.md) - Data structures
- [Error Handling](./error-handling.md) - Error patterns
- [Adding Commands](./adding-commands.md) - End-to-end example
