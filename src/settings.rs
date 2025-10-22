use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub log_level: String,
    pub bind_address: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 8081,
            log_level: "info".to_string(),
            bind_address: "0.0.0.0".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub default: String,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            default: "l".to_string(), // Default to claude-sonnet-4
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub request_timeout: u64,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            request_timeout: 120,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub token_file: String,
}

impl Default for StorageConfig {
    fn default() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        let token_path = home_dir
            .join(".maximize")
            .join("tokens.json");

        Self {
            token_file: token_path.to_string_lossy().to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub server: ServerConfig,
    pub models: ModelConfig,
    pub api: ApiConfig,
    pub storage: StorageConfig,
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub port: u16,
    pub log_level: String,
    pub bind_address: String,
    pub default_model: String,
    pub request_timeout: u64,
    pub token_file: String,
    pub model_map: HashMap<String, String>,
    pub api_key: Option<String>,
}

impl Settings {
    pub fn load() -> anyhow::Result<Self> {
        let config = crate::config_loader::ConfigLoader::load()?;

        // Create model nickname mapping
        let mut model_map = HashMap::new();
        model_map.insert("xs".to_string(), "claude-3-5-haiku-20241022".to_string());
        model_map.insert("s".to_string(), "claude-3-5-sonnet-20241022".to_string());
        model_map.insert("m".to_string(), "claude-3-7-sonnet-20250219".to_string());
        model_map.insert("l".to_string(), "claude-sonnet-4-20250514".to_string());
        model_map.insert("xl".to_string(), "claude-opus-4-20250514".to_string());
        model_map.insert("xxl".to_string(), "claude-opus-4-1-20250805".to_string());

        // Load API key from environment
        let api_key = std::env::var("MAXIMIZE_API_KEY").ok();

        Ok(Self {
            port: config.server.port,
            log_level: config.server.log_level.clone(),
            bind_address: config.server.bind_address.clone(),
            default_model: config.models.default.clone(),
            request_timeout: config.api.request_timeout,
            token_file: config.storage.token_file.clone(),
            model_map,
            api_key,
        })
    }

    pub fn resolve_model(&self, nickname: &str) -> String {
        self.model_map
            .get(nickname)
            .cloned()
            .unwrap_or_else(|| nickname.to_string())
    }

    // Constants (not user configurable)
    pub fn anthropic_version() -> &'static str {
        "2023-06-01"
    }

    pub fn anthropic_beta() -> &'static str {
        "claude-code-20250219,oauth-2025-04-20,fine-grained-tool-streaming-2025-05-14"
    }

    pub fn api_base() -> &'static str {
        "https://api.anthropic.com"
    }

    pub fn auth_base_authorize() -> &'static str {
        "https://claude.ai"
    }

    pub fn auth_base_token() -> &'static str {
        "https://console.anthropic.com"
    }

    pub fn client_id() -> &'static str {
        "9d1c250a-e61b-44d9-88ed-5944d1962f5e"
    }

    pub fn redirect_uri() -> &'static str {
        "https://console.anthropic.com/oauth/code/callback"
    }

    pub fn scopes() -> &'static str {
        "org:create_api_key user:profile user:inference"
    }
}
