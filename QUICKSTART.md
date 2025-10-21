# 🚀 Quickstart Guide

Get Maximize running in 5 minutes.

## Prerequisites

- ✅ Active Claude Pro or Max subscription
- ✅ **Either:**
  - Docker & Docker Compose (for Docker deployment)
  - **OR** Rust installed (https://rustup.rs/) (for source build)
- ✅ Windows, Linux, or macOS

## Choose Your Path

### 🐳 Path A: Docker (Recommended for Production)

**Step 1: Build & Start**

```bash
# Build and start container
docker-compose up -d

# Or using make (Unix)
make docker

# Or using docker.bat (Windows)
docker.bat build
docker.bat up
```

**Step 2: Authenticate**

```bash
# Attach to container
docker attach maximize

# Or: make docker-attach (Unix)
# Or: docker.bat attach (Windows)

# In the menu:
# Select: 2 (Login / Re-authenticate)
# Complete OAuth flow

# Detach (keep running):
# Press: Ctrl+P, then Ctrl+Q
```

**Step 3: Use It!**

Proxy is running at `http://localhost:8081` ✨

See [DOCKER.md](DOCKER.md) for advanced Docker usage.

---

### 🦀 Path B: Build from Source

**Step 1: Build**

```bash
# Clone and build
git clone <repository-url>
cd maximize
cargo build --release

# Binary is now at: target/release/maximize
```

**Windows users:** Use `build.bat` instead:
```cmd
build.bat
```

**Unix users:** Use Makefile:
```bash
make
```

## Step 2: Run & Login

```bash
# Start the CLI
./target/release/maximize

# In the menu:
# Select: 2 (Login / Re-authenticate)
```

1. Browser opens automatically
2. Login to claude.ai
3. Authorize the application
4. Copy the code shown
5. Paste into CLI

## Step 3: Start Proxy

```bash
# In the menu:
# Select: 1 (Start Proxy Server)
```

Proxy is now running at `http://localhost:8081` ✨

## Step 4: Test It

### Python
```python
from anthropic import Anthropic

client = Anthropic(
    api_key="dummy",
    base_url="http://localhost:8081"
)

message = client.messages.create(
    model="l",
    max_tokens=1024,
    messages=[{"role": "user", "content": "Hello!"}]
)

print(message.content)
```

### cURL
```bash
curl -X POST http://localhost:8081/v1/messages \
  -H "Content-Type: application/json" \
  -H "x-api-key: dummy" \
  -d '{
    "model": "l",
    "max_tokens": 1024,
    "messages": [{"role": "user", "content": "Hello!"}]
  }'
```

## Model Nicknames

Use these shortcuts instead of full model names:

- `xs` - Fastest (Haiku)
- `s` - Balanced (Sonnet 3.5)
- `m` - Enhanced (Sonnet 3.7)
- `l` - **Default** (Sonnet 4) ⭐
- `xl` - Maximum (Opus 4)
- `xxl` - Ultimate (Opus 4.1)

See [MODELS.md](MODELS.md) for details.

## Common Commands

### Start with Debug
```bash
./target/release/maximize --debug
```

### Bind to Localhost Only
```bash
./target/release/maximize --bind 127.0.0.1
```

### Install System-Wide (Unix)
```bash
make install
# Now just: maximize
```

## Configuration (Optional)

Create `config.json`:
```json
{
  "server": {
    "port": 8081,
    "bind_address": "0.0.0.0"
  },
  "models": {
    "default": "l"
  }
}
```

## Troubleshooting

### "No tokens available"
→ Select option 2 (Login) in the CLI

### "Token expired"
→ Select option 3 (Refresh Token) in the CLI

### "Port already in use"
→ Run with: `./maximize --bind 127.0.0.1:8082`

### "Browser won't open"
→ Copy the URL from CLI and open manually

## Next Steps

- 📖 Read [USAGE.md](USAGE.md) for detailed usage
- 🆚 See [COMPARISON.md](COMPARISON.md) vs Python version
- 📋 Check [MODELS.md](MODELS.md) for model guide
- 🔧 Review [README.md](README.md) for full documentation

## Need Help?

1. Enable debug mode: `./maximize --debug`
2. Check token status: Select option 4 in CLI
3. Review logs in terminal
4. Open an issue on GitHub

---

**That's it!** You're now running a high-performance Claude proxy. 🎉

**Pro Tip:** Keep the CLI running in the background and it will auto-refresh your tokens.
