use crate::client::N8nClient;
use crate::error::{N8nError, Result};
use serde::Deserialize;

/// Health check response
#[derive(Debug, Deserialize)]
pub struct HealthResponse {
    pub status: String,
}

impl N8nClient {
    /// Basic health check (/healthz)
    pub async fn health_check(&self) -> Result<HealthResponse> {
        // Health endpoint is at root, not under /api/v1
        let url = format!("{}/healthz", self.base_url());
        let response = reqwest::get(&url).await.map_err(|e| {
            if e.is_connect() {
                N8nError::ConnectionFailed {
                    url: url.clone(),
                    message: e.to_string(),
                }
            } else {
                N8nError::Request(e)
            }
        })?;

        if response.status().is_success() {
            Ok(HealthResponse {
                status: "ok".to_string(),
            })
        } else {
            Ok(HealthResponse {
                status: format!("unhealthy ({})", response.status()),
            })
        }
    }

    /// Readiness check (/healthz/readiness)
    pub async fn readiness_check(&self) -> Result<HealthResponse> {
        let url = format!("{}/healthz/readiness", self.base_url());
        let response = reqwest::get(&url).await.map_err(|e| {
            if e.is_connect() {
                N8nError::ConnectionFailed {
                    url: url.clone(),
                    message: e.to_string(),
                }
            } else {
                N8nError::Request(e)
            }
        })?;

        if response.status().is_success() {
            Ok(HealthResponse {
                status: "ready".to_string(),
            })
        } else {
            Ok(HealthResponse {
                status: format!("not ready ({})", response.status()),
            })
        }
    }
}
