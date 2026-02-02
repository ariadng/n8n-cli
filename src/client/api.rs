use crate::config::Config;
use crate::error::{ApiErrorResponse, N8nError, Result};
use reqwest::{header, Client, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

/// The core n8n API client
pub struct N8nClient {
    http: Client,
    base_url: String,
}

impl N8nClient {
    /// Creates a new client from configuration
    pub fn new(config: &Config) -> Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "X-N8N-API-KEY",
            header::HeaderValue::from_str(&config.api_key).map_err(|_| N8nError::InvalidApiKey)?,
        );
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let http = Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .build()
            .map_err(N8nError::HttpClient)?;

        Ok(Self {
            http,
            base_url: config.base_url.trim_end_matches('/').to_string(),
        })
    }

    /// Builds a URL for an API endpoint
    fn url(&self, path: &str) -> String {
        format!("{}/api/v1{}", self.base_url, path)
    }

    /// GET request with deserialization
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = self.url(path);
        let response = self
            .http
            .get(&url)
            .send()
            .await
            .map_err(|e| self.map_request_error(e, &url))?;

        self.handle_response(response).await
    }

    /// GET request with query parameters
    pub async fn get_with_query<T, Q>(&self, path: &str, query: &Q) -> Result<T>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
    {
        let url = self.url(path);
        let response = self
            .http
            .get(&url)
            .query(query)
            .send()
            .await
            .map_err(|e| self.map_request_error(e, &url))?;

        self.handle_response(response).await
    }

    /// POST request with body
    pub async fn post<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let url = self.url(path);
        let response = self
            .http
            .post(&url)
            .json(body)
            .send()
            .await
            .map_err(|e| self.map_request_error(e, &url))?;

        self.handle_response(response).await
    }

    /// POST request without body (for actions like activate)
    pub async fn post_empty<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = self.url(path);
        let response = self
            .http
            .post(&url)
            .send()
            .await
            .map_err(|e| self.map_request_error(e, &url))?;

        self.handle_response(response).await
    }

    /// PUT request with body
    pub async fn put<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let url = self.url(path);
        let response = self
            .http
            .put(&url)
            .json(body)
            .send()
            .await
            .map_err(|e| self.map_request_error(e, &url))?;

        self.handle_response(response).await
    }

    /// DELETE request
    pub async fn delete(&self, path: &str) -> Result<()> {
        let url = self.url(path);
        let response = self
            .http
            .delete(&url)
            .send()
            .await
            .map_err(|e| self.map_request_error(e, &url))?;

        let status = response.status();
        if status.is_success() || status == StatusCode::NO_CONTENT {
            Ok(())
        } else {
            Err(self.extract_api_error(response).await)
        }
    }

    /// Map request errors to N8nError
    fn map_request_error(&self, error: reqwest::Error, url: &str) -> N8nError {
        if error.is_connect() {
            N8nError::ConnectionFailed {
                url: url.to_string(),
                message: error.to_string(),
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

    /// Handle response and extract result or error
    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        let status = response.status();

        if status.is_success() {
            response.json().await.map_err(N8nError::Deserialize)
        } else {
            Err(self.extract_api_error(response).await)
        }
    }

    /// Extract API error from response
    async fn extract_api_error(&self, response: Response) -> N8nError {
        let status = response.status();

        // Try to get response body as text first
        let body_text = response.text().await.unwrap_or_default();

        // Try to parse as API error, fall back to using raw text
        let error_body: ApiErrorResponse = serde_json::from_str(&body_text)
            .unwrap_or_else(|_| {
                // If the response is JSON with a "message" field
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body_text) {
                    if let Some(msg) = json.get("message").and_then(|m| m.as_str()) {
                        return ApiErrorResponse {
                            code: status.as_u16(),
                            message: msg.to_string(),
                            hint: None,
                        };
                    }
                }
                // Use raw text as message if not empty
                if !body_text.is_empty() && body_text.len() < 500 {
                    ApiErrorResponse {
                        code: status.as_u16(),
                        message: format!("{}: {}", status, body_text),
                        hint: None,
                    }
                } else {
                    ApiErrorResponse::unknown(status)
                }
            });

        N8nError::Api(error_body)
    }

    /// Get base URL for reference
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}
