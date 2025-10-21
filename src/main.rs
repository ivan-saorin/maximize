mod cli;
mod config_loader;
mod oauth;
mod proxy;
mod settings;
mod storage;

use anyhow::Result;
use clap::Parser;
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
}

fn main() -> Result<()> {
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

    // Create and run CLI (CLI manages its own Tokio runtime)
    let mut cli = cli::Cli::new(settings)?;
    cli.run()?;

    Ok(())
}
