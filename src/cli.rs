use anyhow::Result;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::io;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;

use crate::oauth::OAuthManager;
use crate::proxy::{create_router, AppState};
use crate::settings::Settings;

pub struct Cli {
    oauth_manager: Arc<OAuthManager>,
    settings: Arc<Settings>,
    rt: Runtime,
    server_handle: Option<thread::JoinHandle<()>>,
}

impl Cli {
    pub fn new(settings: Settings) -> Result<Self> {
        let oauth_manager = Arc::new(OAuthManager::new(&settings.token_file)?);
        let settings = Arc::new(settings);
        let rt = Runtime::new()?;

        Ok(Self {
            oauth_manager,
            settings,
            rt,
            server_handle: None,
        })
    }

    fn clear_screen(&self) {
        let _ = Term::stdout().clear_screen();
    }

    fn display_header(&self) {
        println!("{}", "=".repeat(50));
        println!("    {}", style("Maximize - Anthropic Claude Proxy").bold());
        println!("{}", "=".repeat(50));
    }

    fn get_auth_status(&self) -> (String, String) {
        let status = self.oauth_manager.storage().get_status();

        if !status.has_tokens {
            return ("NO AUTH".to_string(), "No tokens available".to_string());
        }

        if status.is_expired {
            return ("EXPIRED".to_string(), format!("Expired {}", status.time_until_expiry));
        }

        ("VALID".to_string(), format!("Expires in {}", status.time_until_expiry))
    }

    fn display_menu(&self, server_running: bool) {
        let (auth_status, auth_detail) = self.get_auth_status();

        let status_style = match auth_status.as_str() {
            "VALID" => style(&auth_status).green(),
            "EXPIRED" => style(&auth_status).yellow(),
            _ => style(&auth_status).red(),
        };

        println!(" Auth Status: {} ({})", status_style, auth_detail);

        if server_running {
            println!(
                " Server Status: {} at http://{}:{}",
                style("RUNNING").green(),
                self.settings.bind_address,
                self.settings.port
            );
        } else {
            println!(" Server Status: {}", style("STOPPED").dim());
        }

        println!("{}", "-".repeat(50));
    }

    fn show_token_status(&self) {
        let status = self.oauth_manager.storage().get_status();

        println!("\n{}", style("Token Status Details").cyan().bold());
        println!("{}", "-".repeat(50));
        println!("Has Tokens: {}", if status.has_tokens { "Yes" } else { "No" });
        println!("Is Expired: {}", if status.is_expired { "Yes" } else { "No" });

        if let Some(expires_at) = status.expires_at {
            println!("Expires At: {}", expires_at);
            println!("Time Until Expiry: {}", status.time_until_expiry);
        }

        println!("Token File: {}", self.oauth_manager.storage().token_file().display());
        println!("\nPress Enter to continue...");
        let _ = io::stdin().read_line(&mut String::new());
    }

    fn start_proxy_server(&mut self, retry_count: usize) -> Result<bool> {
        const MAX_RETRIES: usize = 3;

        if self.server_handle.is_some() {
            println!("{}", style("Server is already running").yellow());
            return Ok(false);
        }

        // Check authentication with automatic refresh
        let (auth_ok, auth_status, message) = self.check_and_refresh_auth();

        if !auth_ok {
            println!("{} {}", style("ERROR:").red(), message);

            if auth_status == "NETWORK_ERROR" && retry_count < MAX_RETRIES {
                println!(
                    "\n{} Retry attempt {} of {}",
                    style("⚠").yellow(),
                    retry_count + 1,
                    MAX_RETRIES
                );
                println!("\nWould you like to:");
                println!("1. Retry token refresh");
                println!("2. Return to main menu");

                let choice: usize = Input::new()
                    .with_prompt("Select option")
                    .validate_with(|input: &String| -> Result<(), &str> {
                        match input.as_str() {
                            "1" | "2" => Ok(()),
                            _ => Err("Please enter 1 or 2"),
                        }
                    })
                    .interact_text()
                    .unwrap_or_else(|_| "2".to_string())
                    .parse()
                    .unwrap_or(2);

                if choice == 1 {
                    return self.start_proxy_server(retry_count + 1);
                }
            } else if auth_status == "NETWORK_ERROR" {
                println!(
                    "\n{} Maximum retry attempts ({}) reached.",
                    style("✗").red(),
                    MAX_RETRIES
                );
                println!("Please check your network connection and try again later.");
            }

            println!("\nPress Enter to continue...");
            let _ = io::stdin().read_line(&mut String::new());
            return Ok(false);
        }

        if auth_status == "REFRESHED" {
            println!("{} {}", style("✓").green(), message);
        }

        println!("Starting proxy server...");

        let oauth_manager = Arc::clone(&self.oauth_manager);
        let settings = Arc::clone(&self.settings);
        let bind_addr = format!("{}:{}", settings.bind_address, settings.port);

        let handle = thread::spawn(move || {
            let rt = Runtime::new().expect("Failed to create runtime");
            rt.block_on(async {
                let state = AppState {
                    oauth_manager,
                    settings: settings.clone(),
                    api_key: settings.api_key.clone(),
                };

                let app = create_router(state);
                let listener = tokio::net::TcpListener::bind(&bind_addr)
                    .await
                    .expect("Failed to bind");

                tracing::info!("Proxy server listening on {}", bind_addr);

                axum::serve(listener, app)
                    .await
                    .expect("Server error");
            });
        });

        self.server_handle = Some(handle);

        // Wait for server to start
        thread::sleep(Duration::from_secs(1));

        println!(
            "{} Proxy running at http://{}:{}",
            style("✓").green(),
            self.settings.bind_address,
            self.settings.port
        );
        println!("\nBase URL: http://{}:{}", self.settings.bind_address, self.settings.port);
        println!("API Key: any-placeholder-string");
        println!("Endpoint: /v1/messages");

        println!("\nPress Enter to continue...");
        let _ = io::stdin().read_line(&mut String::new());

        Ok(true)
    }

    fn check_and_refresh_auth(&self) -> (bool, String, String) {
        let status = self.oauth_manager.storage().get_status();

        if !status.has_tokens {
            return (
                false,
                "NO_AUTH".to_string(),
                "No authentication tokens found. Please login first (option 2)".to_string(),
            );
        }

        if !status.is_expired {
            return (
                true,
                "VALID".to_string(),
                format!("Token valid for: {}", status.time_until_expiry),
            );
        }

        let refresh_token = self.oauth_manager.storage().get_refresh_token();
        if refresh_token.is_none() {
            return (
                false,
                "NO_REFRESH".to_string(),
                "Token expired and no refresh token available. Please login again (option 2)".to_string(),
            );
        }

        println!("{} Token expired, attempting automatic refresh...", style("⚠").yellow());

        match self.rt.block_on(self.oauth_manager.refresh_tokens()) {
            Ok(true) => {
                let new_status = self.oauth_manager.storage().get_status();
                (
                    true,
                    "REFRESHED".to_string(),
                    format!("Automatically refreshed expired token. Token valid for: {}", new_status.time_until_expiry),
                )
            }
            Ok(false) => (
                false,
                "REFRESH_FAILED".to_string(),
                "Refresh token invalid or expired. Please login again (option 2)".to_string(),
            ),
            Err(_) => (
                false,
                "NETWORK_ERROR".to_string(),
                "Network error during token refresh. Check connection and retry".to_string(),
            ),
        }
    }

    fn stop_proxy_server(&mut self) {
        if self.server_handle.is_none() {
            println!("{}", style("Server is not running").yellow());
            println!("\nPress Enter to continue...");
            let _ = io::stdin().read_line(&mut String::new());
            return;
        }

        println!("Stopping proxy server...");
        // The server will stop when the handle is dropped
        self.server_handle = None;
        println!("{} Server stopped", style("✓").green());

        println!("\nPress Enter to continue...");
        let _ = io::stdin().read_line(&mut String::new());
    }

    fn login(&self) {
        println!("Starting OAuth login flow...");

        match self.oauth_manager.start_login_flow() {
            Ok(auth_url) => {
                println!("{} Browser opened successfully", style("✓").green());
                println!("\n{}", style("If browser didn't open, use this URL:").yellow());
                println!("{}", style(&auth_url).cyan().underlined());
            }
            Err(e) => {
                println!("{} Could not open browser: {}", style("⚠").yellow(), e);
                
                // Still try to get the URL
                if let Ok(auth_url) = self.oauth_manager.get_authorize_url() {
                    println!("\n{}", style("Please open this URL in your browser:").yellow().bold());
                    println!("{}", style(&auth_url).cyan().underlined());
                } else {
                    println!("{} Failed to generate authorization URL", style("✗").red());
                    println!("\nPress Enter to continue...");
                    let _ = io::stdin().read_line(&mut String::new());
                    return;
                }
            }
        }

        println!("\n{} Complete the login process in your browser", style("Step 1:").bold());
        println!("  1. Login to your Claude Pro/Max account if prompted");
        println!("  2. Authorize the application");
        println!("  3. You will see an authorization code on the Anthropic page");

        println!("\n{} Paste the authorization code below", style("Step 2:").bold());
        println!("{}", style("The code should look like: CODE#STATE").dim());

        let code: String = Input::new()
            .with_prompt("\nAuthorization code")
            .interact_text()
            .unwrap_or_default();

        if code.trim().is_empty() || code.trim().len() < 10 {
            println!("{} Invalid or missing code. Please paste the complete code from the browser.", style("✗").red());
            println!("\nPress Enter to continue...");
            let _ = io::stdin().read_line(&mut String::new());
            return;
        }

        println!("\n{} Exchanging code for tokens...", style("Step 3:").bold());

        match self.rt.block_on(self.oauth_manager.exchange_code(&code.trim())) {
            Ok(_) => {
                println!("{} Tokens obtained successfully", style("✓").green());
                let status = self.oauth_manager.storage().get_status();
                if let Some(expires_at) = status.expires_at {
                    println!("Token expires at: {}", expires_at);
                }
            }
            Err(e) => {
                println!("{} Failed to exchange code for tokens: {}", style("✗").red(), e);
            }
        }

        println!("\nPress Enter to continue...");
        let _ = io::stdin().read_line(&mut String::new());
    }

    fn refresh_token(&self) {
        println!("Attempting to refresh token...");

        if self.oauth_manager.storage().get_refresh_token().is_none() {
            println!("{} No refresh token available - please login first", style("✗").red());
            println!("\nPress Enter to continue...");
            let _ = io::stdin().read_line(&mut String::new());
            return;
        }

        match self.rt.block_on(self.oauth_manager.refresh_tokens()) {
            Ok(true) => {
                println!("{} Token refreshed successfully!", style("✓").green());
                let (auth_status, auth_detail) = self.get_auth_status();
                let status_style = if auth_status == "VALID" {
                    style(&auth_status).green()
                } else {
                    style(&auth_status).yellow()
                };
                println!("Status: {} ({})", status_style, auth_detail);
            }
            Ok(false) => {
                println!("{} Token refresh failed - please login again", style("✗").red());
                println!("This usually happens when the refresh token has expired.");
            }
            Err(e) => {
                println!("{} Token refresh failed: {}", style("✗").red(), e);
                println!("Please try logging in again (option 2)");
            }
        }

        println!("\nPress Enter to continue...");
        let _ = io::stdin().read_line(&mut String::new());
    }

    fn logout(&self) {
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Are you sure you want to clear all tokens?")
            .default(false)
            .interact()
            .unwrap_or(false)
        {
            match self.oauth_manager.storage().clear_tokens() {
                Ok(_) => println!("{} Tokens cleared successfully", style("✓").green()),
                Err(e) => println!("{} Error: {}", style("✗").red(), e),
            }
        } else {
            println!("Logout cancelled");
        }

        println!("\nPress Enter to continue...");
        let _ = io::stdin().read_line(&mut String::new());
    }

    pub fn run(&mut self) -> Result<()> {
        let mut server_running = false;

        loop {
            self.clear_screen();
            self.display_header();
            self.display_menu(server_running);

            let options = if server_running {
                vec![
                    "Stop Proxy Server",
                    "Login / Re-authenticate",
                    "Refresh Token",
                    "Show Token Status",
                    "Logout (Clear Tokens)",
                    "Exit",
                ]
            } else {
                vec![
                    "Start Proxy Server",
                    "Login / Re-authenticate",
                    "Refresh Token",
                    "Show Token Status",
                    "Logout (Clear Tokens)",
                    "Exit",
                ]
            };

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select option")
                .items(&options)
                .default(0)
                .interact()
                .unwrap_or(5);

            match selection {
                0 => {
                    if server_running {
                        self.stop_proxy_server();
                        server_running = false;
                    } else {
                        if self.start_proxy_server(0)? {
                            server_running = true;
                        }
                    }
                }
                1 => self.login(),
                2 => self.refresh_token(),
                3 => self.show_token_status(),
                4 => self.logout(),
                5 => {
                    if server_running {
                        println!("Stopping server before exit...");
                        self.stop_proxy_server();
                    }
                    println!("Goodbye!");
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }
}
