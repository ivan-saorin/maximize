use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenData {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenStatus {
    pub has_tokens: bool,
    pub is_expired: bool,
    pub expires_at: Option<String>,
    pub time_until_expiry: String,
    pub expires_in_seconds: Option<i64>,
}

pub struct TokenStorage {
    token_path: PathBuf,
}

impl TokenStorage {
    pub fn new(token_file: &str) -> Result<Self> {
        let token_path = PathBuf::from(token_file);
        
        // Additional validation: token_path should not be a directory
        if token_path.exists() && token_path.is_dir() {
            anyhow::bail!(
                "Token file path '{}' is a directory. Please specify a file path like: {}{}tokens.json",
                token_path.display(),
                token_path.display(),
                std::path::MAIN_SEPARATOR
            );
        }
        
        let storage = Self { token_path };
        storage.ensure_secure_directory()?;
        Ok(storage)
    }

    fn ensure_secure_directory(&self) -> Result<()> {
        if let Some(parent) = self.token_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).context("Failed to create token directory")?;

                #[cfg(unix)]
                {
                    let metadata = fs::metadata(parent)?;
                    let mut permissions = metadata.permissions();
                    permissions.set_mode(0o700);
                    fs::set_permissions(parent, permissions)?;
                }
            }
        }
        Ok(())
    }

    pub fn save_tokens(&self, access_token: &str, refresh_token: &str, expires_in: i64) -> Result<()> {
        let expires_at = Utc::now().timestamp() + expires_in;
        let data = TokenData {
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
            expires_at,
        };

        let json = serde_json::to_string_pretty(&data)?;
        fs::write(&self.token_path, json)?;

        #[cfg(unix)]
        {
            let metadata = fs::metadata(&self.token_path)?;
            let mut permissions = metadata.permissions();
            permissions.set_mode(0o600);
            fs::set_permissions(&self.token_path, permissions)?;
        }

        Ok(())
    }

    fn try_load_from_file(&self) -> Result<Option<TokenData>> {
        if !self.token_path.exists() {
            return Ok(None);
        }

        // Check if path is a directory (common misconfiguration)
        if self.token_path.is_dir() {
            anyhow::bail!(
                "Token file path '{}' is a directory. Expected a file path like: {}{}tokens.json",
                self.token_path.display(),
                self.token_path.display(),
                std::path::MAIN_SEPARATOR
            );
        }

        let contents = fs::read_to_string(&self.token_path)
            .context(format!("Failed to read token file: {}", self.token_path.display()))?;
        let data: TokenData = serde_json::from_str(&contents)
            .context("Failed to parse token file as JSON")?;
        
        tracing::debug!("Loading tokens from file: {}", self.token_path.display());
        tracing::debug!("File token expires at: {}", data.expires_at);
        
        Ok(Some(data))
    }

    fn save_token_data(&self, data: &TokenData) -> Result<()> {
        let json = serde_json::to_string_pretty(data)?;
        fs::write(&self.token_path, json)?;

        #[cfg(unix)]
        {
            let metadata = fs::metadata(&self.token_path)?;
            let mut permissions = metadata.permissions();
            permissions.set_mode(0o600);
            fs::set_permissions(&self.token_path, permissions)?;
        }

        Ok(())
    }

    pub fn load_tokens(&self) -> Result<Option<TokenData>> {
        // First, try loading from environment variables (for containerized deployments)
        // But ONLY if both are set AND non-empty
        if let (Ok(access_token), Ok(refresh_token)) = (
            std::env::var("MAXIMIZE_ACCESS_TOKEN"),
            std::env::var("MAXIMIZE_REFRESH_TOKEN"),
        ) {
            // Only use env vars if they're actually populated (not empty strings)
            if !access_token.trim().is_empty() && !refresh_token.trim().is_empty() {
                // Try to load existing token data from file to preserve expiry time
                // This ensures we don't reset the expiry on every load
                if let Ok(Some(existing)) = self.try_load_from_file() {
                    // If we have existing data with the same tokens, use its expiry
                    if existing.access_token == access_token {
                        tracing::debug!("Loading tokens from environment variables (preserving existing expiry)");
                        return Ok(Some(existing));
                    }
                }
                
                // New tokens from env vars - get expiry timestamp
                // First check if we have an absolute expiry timestamp (preferred)
                let expires_at = if let Ok(expires_at_str) = std::env::var("MAXIMIZE_TOKEN_EXPIRES_AT") {
                    // Use absolute timestamp if provided
                    match expires_at_str.parse::<i64>() {
                        Ok(ts) => {
                            tracing::debug!("Using absolute MAXIMIZE_TOKEN_EXPIRES_AT: {}", ts);
                            ts
                        }
                        Err(_) => {
                            tracing::warn!("Invalid MAXIMIZE_TOKEN_EXPIRES_AT value, falling back to expires_in");
                            let expires_in = std::env::var("MAXIMIZE_TOKEN_EXPIRES_IN")
                                .ok()
                                .and_then(|v| v.parse::<i64>().ok())
                                .unwrap_or(86400);
                            Utc::now().timestamp() + expires_in
                        }
                    }
                } else {
                    // Fall back to relative expires_in (unreliable after restart!)
                    tracing::warn!("No MAXIMIZE_TOKEN_EXPIRES_AT set, calculating from now (may be incorrect after restart)");
                    let expires_in = std::env::var("MAXIMIZE_TOKEN_EXPIRES_IN")
                        .ok()
                        .and_then(|v| v.parse::<i64>().ok())
                        .unwrap_or(86400); // Default 24 hours
                    Utc::now().timestamp() + expires_in
                };
                
                let now = Utc::now().timestamp();
                let time_until_expiry = expires_at - now;
                tracing::debug!("Loading NEW tokens from environment variables");
                tracing::debug!("Token expires at: {} (in {} seconds, ~{} hours)", expires_at, time_until_expiry, time_until_expiry / 3600);
                
                let token_data = TokenData {
                    access_token,
                    refresh_token,
                    expires_at,
                };
                
                // Save to file to persist expiry time
                if let Err(e) = self.save_token_data(&token_data) {
                    tracing::warn!("Failed to persist env token data to file: {}", e);
                }
                
                return Ok(Some(token_data));
            } else {
                tracing::debug!("Environment variables set but empty, falling back to file");
            }
        }

        tracing::debug!("No environment variables found, trying file: {}", self.token_path.display());

        // Fall back to file-based token storage
        self.try_load_from_file()
    }

    pub fn clear_tokens(&self) -> Result<()> {
        if self.token_path.exists() {
            fs::remove_file(&self.token_path)?;
        }
        Ok(())
    }

    pub fn is_token_expired(&self) -> bool {
        match self.load_tokens() {
            Ok(Some(tokens)) => {
                let now = Utc::now().timestamp();
                // Add 60 second buffer before expiry
                now >= (tokens.expires_at - 60)
            }
            _ => true,
        }
    }

    pub fn get_access_token(&self) -> Option<String> {
        if self.is_token_expired() {
            return None;
        }

        self.load_tokens()
            .ok()
            .flatten()
            .map(|t| t.access_token)
    }

    pub fn get_refresh_token(&self) -> Option<String> {
        self.load_tokens()
            .ok()
            .flatten()
            .map(|t| t.refresh_token)
    }

    pub fn get_status(&self) -> TokenStatus {
        match self.load_tokens() {
            Ok(Some(tokens)) => {
                let now = Utc::now().timestamp();
                let expires_at = DateTime::from_timestamp(tokens.expires_at, 0)
                    .map(|dt| dt.to_rfc3339());

                if now >= tokens.expires_at {
                    let time_since = now - tokens.expires_at;
                    let hours_since = time_since / 3600;
                    let mins_since = (time_since % 3600) / 60;

                    let time_str = if hours_since > 0 {
                        format!("{}h {}m ago", hours_since, mins_since)
                    } else {
                        format!("{}m ago", mins_since)
                    };

                    TokenStatus {
                        has_tokens: true,
                        is_expired: true,
                        expires_at,
                        time_until_expiry: time_str,
                        expires_in_seconds: None,
                    }
                } else {
                    let time_remaining = tokens.expires_at - now;
                    let hours = time_remaining / 3600;
                    let minutes = (time_remaining % 3600) / 60;

                    let time_str = if hours > 0 {
                        format!("{}h {}m", hours, minutes)
                    } else {
                        format!("{}m", minutes)
                    };

                    TokenStatus {
                        has_tokens: true,
                        is_expired: false,
                        expires_at,
                        time_until_expiry: time_str,
                        expires_in_seconds: Some(time_remaining),
                    }
                }
            }
            _ => TokenStatus {
                has_tokens: false,
                is_expired: true,
                expires_at: None,
                time_until_expiry: "No tokens".to_string(),
                expires_in_seconds: None,
            },
        }
    }

    pub fn token_file(&self) -> &Path {
        &self.token_path
    }
}
