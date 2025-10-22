use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Instant;
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::oauth::OAuthManager;
use crate::settings::Settings;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThinkingParameter {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default = "default_budget")]
    pub budget_tokens: i32,
}

fn default_budget() -> i32 {
    16000
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicMessageRequest {
    pub model: String,
    pub messages: Vec<Value>,
    pub max_tokens: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<Value>,
    #[serde(default)]
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<ThinkingParameter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Value>>,
}

#[derive(Clone)]
pub struct AppState {
    pub oauth_manager: Arc<OAuthManager>,
    pub settings: Arc<Settings>,
    pub api_key: Option<String>,
}

fn log_request(request_id: &str, request_data: &AnthropicMessageRequest, headers: &HeaderMap) {
    debug!("[{}] RAW REQUEST CAPTURE", request_id);
    debug!("[{}] Endpoint: /v1/messages", request_id);
    debug!("[{}] Model: {}", request_id, request_data.model);
    debug!("[{}] Stream: {}", request_id, request_data.stream);
    debug!("[{}] Max Tokens: {}", request_id, request_data.max_tokens);

    debug!("[{}] ===== INCOMING HEADERS FROM CLIENT =====", request_id);
    for (name, value) in headers.iter() {
        let header_name = name.as_str();
        if header_name.to_lowercase().contains("authorization")
            || header_name.to_lowercase().contains("api-key")
        {
            debug!("[{}] {}: [REDACTED]", request_id, header_name);
        } else if let Ok(v) = value.to_str() {
            debug!("[{}] {}: {}", request_id, header_name, v);
        }
    }

    if let Some(beta) = headers.get("anthropic-beta") {
        if let Ok(v) = beta.to_str() {
            debug!("[{}] *** ANTHROPIC-BETA HEADER FOUND: {} ***", request_id, v);
        }
    }

    if let Some(thinking) = &request_data.thinking {
        debug!("[{}] THINKING FIELDS DETECTED: {:?}", request_id, thinking);
    }
}

fn sanitize_anthropic_request(mut request_data: AnthropicMessageRequest) -> AnthropicMessageRequest {
    // Universal parameter validation
    if let Some(top_p) = request_data.top_p {
        if top_p.is_nan() || !(0.0..=1.0).contains(&top_p) {
            debug!("Removing invalid top_p value: {}", top_p);
            request_data.top_p = None;
        }
    }

    if let Some(temp) = request_data.temperature {
        if temp.is_nan() {
            debug!("Removing invalid temperature value: {}", temp);
            request_data.temperature = None;
        }
    }

    if let Some(top_k) = request_data.top_k {
        if top_k <= 0 {
            debug!("Removing invalid top_k value: {}", top_k);
            request_data.top_k = None;
        }
    }

    // Handle tools parameter
    if let Some(tools) = &request_data.tools {
        if tools.is_empty() {
            debug!("Removing empty tools list");
            request_data.tools = None;
        }
    }

    // Handle thinking parameter
    if let Some(thinking) = &request_data.thinking {
        if thinking.type_ == "enabled" {
            debug!("Thinking enabled - applying Anthropic API constraints");

            if let Some(temp) = request_data.temperature {
                if (temp - 1.0).abs() > f32::EPSILON {
                    debug!("Adjusting temperature from {} to 1.0 (thinking enabled)", temp);
                    request_data.temperature = Some(1.0);
                }
            }

            if let Some(top_p) = request_data.top_p {
                if !(0.95..=1.0).contains(&top_p) {
                    let adjusted = top_p.max(0.95).min(1.0);
                    debug!("Adjusting top_p from {} to {} (thinking constraints)", top_p, adjusted);
                    request_data.top_p = Some(adjusted);
                }
            }

            if request_data.top_k.is_some() {
                debug!("Removing top_k parameter (not allowed with thinking)");
                request_data.top_k = None;
            }
        }
    }

    request_data
}

fn inject_claude_code_system_message(mut request_data: AnthropicMessageRequest) -> AnthropicMessageRequest {
    let claude_code_spoof_element = json!({
        "type": "text",
        "text": "You are Claude Code, Anthropic's official CLI for Claude.",
        "cache_control": {"type": "ephemeral"}
    });

    match &mut request_data.system {
        Some(system) => {
            if let Value::Array(arr) = system {
                let mut new_arr = vec![claude_code_spoof_element];
                new_arr.extend(arr.clone());
                *arr = new_arr;
            } else if let Value::String(s) = system {
                let existing_element = json!({
                    "type": "text",
                    "text": s.clone(),
                    "cache_control": {"type": "ephemeral"}
                });
                *system = Value::Array(vec![claude_code_spoof_element, existing_element]);
            }
        }
        None => {
            request_data.system = Some(Value::Array(vec![claude_code_spoof_element]));
        }
    }

    debug!("Injected Claude Code system message array for Anthropic authentication bypass");
    if let Some(Value::Array(arr)) = &request_data.system {
        debug!("Final system message array length: {}", arr.len());
    }

    request_data
}

async fn make_anthropic_request(
    request_data: &AnthropicMessageRequest,
    access_token: &str,
    client_beta_headers: Option<&str>,
) -> Result<reqwest::Response, reqwest::Error> {
    let required_betas = vec![
        "claude-code-20250219",
        "oauth-2025-04-20",
        "fine-grained-tool-streaming-2025-05-14",
    ];

    let all_betas = if let Some(client_betas) = client_beta_headers {
        let client_beta_list: Vec<&str> = client_betas.split(',').map(|s| s.trim()).collect();
        let mut combined: Vec<&str> = required_betas.clone();
        combined.extend(client_beta_list);
        combined.sort();
        combined.dedup();
        combined
    } else {
        required_betas
    };

    let beta_header_value = all_betas.join(",");

    let client = reqwest::Client::new();
    client
        .post("https://api.anthropic.com/v1/messages?beta=true")
        .json(request_data)
        .header("host", "api.anthropic.com")
        .header("Accept", "application/json")
        .header("X-Stainless-Retry-Count", "0")
        .header("X-Stainless-Timeout", "600")
        .header("X-Stainless-Lang", "js")
        .header("X-Stainless-Package-Version", "0.60.0")
        .header("X-Stainless-OS", "Windows")
        .header("X-Stainless-Arch", "x64")
        .header("X-Stainless-Runtime", "node")
        .header("X-Stainless-Runtime-Version", "v22.19.0")
        .header("anthropic-dangerous-direct-browser-access", "true")
        .header("anthropic-version", "2023-06-01")
        .header("authorization", format!("Bearer {}", access_token))
        .header("x-app", "cli")
        .header("User-Agent", "claude-cli/1.0.113 (external, cli)")
        .header("content-type", "application/json")
        .header("anthropic-beta", beta_header_value)
        .header("x-stainless-helper-method", "stream")
        .header("accept-language", "*")
        .header("sec-fetch-mode", "cors")
        .send()
        .await
}

pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().timestamp()
    }))
}

pub async fn auth_status(State(state): State<AppState>) -> impl IntoResponse {
    let status = state.oauth_manager.storage().get_status();
    Json(status)
}

pub async fn debug_token(State(state): State<AppState>) -> impl IntoResponse {
    let storage = state.oauth_manager.storage();
    
    // Check if we have a token from environment
    let env_token = std::env::var("MAXIMIZE_ACCESS_TOKEN").ok();
    
    if let Some(token) = env_token {
        let preview = if token.len() > 20 {
            format!("{}...{}", &token[..10], &token[token.len()-10..])
        } else {
            "***".to_string()
        };
        
        let is_valid_format = token.starts_with("sk-ant-") || token.starts_with("sess-");
        
        Json(json!({
            "has_token": true,
            "source": "environment",
            "starts_with": &token[..std::cmp::min(10, token.len())],
            "token_length": token.len(),
            "token_preview": preview,
            "looks_valid": is_valid_format,
            "warning": if !is_valid_format {
                Some("Token format looks wrong! Anthropic tokens should start with 'sk-ant-'")
            } else {
                None
            }
        }))
    } else if let Some(token) = storage.get_access_token() {
        let preview = if token.len() > 20 {
            format!("{}...{}", &token[..10], &token[token.len()-10..])
        } else {
            "***".to_string()
        };
        
        let is_valid_format = token.starts_with("sk-ant-") || token.starts_with("sess-");
        
        Json(json!({
            "has_token": true,
            "source": "file",
            "starts_with": &token[..std::cmp::min(10, token.len())],
            "token_length": token.len(),
            "token_preview": preview,
            "looks_valid": is_valid_format,
            "warning": if !is_valid_format {
                Some("Token format looks wrong! Anthropic tokens should start with 'sk-ant-'")
            } else {
                None
            }
        }))
    } else {
        Json(json!({
            "has_token": false,
            "error": "No token found in environment or file"
        }))
    }
}

pub async fn token_debug(State(state): State<AppState>) -> impl IntoResponse {
    let token = state.oauth_manager.storage().get_access_token();
    
    let token_info = if let Some(t) = token {
        if t.len() > 16 {
            json!({
                "has_token": true,
                "token_preview": format!("{}...{}", &t[..12], &t[t.len()-12..]),
                "token_length": t.len(),
                "starts_with": &t[..10],
                "source": if std::env::var("MAXIMIZE_ACCESS_TOKEN").is_ok() { "environment" } else { "file" }
            })
        } else {
            json!({
                "has_token": true,
                "token_preview": "[too short]",
                "token_length": t.len(),
                "warning": "Token is unusually short"
            })
        }
    } else {
        json!({
            "has_token": false,
            "message": "No token loaded"
        })
    };
    
    Json(token_info)
}

pub async fn anthropic_messages(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(mut request): Json<AnthropicMessageRequest>,
) -> Result<Response, (StatusCode, Json<Value>)> {
    let request_id = Uuid::new_v4().to_string()[..8].to_string();
    let start_time = Instant::now();

    info!("[{}] ===== NEW ANTHROPIC MESSAGES REQUEST =====", request_id);
    log_request(&request_id, &request, &headers);

    // Resolve model nickname to actual model name
    let actual_model = state.settings.resolve_model(&request.model);
    if actual_model != request.model {
        debug!("[{}] Resolved model nickname '{}' to '{}'", request_id, request.model, actual_model);
        request.model = actual_model;
    }

    // Get valid access token with automatic refresh
    let access_token = state
        .oauth_manager
        .get_valid_token()
        .await
        .map_err(|e| {
            error!("[{}] Token refresh error: {}", request_id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": {"message": format!("Token refresh error: {}", e)}})),
            )
        })?
        .ok_or_else(|| {
            error!("[{}] No valid token available", request_id);
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": {"message": "OAuth expired; please authenticate using the CLI"}})),
            )
        })?;
    
    // Debug: Log token info (first/last 8 chars only for security)
    if access_token.len() > 16 {
        info!(
            "[{}] Using access token: {}...{} (length: {})",
            request_id,
            &access_token[..8],
            &access_token[access_token.len()-8..],
            access_token.len()
        );
    } else {
        warn!("[{}] Access token is unusually short: {} chars", request_id, access_token.len());
    }

    // Ensure max_tokens is sufficient if thinking is enabled
    if let Some(thinking) = &request.thinking {
        if thinking.type_ == "enabled" {
            let thinking_budget = thinking.budget_tokens;
            let min_response_tokens = 1024;
            let required_total = thinking_budget + min_response_tokens;
            if request.max_tokens < required_total {
                request.max_tokens = required_total;
                debug!(
                    "[{}] Increased max_tokens to {} (thinking: {} + response: {})",
                    request_id, required_total, thinking_budget, min_response_tokens
                );
            }
        }
    }

    // Sanitize request
    request = sanitize_anthropic_request(request);

    // Inject Claude Code system message
    request = inject_claude_code_system_message(request);

    // Extract client beta headers
    let client_beta_headers = headers
        .get("anthropic-beta")
        .and_then(|v| v.to_str().ok());

    debug!("[{}] FULL REQUEST BODY: {}", request_id, serde_json::to_string_pretty(&request).unwrap_or_default());

    let is_streaming = request.stream;

    match make_anthropic_request(&request, &access_token, client_beta_headers).await {
        Ok(response) => {
            let status = response.status();
            let elapsed_ms = start_time.elapsed().as_millis();

            info!(
                "[{}] Anthropic request completed in {}ms status={}",
                request_id, elapsed_ms, status
            );

            if !status.is_success() {
                let error_text = response.text().await.unwrap_or_default();
                error!("[{}] Anthropic API error {}: {}", request_id, status, error_text);

                let error_json: Value = serde_json::from_str(&error_text)
                    .unwrap_or_else(|_| json!({"error": {"type": "api_error", "message": error_text}}));

                return Err((StatusCode::from_u16(status.as_u16()).unwrap(), Json(error_json)));
            }

            if is_streaming {
                // Handle streaming response
                let stream = response.bytes_stream();
                let body = axum::body::Body::from_stream(stream);

                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "text/event-stream")
                    .header("Cache-Control", "no-cache")
                    .header("Connection", "keep-alive")
                    .body(body)
                    .unwrap())
            } else {
                // Handle non-streaming response
                let body_text = response.text().await.map_err(|e| {
                    error!("[{}] Failed to read response body: {}", request_id, e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": {"message": format!("Failed to read response: {}", e)}})),
                    )
                })?;

                let anthropic_response: Value = serde_json::from_str(&body_text).map_err(|e| {
                    error!("[{}] Failed to parse response JSON: {}", request_id, e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": {"message": format!("Failed to parse response: {}", e)}})),
                    )
                })?;

                let final_elapsed_ms = start_time.elapsed().as_millis();
                info!(
                    "[{}] ===== ANTHROPIC MESSAGES FINISHED ===== Total time: {}ms",
                    request_id, final_elapsed_ms
                );

                Ok(Json(anthropic_response).into_response())
            }
        }
        Err(e) => {
            let final_elapsed_ms = start_time.elapsed().as_millis();
            error!(
                "[{}] Request failed after {}ms: {}",
                request_id, final_elapsed_ms, e
            );
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": {"message": format!("{}", e)}})),
            ))
        }
    }
}

async fn api_key_auth(
    State(state): State<AppState>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<Value>)> {
    // Skip auth check if no API key is configured
    let Some(required_key) = &state.api_key else {
        return Ok(next.run(request).await);
    };

    // Check Authorization header
    let auth_header = headers
        .get("authorization")
        .or_else(|| headers.get("x-api-key"))
        .and_then(|v| v.to_str().ok());

    let provided_key = match auth_header {
        Some(header) => {
            // Support both "Bearer <key>" and direct key formats
            if header.starts_with("Bearer ") {
                header.trim_start_matches("Bearer ")
            } else {
                header
            }
        }
        None => {
            warn!("API request missing authorization header");
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": {
                        "type": "authentication_error",
                        "message": "Missing API key. Provide via Authorization header."
                    }
                })),
            ));
        }
    };

    if provided_key != required_key {
        warn!("API request with invalid API key");
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "error": {
                    "type": "authentication_error",
                    "message": "Invalid API key"
                }
            })),
        ));
    }

    Ok(next.run(request).await)
}

pub fn create_router(state: AppState) -> Router {
    let protected_routes = Router::new()
        .route("/v1/messages", post(anthropic_messages))
        .layer(middleware::from_fn_with_state(state.clone(), api_key_auth));

    Router::new()
        .route("/healthz", get(health_check))
        .route("/auth/status", get(auth_status))
        .route("/debug/token", get(token_debug))  // Debug endpoint
        .merge(protected_routes)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
