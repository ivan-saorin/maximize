# ✅ Maximize Transformation Complete!

## What Was Done

Your Maximize proxy has been successfully transformed into a **production-grade API server** ready for CapRover deployment! 🎉

### Core Changes

#### 1. Server-Only Mode (`src/main.rs`)
- Added `--server-only` flag for headless operation
- New async server function that runs without CLI
- Perfect for containers and production deployments

#### 2. Environment-Based Configuration (`src/storage.rs`)
- Tokens can now be loaded from environment variables:
  - `MAXIMIZE_ACCESS_TOKEN`
  - `MAXIMIZE_REFRESH_TOKEN`
  - `MAXIMIZE_TOKEN_EXPIRES_IN`
- Falls back to file-based storage if env vars not present
- Seamless integration with Docker/CapRover

#### 3. API Key Authentication (`src/proxy.rs`)
- Added middleware to protect `/v1/messages` endpoint
- Supports both `Authorization: Bearer <key>` and `X-API-Key: <key>` headers
- Configurable via `MAXIMIZE_API_KEY` environment variable
- Optional - disabled if not set (backward compatible)

#### 4. Updated Settings (`src/settings.rs`)
- Added `api_key` field to Settings struct
- Loads from `MAXIMIZE_API_KEY` environment variable
- Passed to AppState for authentication

#### 5. CLI Updates (`src/cli.rs`)
- Updated to work with new AppState structure
- Includes API key in server state
- No breaking changes to CLI functionality

### New Files Created

#### Deployment Files
- **`Dockerfile`** - Multi-stage production build
  - Optimized size with slim images
  - Non-root user for security
  - Built-in health checks
  
- **`captain-definition`** - CapRover deployment config
  - Simple JSON configuration
  - Points to Dockerfile

- **`.env.example`** - Environment variable template
  - All required and optional variables
  - Usage examples and security notes

- **`.gitignore`** - Prevents committing sensitive files
  - Tokens, env files, build artifacts

#### Documentation
- **`DEPLOYMENT.md`** - Comprehensive deployment guide
  - CapRover step-by-step instructions
  - Docker deployment options
  - Security best practices
  - Client configuration examples
  - Troubleshooting section

- **`CHANGELOG.md`** - What's new summary
  - Feature overview
  - Migration guide
  - Quick reference tables

- **`deploy-caprover.sh`** - Deployment helper script
  - Extracts tokens automatically
  - Generates secure API keys
  - Deploys to CapRover
  - Provides next steps

## How to Use

### Option 1: Quick CapRover Deployment

```bash
# 1. Authenticate locally first (one time)
cargo build --release
./target/release/maximize
# Select option 2, complete OAuth

# 2. Make deploy script executable
chmod +x deploy-caprover.sh

# 3. Deploy!
./deploy-caprover.sh

# 4. Follow the on-screen instructions to set environment variables in CapRover
```

### Option 2: Manual CapRover Deployment

```bash
# 1. Get tokens
./target/release/maximize  # Login via CLI
cat ~/.maximize/tokens.json  # Copy tokens

# 2. Deploy to CapRover
caprover deploy -a maximize

# 3. In CapRover UI, set environment variables:
MAXIMIZE_ACCESS_TOKEN=sk-ant-your-token
MAXIMIZE_REFRESH_TOKEN=refresh-your-token
MAXIMIZE_API_KEY=$(openssl rand -hex 32)

# 4. Enable HTTPS
# 5. Test: curl https://maximize.yourdomain.com/healthz
```

### Option 3: Docker Deployment

```bash
# Build
docker build -t maximize .

# Run
docker run -d \
  -p 8081:8081 \
  -e MAXIMIZE_ACCESS_TOKEN="sk-ant-..." \
  -e MAXIMIZE_REFRESH_TOKEN="refresh-..." \
  -e MAXIMIZE_API_KEY="your-key" \
  --name maximize \
  maximize
```

### Option 4: Direct Server Mode (No Container)

```bash
# Build
cargo build --release

# Configure
export MAXIMIZE_ACCESS_TOKEN="sk-ant-..."
export MAXIMIZE_REFRESH_TOKEN="refresh-..."
export MAXIMIZE_API_KEY="your-key"

# Run
./target/release/maximize --server-only
```

## Using Your Deployed Proxy

### Python Example

```python
from anthropic import Anthropic

client = Anthropic(
    api_key="your-maximize-api-key",  # From MAXIMIZE_API_KEY
    base_url="https://maximize.yourdomain.com"
)

response = client.messages.create(
    model="l",  # Sonnet 4
    max_tokens=1024,
    messages=[{"role": "user", "content": "Hello!"}]
)
```

### curl Example

```bash
curl https://maximize.yourdomain.com/v1/messages \
  -H "Authorization: Bearer your-api-key" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "l",
    "max_tokens": 1024,
    "messages": [{"role": "user", "content": "Hello!"}]
  }'
```

## Key Features

✅ **Server-Only Mode** - Runs without interactive CLI
✅ **Environment Config** - No file management in containers
✅ **API Key Auth** - Protects your proxy from unauthorized access
✅ **CapRover Ready** - One-command deployment
✅ **Docker Support** - Production-grade containerization
✅ **HTTPS Ready** - Via CapRover or reverse proxy
✅ **Streaming** - Full SSE support (already implemented)
✅ **Model Nicknames** - xs, s, m, l, xl, xxl
✅ **All Features** - Thinking mode, tools, everything works

## Security Checklist

Before going to production:

- [ ] Generate strong API key: `openssl rand -hex 32`
- [ ] Set `MAXIMIZE_API_KEY` environment variable
- [ ] Enable HTTPS in CapRover
- [ ] Set proper token expiration
- [ ] Restrict access if possible (IP whitelist, VPN, etc.)
- [ ] Monitor `/healthz` endpoint
- [ ] Set up log monitoring
- [ ] Document your API key securely

## Next Steps

1. **Test Locally First**
   ```bash
   cargo build --release
   ./target/release/maximize --server-only
   ```

2. **Deploy to CapRover**
   ```bash
   ./deploy-caprover.sh
   ```

3. **Update Your Projects**
   - Change base_url to your deployed proxy
   - Add API key to requests
   - Test with health endpoint

4. **Monitor**
   - Set up health check monitoring
   - Watch CapRover logs
   - Track token expiration

## Files Changed Summary

| File | Changes |
|------|---------|
| `src/main.rs` | Added `--server-only` mode, async server function |
| `src/proxy.rs` | Added API key authentication middleware, updated AppState |
| `src/storage.rs` | Environment variable token loading |
| `src/settings.rs` | Added `api_key` field, env loading |
| `src/cli.rs` | Updated for new AppState structure |
| **NEW** `Dockerfile` | Production container build |
| **NEW** `captain-definition` | CapRover config |
| **NEW** `DEPLOYMENT.md` | Complete deployment guide |
| **NEW** `CHANGELOG.md` | Feature summary |
| **NEW** `.env.example` | Environment template |
| **NEW** `.gitignore` | Security - prevents token commits |
| **NEW** `deploy-caprover.sh` | Deployment helper |

## Testing the Changes

```bash
# 1. Build
cargo build --release

# 2. Test CLI mode (should work as before)
./target/release/maximize

# 3. Get tokens
cat ~/.maximize/tokens.json

# 4. Test server-only mode
export MAXIMIZE_ACCESS_TOKEN="sk-ant-..."
export MAXIMIZE_REFRESH_TOKEN="refresh-..."
export MAXIMIZE_API_KEY="test-key"
./target/release/maximize --server-only

# 5. Test health check (in another terminal)
curl http://localhost:8081/healthz

# 6. Test API (should require API key)
curl http://localhost:8081/v1/messages \
  -H "Authorization: Bearer test-key" \
  -H "Content-Type: application/json" \
  -d '{"model":"l","max_tokens":100,"messages":[{"role":"user","content":"Hi"}]}'
```

## Troubleshooting

### Build Errors

If you get compilation errors, make sure you have:
- Rust 1.70 or later
- OpenSSL development libraries

```bash
# Ubuntu/Debian
sudo apt-get install pkg-config libssl-dev

# macOS
brew install openssl

# Then rebuild
cargo clean
cargo build --release
```

### Runtime Issues

**"No tokens found"**
- Set `MAXIMIZE_ACCESS_TOKEN` and `MAXIMIZE_REFRESH_TOKEN`
- Or run CLI first to authenticate

**"Invalid API key"**
- Make sure `MAXIMIZE_API_KEY` matches in server and client
- Check Authorization header format

**Port already in use**
- Change port: `--bind 127.0.0.1:8082`
- Or set `MAXIMIZE_PORT=8082`

## Benefits

### Before
- ❌ CLI only, required interactive terminal
- ❌ Token file management in containers
- ❌ No authentication protection
- ❌ Manual deployment steps

### After
- ✅ Runs headless in any environment
- ✅ Environment-based configuration
- ✅ API key protection
- ✅ One-command deployment
- ✅ Production-ready
- ✅ Multiple projects can share one proxy

## Questions?

- 📖 **Detailed guide**: [DEPLOYMENT.md](DEPLOYMENT.md)
- 🔄 **What changed**: [CHANGELOG.md](CHANGELOG.md)
- 🎬 **Get started**: Run `./deploy-caprover.sh`

---

**Remember**: This is still for educational purposes only. Use at your own risk.

🎉 **You're ready to deploy your production Claude Max proxy!**
