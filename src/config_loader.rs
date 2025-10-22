use anyhow::Result;
use serde_json::Value;
use std::env;
use std::fs;
use std::path::Path;

use crate::settings::{ApiConfig, Config, ModelConfig, ServerConfig, StorageConfig};

/// Expand tilde (~) in paths to home directory
fn expand_tilde(path: &str) -> String {
    if path.starts_with("~/") || path == "~" {
        if let Some(home) = dirs::home_dir() {
            let remainder = &path[1..];
            return home.join(remainder).to_string_lossy().to_string();
        }
    }
    path.to_string()
}

pub struct ConfigLoader {
    config_data: Value,
}

impl ConfigLoader {
    pub fn new(config_path: Option<&str>) -> Result<Self> {
        let path = config_path.unwrap_or("config.json");
        let config_data = Self::load_config_file(path)?;

        Ok(Self { config_data })
    }

    fn load_config_file(path: &str) -> Result<Value> {
        let config_path = Path::new(path);
        
        if !config_path.exists() {
            return Ok(Value::Object(serde_json::Map::new()));
        }

        // Check if it's a directory
        if config_path.is_dir() {
            eprintln!("Warning: '{}' is a directory, not a config file. Skipping.", path);
            return Ok(Value::Object(serde_json::Map::new()));
        }

        let contents = fs::read_to_string(path)?;
        let json: Value = serde_json::from_str(&contents)?;
        Ok(json)
    }

    fn get_nested_value(&self, path: &str) -> Option<&Value> {
        let keys: Vec<&str> = path.split('.').collect();
        let mut current = &self.config_data;

        for key in keys {
            match current {
                Value::Object(map) => {
                    current = map.get(key)?;
                }
                _ => return None,
            }
        }

        Some(current)
    }

    pub fn get_string(&self, env_var: &str, config_path: &str, default: &str) -> String {
        // 1. Check environment variable
        if let Ok(value) = env::var(env_var) {
            return value;
        }

        // 2. Check config.json
        if let Some(value) = self.get_nested_value(config_path) {
            if let Some(s) = value.as_str() {
                return s.to_string();
            }
        }

        // 3. Return default
        default.to_string()
    }

    pub fn get_u16(&self, env_var: &str, config_path: &str, default: u16) -> u16 {
        // 1. Check environment variable
        if let Ok(value) = env::var(env_var) {
            if let Ok(num) = value.parse() {
                return num;
            }
        }

        // 2. Check config.json
        if let Some(value) = self.get_nested_value(config_path) {
            if let Some(num) = value.as_u64() {
                return num as u16;
            }
        }

        // 3. Return default
        default
    }

    pub fn get_u64(&self, env_var: &str, config_path: &str, default: u64) -> u64 {
        // 1. Check environment variable
        if let Ok(value) = env::var(env_var) {
            if let Ok(num) = value.parse() {
                return num;
            }
        }

        // 2. Check config.json
        if let Some(value) = self.get_nested_value(config_path) {
            if let Some(num) = value.as_u64() {
                return num;
            }
        }

        // 3. Return default
        default
    }

    pub fn load() -> Result<Config> {
        let loader = Self::new(None)?;

        let server = ServerConfig {
            port: loader.get_u16("PORT", "server.port", 8081),
            log_level: loader.get_string("LOG_LEVEL", "server.log_level", "info"),
            bind_address: loader.get_string("BIND_ADDRESS", "server.bind_address", "0.0.0.0"),
        };

        let models = ModelConfig {
            default: loader.get_string("DEFAULT_MODEL", "models.default", "l"),
        };

        let api = ApiConfig {
            request_timeout: loader.get_u64("REQUEST_TIMEOUT", "api.request_timeout", 120),
        };

        let storage_default = StorageConfig::default();
        let token_file_raw = loader.get_string("TOKEN_FILE", "storage.token_file", &storage_default.token_file);
        let mut token_file = expand_tilde(&token_file_raw);
        
        // Validate token file path - if it's a directory, append default filename
        let token_path = Path::new(&token_file);
        if token_path.exists() && token_path.is_dir() {
            eprintln!("Warning: TOKEN_FILE '{}' is a directory. Using '{}/tokens.json' instead.", token_file, token_file);
            token_file = token_path.join("tokens.json").to_string_lossy().to_string();
        } else if token_file.ends_with('/') || token_file.ends_with('\\') {
            eprintln!("Warning: TOKEN_FILE '{}' appears to be a directory path. Using '{}tokens.json' instead.", token_file, token_file);
            token_file.push_str("tokens.json");
        }
        
        let storage = StorageConfig {
            token_file,
        };

        Ok(Config {
            server,
            models,
            api,
            storage,
        })
    }
}
