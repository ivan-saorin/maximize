mod cli;
mod config_loader;
mod oauth;
mod proxy;
mod settings;
mod storage;

use anyhow::Result;
use clap::Parser;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(name = "maximize")]
#[command(about = "High-performance Anthropic Claude Max Proxy", long_about = None)]
struct Args {
    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,

    /// Override bind address (default: from config)
    #[arg(short, long)]
    bind: Option<String>,

    /// Run in server-only mode (no CLI, for production/containers)
    #[arg(long)]
    server_only: bool,
}

async fn run_server_only(settings: settings::Settings) -> Result<()> {
    use tracing::info;

    let settings = Arc::new(settings);
    let oauth_manager = Arc::new(oauth::OAuthManager::new(&settings.token_file)?);

    // Check for authorization code in environment and exchange it automatically
    if let Ok(auth_code) = std::env::var("MAXIMIZE_AUTHENTICATION_CODE") {
        info!("🔄 Found MAXIMIZE_AUTHENTICATION_CODE, exchanging for tokens...");
        info!("📝 Using code: {}...{}", &auth_code[..20.min(auth_code.len())], if auth_code.len() > 40 { "..." } else { "" });
        
        match oauth_manager.exchange_code(&auth_code).await {
            Ok(_) => {
                info!("✅ Successfully exchanged authorization code for tokens!");
                info!("💡 Tokens saved to: {}", settings.token_file);
                info!("");
                
                // Load and display the tokens so user can set them as env vars
                if let Ok(Some(token_data)) = oauth_manager.storage().load_tokens() {
                    info!("📋 COPY THESE TOKENS TO YOUR ENVIRONMENT VARIABLES:");
                    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    info!("MAXIMIZE_ACCESS_TOKEN=\"{}\"", token_data.access_token);
                    info!("MAXIMIZE_REFRESH_TOKEN=\"{}\"", token_data.refresh_token);
                    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    info!("");
                    info!("💡 After setting these, remove MAXIMIZE_AUTHENTICATION_CODE");
                    info!("💡 Tokens will auto-refresh and persist across restarts!");
                } else {
                    tracing::warn!("Could not read tokens from file to display them");
                }
                info!("");
            }
            Err(e) => {
                tracing::error!("❌ Failed to exchange authorization code: {}", e);
                tracing::error!("");
                tracing::error!("Common issues:");
                tracing::error!("  1. Code has expired (they expire in ~5 minutes)");
                tracing::error!("  2. Code was already used (single-use only)");
                tracing::error!("  3. Code format is wrong (must be: CODE#STATE)");
                tracing::error!("");
                tracing::error!("Solution: Get a FRESH code from the OAuth URL below and use it immediately");
                tracing::error!("");
                // Don't return error, let server start so user can get new OAuth URL
            }
        }
    }

    // Always show OAuth URL for easy access
    info!("🔗 OAuth URL (for authentication):");
    match oauth_manager.get_authorize_url() {
        Ok(auth_url) => {
            info!("   {}", auth_url);
        }
        Err(e) => {
            tracing::error!("Failed to generate auth URL: {}", e);
        }
    }
    info!("");

    // Check for valid tokens (from file or environment)
    let has_tokens = oauth_manager.storage().get_status().has_tokens;
    if !has_tokens {
        tracing::warn!("❌ No tokens found. You need to authenticate first.");
        tracing::warn!("");
        tracing::warn!("📋 After authorizing at the URL above, you can either:");
        tracing::warn!("");
        tracing::warn!("   Option 1 (Easiest - Auto exchange):");
        tracing::warn!("   export MAXIMIZE_AUTHENTICATION_CODE=\"CODE#STATE\"");
        tracing::warn!("   (Server will auto-exchange on restart)");
        tracing::warn!("");
        tracing::warn!("   Option 2 (Manual - Set tokens directly):");
        tracing::warn!("   export MAXIMIZE_ACCESS_TOKEN=\"sk-ant-...\"");
        tracing::warn!("   export MAXIMIZE_REFRESH_TOKEN=\"refresh-...\"");
        tracing::warn!("");
        tracing::warn!("   Option 3 (Interactive - Use CLI):");
        tracing::warn!("   ./maximize → Select option 2 (Login)");
        tracing::warn!("");
    } else {
        info!("✅ Tokens loaded successfully");
    }

    // Log API key status
    if settings.api_key.is_some() {
        info!("🔐 API key authentication: ENABLED");
    } else {
        tracing::warn!("⚠️  API key authentication: DISABLED (set MAXIMIZE_API_KEY to enable)");
    }

    let state = proxy::AppState {
        oauth_manager,
        settings: settings.clone(),
        api_key: settings.api_key.clone(),
    };

    let app = proxy::create_router(state);
    let bind_addr = format!("{}:{}", settings.bind_address, settings.port);

    info!("🚀 Maximize server starting in SERVER-ONLY mode");
    info!("📍 Listening on: {}", bind_addr);
    info!("🔗 Base URL: http://{}", bind_addr);
    info!("📡 Endpoint: /v1/messages");

    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn main() -> Result<()> {
    // Load .env file if it exists (must be first, before any env var access)
    match dotenvy::dotenv() {
        Ok(path) => {
            eprintln!("✅ Loaded .env from: {}", path.display());
        }
        Err(dotenvy::Error::Io(ref io_err)) if io_err.kind() == std::io::ErrorKind::NotFound => {
            // Silently ignore if .env doesn't exist
        }
        Err(e) => {
            eprintln!("⚠️  Warning: Failed to load .env file: {}", e);
        }
    }

    let args = Args::parse();

    // Setup logging
    let log_level = if args.debug { "debug" } else { "info" };
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(log_level));

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load settings
    let mut settings = settings::Settings::load()?;

    // Override bind address if provided
    if let Some(bind) = args.bind {
        settings.bind_address = bind;
    }

    if args.server_only {
        // Run in server-only mode (no CLI)
        tracing::info!("Starting in server-only mode...");
        let rt = Runtime::new()?;
        rt.block_on(run_server_only(settings))?;
    } else {
        // Create and run CLI (CLI manages its own Tokio runtime)
        let mut cli = cli::Cli::new(settings)?;
        cli.run()?;
    }

    Ok(())
}
