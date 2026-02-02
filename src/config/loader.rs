use crate::error::{N8nError, Result};
use crate::output::OutputFormat;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Runtime configuration (fully resolved)
#[derive(Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub api_key: String,
    pub output_format: OutputFormat,
    pub timeout_secs: u64,
    pub verbose: bool,
    pub quiet: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:5678".to_string(),
            api_key: String::new(),
            output_format: OutputFormat::Table,
            timeout_secs: 30,
            verbose: false,
            quiet: false,
        }
    }
}

/// Configuration file structure
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ConfigFile {
    pub default_profile: Option<String>,
    pub output_format: Option<OutputFormat>,
    pub timeout_secs: Option<u64>,
    #[serde(default)]
    pub profiles: HashMap<String, Profile>,
}

/// Named profile configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Profile {
    pub base_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_env: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<OutputFormat>,
}

/// CLI options that can override configuration
pub struct CliOverrides {
    pub profile: Option<String>,
    pub url: Option<String>,
    pub api_key: Option<String>,
    pub output: OutputFormat,
    pub verbose: bool,
    pub quiet: bool,
}

/// Get the default config file path
pub fn config_file_path() -> Option<PathBuf> {
    ProjectDirs::from("", "", "n8n-cli").map(|dirs| dirs.config_dir().join("config.toml"))
}

/// Load configuration with layering: defaults → file → env → CLI
pub fn load_config(overrides: CliOverrides) -> Result<Config> {
    let mut config = Config::default();

    // Layer 1: Load config file if it exists
    if let Some(config_path) = config_file_path() {
        if config_path.exists() {
            let contents = std::fs::read_to_string(&config_path).map_err(N8nError::ConfigFileRead)?;
            let file_config: ConfigFile =
                toml::from_str(&contents).map_err(N8nError::ConfigFileParse)?;

            // Apply file-level defaults
            if let Some(fmt) = file_config.output_format {
                config.output_format = fmt;
            }
            if let Some(timeout) = file_config.timeout_secs {
                config.timeout_secs = timeout;
            }

            // Determine which profile to use
            let profile_name = overrides
                .profile
                .clone()
                .or(std::env::var("N8N_PROFILE").ok())
                .or(file_config.default_profile);

            // Apply profile settings
            if let Some(name) = profile_name {
                if let Some(profile) = file_config.profiles.get(&name) {
                    config.base_url = profile.base_url.clone();

                    // Resolve API key from profile
                    if let Some(key) = &profile.api_key {
                        config.api_key = key.clone();
                    } else if let Some(env_var) = &profile.api_key_env {
                        if let Ok(key) = std::env::var(env_var) {
                            config.api_key = key;
                        }
                    }

                    if let Some(fmt) = profile.output_format {
                        config.output_format = fmt;
                    }
                } else {
                    return Err(N8nError::ProfileNotFound(name));
                }
            }
        }
    }

    // Layer 2: Environment variables override file settings
    if let Ok(url) = std::env::var("N8N_BASE_URL") {
        config.base_url = url;
    }
    if let Ok(key) = std::env::var("N8N_API_KEY") {
        config.api_key = key;
    }

    // Layer 3: CLI arguments override everything
    if let Some(url) = overrides.url {
        config.base_url = url;
    }
    if let Some(key) = overrides.api_key {
        config.api_key = key;
    }

    // Output format from CLI always wins (it has a default value)
    config.output_format = overrides.output;
    config.verbose = overrides.verbose;
    config.quiet = overrides.quiet;

    Ok(config)
}

/// Validate that required configuration is present
pub fn validate_config(config: &Config) -> Result<()> {
    if config.api_key.is_empty() {
        return Err(N8nError::MissingApiKey);
    }
    if config.base_url.is_empty() {
        return Err(N8nError::MissingBaseUrl);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.base_url, "http://localhost:5678");
        assert!(config.api_key.is_empty());
    }
}
