# Maximize v2.0 - Production API Server 🚀

## What's New

Your Maximize proxy has been transformed into a **full-fledged production API server** with enterprise-ready features!

## Major Changes

### 1. **Server-Only Mode** 🖥️
No more CLI dependency! Run headless in containers:

```bash
# Old way (CLI required)
./maximize

# New way (server-only, perfect for containers)
./maximize --server-only
```

### 2. **Environment-Based Token Injection** 🔐
Tokens can now be provided via environment variables (perfect for Docker/CapRover):

```bash
export MAXIMIZE_ACCESS_TOKEN="sk-ant-..."
export MAXIMIZE_REFRESH_TOKEN="refresh-..."
./maximize --server-only
```

No more dealing with token files in containers!

### 3. **API Key Authentication** 🛡️
Protect your proxy from unauthorized access:

```bash
export MAXIMIZE_API_KEY="your-secure-key"
./maximize --server-only
```

Clients must now provide the API key:

```bash
curl https://your-server/v1/messages \
  -H "Authorization: Bearer your-secure-key" \
  -d '{...}'
```

### 4. **CapRover Ready** ☁️
One-command deployment to your CapRover instance:

```bash
caprover deploy
```

Includes:
- `Dockerfile` for production builds
- `captain-definition` for CapRover
- Health checks
- Automatic HTTPS via CapRover

### 5. **Docker Support** 🐳
Production-grade Dockerfile with:
- Multi-stage builds (smaller images)
- Non-root user
- Health checks
- Optimized for security

### 6. **Full API Compatibility** ✅
Everything works exactly as before:
- ✅ Streaming responses
- ✅ Model nicknames (xs, s, m, l, xl, xxl)
- ✅ Extended thinking mode
- ✅ Tool use
- ✅ All Anthropic features

## File Changes

### New Files
- `Dockerfile` - Production container build
- `captain-definition` - CapRover deployment config
- `DEPLOYMENT.md` - Comprehensive deployment guide
- `.env.example` - Environment variable template

### Modified Files
- `src/main.rs` - Added server-only mode
- `src/proxy.rs` - Added API key authentication middleware
- `src/storage.rs` - Environment-based token loading
- `src/settings.rs` - API key configuration support
- `src/cli.rs` - Updated to support new AppState

## How to Use

### Local Development (Unchanged)

```bash
cargo build --release
./target/release/maximize
# Use interactive CLI as before
```

### Production Deployment

#### Option 1: CapRover (Recommended)

```bash
# 1. Get tokens locally first
./maximize
# Login and get tokens

# 2. Deploy to CapRover
caprover deploy

# 3. Set environment variables in CapRover UI:
MAXIMIZE_ACCESS_TOKEN=sk-ant-...
MAXIMIZE_REFRESH_TOKEN=refresh-...
MAXIMIZE_API_KEY=your-secure-key

# 4. Enable HTTPS in CapRover
# Done! Your API is live at https://maximize.yourdomain.com
```

#### Option 2: Docker

```bash
# Build
docker build -t maximize .

# Run
docker run -d \
  -p 8081:8081 \
  -e MAXIMIZE_ACCESS_TOKEN="sk-ant-..." \
  -e MAXIMIZE_REFRESH_TOKEN="refresh-..." \
  -e MAXIMIZE_API_KEY="your-key" \
  maximize
```

#### Option 3: Direct Binary

```bash
# Build
cargo build --release

# Set environment
export MAXIMIZE_ACCESS_TOKEN="sk-ant-..."
export MAXIMIZE_REFRESH_TOKEN="refresh-..."
export MAXIMIZE_API_KEY="your-key"

# Run
./target/release/maximize --server-only
```

## Client Usage

### Python (Anthropic SDK)

```python
from anthropic import Anthropic

client = Anthropic(
    api_key="your-maximize-api-key",  # Your MAXIMIZE_API_KEY
    base_url="https://maximize.yourdomain.com"
)

response = client.messages.create(
    model="l",  # Sonnet 4
    max_tokens=1024,
    messages=[{"role": "user", "content": "Hello!"}]
)
```

### TypeScript

```typescript
import Anthropic from '@anthropic-ai/sdk';

const client = new Anthropic({
  apiKey: process.env.MAXIMIZE_API_KEY,
  baseURL: 'https://maximize.yourdomain.com'
});

const response = await client.messages.create({
  model: 'l',
  max_tokens: 1024,
  messages: [{ role: 'user', content: 'Hello!' }]
});
```

### curl

```bash
curl https://maximize.yourdomain.com/v1/messages \
  -H "Authorization: Bearer your-api-key" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "l",
    "max_tokens": 1024,
    "messages": [{"role": "user", "content": "Hello!"}],
    "stream": true
  }'
```

## Security Features

1. **API Key Authentication** - Prevents unauthorized access
2. **Environment Variables** - No sensitive data in code
3. **HTTPS Support** - Via CapRover or reverse proxy
4. **Non-root Container** - Docker security best practices
5. **Health Checks** - Monitor service availability

## Migration Guide

### From CLI-Only to Production

1. **Get Your Tokens**
   ```bash
   ./maximize  # Use CLI to authenticate
   cat ~/.maximize/tokens.json  # Copy tokens
   ```

2. **Set Environment Variables**
   ```bash
   export MAXIMIZE_ACCESS_TOKEN="..."
   export MAXIMIZE_REFRESH_TOKEN="..."
   export MAXIMIZE_API_KEY="$(openssl rand -hex 32)"
   ```

3. **Run Server-Only Mode**
   ```bash
   ./maximize --server-only
   ```

4. **Update Your Clients**
   - Add API key to Authorization header
   - Point to your new server URL
   - Done!

## Benefits

### Before (CLI-Only)
- ❌ Required interactive terminal
- ❌ Token file management in containers
- ❌ No authentication
- ❌ Manual setup on each server

### After (Production Server)
- ✅ Runs headless in containers
- ✅ Environment-based configuration
- ✅ API key authentication
- ✅ One-command deployment
- ✅ HTTPS-ready
- ✅ Auto-scaling capable
- ✅ Multiple projects can share one proxy

## Quick Reference

### Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `MAXIMIZE_ACCESS_TOKEN` | Yes (prod) | Claude Max access token |
| `MAXIMIZE_REFRESH_TOKEN` | Yes (prod) | Claude Max refresh token |
| `MAXIMIZE_API_KEY` | Recommended | API key for authentication |
| `MAXIMIZE_TOKEN_EXPIRES_IN` | No | Token expiry (default: 86400s) |
| `MAXIMIZE_PORT` | No | Server port (default: 8081) |
| `MAXIMIZE_BIND_ADDRESS` | No | Bind address (default: 0.0.0.0) |

### Endpoints

| Endpoint | Auth Required | Description |
|----------|---------------|-------------|
| `GET /healthz` | No | Health check |
| `GET /auth/status` | No | Token status |
| `POST /v1/messages` | Yes | Anthropic messages API |

### Model Nicknames

| Nickname | Model |
|----------|-------|
| `xs` | claude-3-5-haiku-20241022 |
| `s` | claude-3-5-sonnet-20241022 |
| `m` | claude-3-7-sonnet-20250219 |
| `l` | claude-sonnet-4-20250514 (default) |
| `xl` | claude-opus-4-20250514 |
| `xxl` | claude-opus-4-1-20250805 |

## Next Steps

1. **Deploy to CapRover**: See [DEPLOYMENT.md](DEPLOYMENT.md)
2. **Configure Projects**: Update your projects to use the new API
3. **Set Up Monitoring**: Use `/healthz` endpoint
4. **Enable HTTPS**: Via CapRover or reverse proxy

## Need Help?

- 📖 [Full Deployment Guide](DEPLOYMENT.md)
- 🐛 [Report Issues](https://github.com/your-repo/issues)
- 💬 [Discussions](https://github.com/your-repo/discussions)

---

**Note**: This is still for educational purposes only. Use at your own risk.
