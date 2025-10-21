# Maximize

High-performance Anthropic Claude Max Proxy in Rust.

Pure Anthropic proxy for Claude Pro/Max subscriptions using OAuth, rewritten in Rust for maximum performance.

## SUPPORT THE ORIGINAL AUTHOR

The original Python implementation: <a href="https://buymeacoffee.com/Pimzino" target="_blank">Buy Me A Coffee</a>

## DISCLAIMER

**FOR EDUCATIONAL PURPOSES ONLY**

This tool:

- Is NOT affiliated with or endorsed by Anthropic
- Uses undocumented OAuth flows from Claude Code
- May violate Anthropic's Terms of Service
- Could stop working at any time without notice
- Comes with NO WARRANTY or support

**USE AT YOUR OWN RISK. The authors assume no liability for any consequences.**

For official access, use Claude Code or Anthropic's API with console API keys.

## Prerequisites

- Active Claude Pro or Claude Max subscription
- Rust 1.70+ (for building from source)
- Or download pre-built binary from releases

## Quick Start

### Option 1: Docker (Recommended for Production)

```bash
# Build and start
docker-compose up -d

# Attach for authentication
docker attach maximize
# Complete OAuth flow (option 2)
# Detach: Ctrl+P, Ctrl+Q

# Proxy is now running at http://localhost:8081
```

See [DOCKER.md](DOCKER.md) for comprehensive Docker documentation.

### Option 2: Building from Source

```bash
# Clone and build
git clone <repository-url>
cd maximize
cargo build --release

# The binary will be at target/release/maximize (or maximize.exe on Windows)
```

### Configuration (Optional)

```bash
# Copy example config
cp config.example.json config.json

# Edit config.json to customize settings
```

### Running

```bash
# Run the CLI
./target/release/maximize

# Or with custom bind address:
./target/release/maximize --bind 127.0.0.1

# Enable debug logging:
./target/release/maximize --debug
```

### Authentication

1. Select option 2 (Login)
2. Browser opens automatically
3. Complete login at claude.ai
4. Copy the authorization code
5. Paste in terminal

### Start Proxy

1. Select option 1 (Start Proxy Server)
2. Server runs at `http://0.0.0.0:8081` (default, listens on all interfaces)

## Model Nicknames

Maximize uses simple nicknames for models:

- `xs` → claude-3-5-haiku-20241022
- `s` → claude-3-5-sonnet-20241022
- `m` → claude-3-7-sonnet-20250219
- `l` → claude-sonnet-4-20250514 (default)
- `xl` → claude-opus-4-20250514
- `xxl` → claude-opus-4-1-20250805

Use these nicknames in your API requests for cleaner configuration.

## Client Configuration

Configure your Anthropic API client:

- **Base URL:** `http://<proxy-host>:8081` (default: `http://0.0.0.0:8081`)
- **API Key:** Any non-empty string (e.g., "dummy")
- **Model:** `l` (or any model nickname: xs, s, m, l, xl, xxl)
- **Endpoint:** `/v1/messages`

## Available Models

Supports all Anthropic Models that you have access to with your Claude Pro / Max subscription.
You can use either the model nickname (xs, s, m, l, xl, xxl) or the full model name.

## Tested & Supported Features

- Browser use
- Images
- Streaming responses
- Extended thinking mode
- Tool use

## Configuration Priority

1. Environment variables (highest)
2. config.json file
3. Built-in defaults (lowest)

## Troubleshooting

If you encounter issues, check [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for common problems and solutions.

Key issues:

- "Is a directory" errors → Check your `TOKEN_FILE` path
- Token expired → Re-authenticate with option 2
- Port in use → Change port in config or kill existing process
- Docker exits immediately → Use attach mode for authentication

## Performance

Maximize is built in Rust for optimal performance:

- Near-zero latency overhead
- Efficient async I/O with Tokio
- Minimal memory footprint
- Fast startup time
- Production-ready error handling

## License

MIT License - see [LICENSE](LICENSE) file

This software is provided for educational purposes only. Users assume all risks.

## Differences from Python Version

- **Performance:** Significantly faster with lower resource usage
- **Model Names:** Uses simple nicknames (xs, s, m, l, xl, xxl) instead of full model names
- **Binary Distribution:** Can be distributed as a single executable
- **Memory Safety:** Rust's ownership system prevents common bugs
- **Concurrent Handling:** Better handling of multiple simultaneous requests
