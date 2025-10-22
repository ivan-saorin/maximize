# CapRover Deployment Guide

## Prerequisites

1. **Get Your Claude Max Tokens**
   
   Run locally first to obtain tokens:
   ```bash
   cargo run
   # Select option 2: Login
   # Complete OAuth flow
   # Tokens saved to ~/.maximize/tokens.json
   ```

2. **Extract Tokens**
   
   ```bash
   # View your tokens
   cat ~/.maximize/tokens.json
   ```
   
   You'll see something like:
   ```json
   {
     "access_token": "sk-ant-...",
     "refresh_token": "refresh_...",
     "expires_at": 1729612800
   }
   ```

## CapRover Deployment

### Method 1: Using CapRover CLI (Recommended)

1. **Initialize CapRover in your project**
   
   ```bash
   npm install -g caprover
   caprover login
   ```

2. **Deploy**
   
   ```bash
   caprover deploy
   ```

3. **Set Environment Variables** in CapRover dashboard:
   
   **Required:**
   - `MAXIMIZE_ACCESS_TOKEN` = Your access_token from tokens.json
   - `MAXIMIZE_REFRESH_TOKEN` = Your refresh_token from tokens.json
   
   **Strongly Recommended:**
   - `MAXIMIZE_API_KEY` = `your-secret-api-key-here` (protect your proxy!)
   
   **Optional:**
   - `MAXIMIZE_TOKEN_EXPIRES_IN` = `86400` (seconds, default 24h)
   - `RUST_LOG` = `info` (or `debug` for verbose logging)

4. **Enable HTTPS** (CapRover dashboard → Enable HTTPS)

5. **Test Your Deployment**
   
   ```bash
   curl https://your-app.your-domain.com/healthz
   ```

### Method 2: Manual Docker Build

```bash
# Build
docker build -t maximize:latest .

# Run
docker run -d \
  --name maximize \
  -p 8081:8081 \
  -e MAXIMIZE_ACCESS_TOKEN="sk-ant-..." \
  -e MAXIMIZE_REFRESH_TOKEN="refresh_..." \
  -e MAXIMIZE_API_KEY="your-secret-key" \
  maximize:latest
```

## Environment Variables Reference

| Variable | Required | Description | Example |
|----------|----------|-------------|---------|
| `MAXIMIZE_ACCESS_TOKEN` | Yes* | Claude Max access token | `sk-ant-api03-...` |
| `MAXIMIZE_REFRESH_TOKEN` | Yes* | Claude Max refresh token | `refresh_...` |
| `MAXIMIZE_API_KEY` | **Recommended** | API key to protect your proxy | `sk-my-secret-key-123` |
| `MAXIMIZE_TOKEN_EXPIRES_IN` | No | Token expiry in seconds | `86400` (24h) |
| `RUST_LOG` | No | Logging level | `info`, `debug`, `warn` |

\* Not required if using mounted `tokens.json` file

## Using Your Deployed Proxy

### With Anthropic SDK

```python
import anthropic

client = anthropic.Anthropic(
    api_key="your-secret-key",  # Your MAXIMIZE_API_KEY
    base_url="https://your-app.your-domain.com"
)

message = client.messages.create(
    model="l",  # or "s", "m", "xl", "xxl"
    max_tokens=1024,
    messages=[{"role": "user", "content": "Hello!"}]
)
```

### With cURL

```bash
curl https://your-app.your-domain.com/v1/messages \
  -H "Authorization: Bearer your-secret-key" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "l",
    "max_tokens": 1024,
    "messages": [{"role": "user", "content": "Hello!"}]
  }'
```

### Model Nicknames

- `xs` → claude-3-5-haiku-20241022
- `s` → claude-3-5-sonnet-20241022  
- `m` → claude-3-7-sonnet-20250219
- `l` → claude-sonnet-4-20250514 ⭐ (default)
- `xl` → claude-opus-4-20250514
- `xxl` → claude-opus-4-1-20250805

## Security Best Practices

1. **Always set `MAXIMIZE_API_KEY`** - Never run unprotected in production
2. **Use HTTPS** - Enable in CapRover dashboard
3. **Rotate tokens periodically** - Get new tokens every few weeks
4. **Monitor usage** - Check CapRover logs regularly
5. **Set resource limits** - Configure memory/CPU limits in CapRover

## Monitoring

### Health Check
```bash
curl https://your-app.your-domain.com/healthz
```

### Token Status
```bash
curl https://your-app.your-domain.com/auth/status
```

### Logs
```bash
caprover logs -a your-app-name --lines 100
```

## Troubleshooting

### "No valid token available"
- Check `MAXIMIZE_ACCESS_TOKEN` and `MAXIMIZE_REFRESH_TOKEN` are set correctly
- Tokens may have expired - get fresh ones by running CLI locally

### "Missing API key"  
- Set `MAXIMIZE_API_KEY` environment variable in CapRover
- Or remove if you want unprotected access (not recommended)

### "Token refresh failed"
- Refresh token expired - re-authenticate locally and update env vars
- Network issue - check CapRover logs

### Build fails
- Ensure Rust 1.75+ is available in builder image
- Check Cargo.toml dependencies are up to date

## Updating

1. **Update tokens** (if expired):
   ```bash
   # Run locally
   cargo run
   # Login again (option 2)
   # Copy new tokens to CapRover env vars
   ```

2. **Redeploy code**:
   ```bash
   git pull
   caprover deploy
   ```

## Architecture

```
Client Request (with your API key)
    ↓
[CapRover HTTPS]
    ↓
[Maximize Proxy - API Key Auth]
    ↓
[Token Management - Auto-refresh]
    ↓
[Anthropic API - Claude Max]
    ↓
Response (streamed back to client)
```

## Cost & Usage

- **Free tier**: Use your existing Claude Pro/Max subscription
- **No API costs**: Leverages your subscription, not pay-per-token API
- **Rate limits**: Subject to Claude Max subscription limits
- **Resource usage**: ~10-50MB RAM, minimal CPU

## Support

For issues or questions:
1. Check logs: `caprover logs -a your-app-name`
2. Test locally: `cargo run --server-only`
3. Verify tokens: Check `~/.maximize/tokens.json`
4. Review original README for CLI usage

## Advanced: Multiple Projects

You can deploy multiple instances for different projects:

```bash
# Deploy instance 1
caprover deploy -a project-a-maximize

# Deploy instance 2  
caprover deploy -a project-b-maximize
```

Each can have different:
- API keys (isolate access per project)
- Token sets (different Claude accounts)
- Resource limits (scale independently)
