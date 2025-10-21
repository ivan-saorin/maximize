use anyhow::{Context, Result};
use base64::{engine::general_purpose, Engine};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;
use url::Url;

use crate::settings::Settings;
use crate::storage::TokenStorage;

#[derive(Debug, Serialize, Deserialize)]
struct PkceData {
    code_verifier: String,
    state: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenRequest {
    code: String,
    state: String,
    grant_type: String,
    client_id: String,
    redirect_uri: String,
    code_verifier: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RefreshRequest {
    grant_type: String,
    refresh_token: String,
    client_id: String,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
    expires_in: Option<i64>,
}

pub struct OAuthManager {
    storage: TokenStorage,
    pkce_file: PathBuf,
}

impl OAuthManager {
    pub fn new(token_file: &str) -> Result<Self> {
        let storage = TokenStorage::new(token_file)?;
        let temp_dir = std::env::temp_dir();
        let pkce_file = temp_dir.join("maximize_oauth_pkce.json");

        Ok(Self { storage, pkce_file })
    }

    fn save_pkce(&self, code_verifier: &str, state: &str) -> Result<()> {
        let data = PkceData {
            code_verifier: code_verifier.to_string(),
            state: state.to_string(),
        };
        let json = serde_json::to_string(&data)?;
        fs::write(&self.pkce_file, json)?;
        Ok(())
    }

    fn load_pkce(&self) -> Result<Option<(String, String)>> {
        if !self.pkce_file.exists() {
            return Ok(None);
        }

        let contents = fs::read_to_string(&self.pkce_file)?;
        let data: PkceData = serde_json::from_str(&contents)?;
        Ok(Some((data.code_verifier, data.state)))
    }

    fn clear_pkce(&self) -> Result<()> {
        if self.pkce_file.exists() {
            fs::remove_file(&self.pkce_file)?;
        }
        Ok(())
    }

    fn generate_pkce(&self) -> (String, String) {
        // Generate high-entropy code_verifier (43-128 chars)
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<u8> = (0..32).map(|_| rng.gen::<u8>()).collect();
        let code_verifier = general_purpose::URL_SAFE_NO_PAD.encode(&random_bytes);

        // Create code_challenge using SHA-256
        let mut hasher = Sha256::new();
        hasher.update(code_verifier.as_bytes());
        let challenge_bytes = hasher.finalize();
        let code_challenge = general_purpose::URL_SAFE_NO_PAD.encode(challenge_bytes);

        (code_verifier, code_challenge)
    }

    pub fn get_authorize_url(&self) -> Result<String> {
        let (code_verifier, code_challenge) = self.generate_pkce();
        // OpenCode uses the verifier as the state
        let state = code_verifier.clone();

        // Save PKCE values for later use
        self.save_pkce(&code_verifier, &state)?;

        let mut url = Url::parse(&format!(
            "{}/oauth/authorize",
            Settings::auth_base_authorize()
        ))?;

        url.query_pairs_mut()
            .append_pair("code", "true")
            .append_pair("client_id", Settings::client_id())
            .append_pair("response_type", "code")
            .append_pair("redirect_uri", Settings::redirect_uri())
            .append_pair("scope", Settings::scopes())
            .append_pair("code_challenge", &code_challenge)
            .append_pair("code_challenge_method", "S256")
            .append_pair("state", &state);

        Ok(url.to_string())
    }

    pub fn start_login_flow(&self) -> Result<String> {
        let auth_url = self.get_authorize_url()?;

        // Open the authorization URL in the default browser
        if let Err(e) = webbrowser::open(&auth_url) {
            tracing::warn!("Failed to open browser: {}", e);
        }

        Ok(auth_url)
    }

    pub async fn exchange_code(&self, code: &str) -> Result<()> {
        // Split the code and state (they come as "code#state")
        let parts: Vec<&str> = code.split('#').collect();
        let actual_code = parts[0];
        let state = parts.get(1).map(|s| s.to_string());

        // Load saved PKCE verifier
        let (code_verifier, saved_state) = self
            .load_pkce()?
            .context("No PKCE verifier found. Start login flow first.")?;

        let state = state.unwrap_or(saved_state);

        let client = reqwest::Client::new();
        let response = client
            .post(&format!("{}/v1/oauth/token", Settings::auth_base_token()))
            .json(&TokenRequest {
                code: actual_code.to_string(),
                state,
                grant_type: "authorization_code".to_string(),
                client_id: Settings::client_id().to_string(),
                redirect_uri: Settings::redirect_uri().to_string(),
                code_verifier,
            })
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("Token exchange failed: {}", error_text);
        }

        let token_data: TokenResponse = response.json().await?;

        // Store tokens securely
        self.storage.save_tokens(
            &token_data.access_token,
            &token_data.refresh_token,
            token_data.expires_in.unwrap_or(3600),
        )?;

        // Clear PKCE values after successful exchange
        self.clear_pkce()?;

        Ok(())
    }

    pub async fn refresh_tokens(&self) -> Result<bool> {
        let refresh_token = match self.storage.get_refresh_token() {
            Some(token) => token,
            None => {
                tracing::warn!("No refresh token available for refresh");
                return Ok(false);
            }
        };

        tracing::info!("Attempting to refresh OAuth tokens...");

        let client = reqwest::Client::new();
        let response = client
            .post(&format!("{}/v1/oauth/token", Settings::auth_base_token()))
            .json(&RefreshRequest {
                grant_type: "refresh_token".to_string(),
                refresh_token,
                client_id: Settings::client_id().to_string(),
            })
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            tracing::error!("Token refresh failed: {}", error_text);
            return Ok(false);
        }

        let token_data: TokenResponse = response.json().await?;

        // Update stored tokens
        self.storage.save_tokens(
            &token_data.access_token,
            &token_data.refresh_token,
            token_data.expires_in.unwrap_or(3600),
        )?;

        tracing::info!("Successfully refreshed OAuth tokens");
        Ok(true)
    }

    pub async fn get_valid_token(&self) -> Result<Option<String>> {
        if !self.storage.is_token_expired() {
            return Ok(self.storage.get_access_token());
        }

        tracing::info!("Token expired, attempting automatic refresh...");

        if self.refresh_tokens().await? {
            Ok(self.storage.get_access_token())
        } else {
            tracing::error!("Failed to refresh token automatically");
            Ok(None)
        }
    }

    pub fn storage(&self) -> &TokenStorage {
        &self.storage
    }
}
