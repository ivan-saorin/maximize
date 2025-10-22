# Maximize Deployment Guide

## Production Server Mode

Maximize can run in server-only mode without the interactive CLI, perfect for containers and production deployments.

## Quick Start: Server-Only Mode

```bash
# Build
cargo build --release

# Run server-only mode
./target/release/maximize --server-only
```

## Environment Variables

### Required for Production

```bash
# Claude Max OAuth tokens (get these via CLI first)
export MAXIMIZE_ACCESS_TOKEN="your-access-token"
export MAXIMIZE_REFRESH_TOKEN="your-refresh-token"
export MAXIMIZE_TOKEN_EXPIRES_IN="86400"  # Optional, defaults to 24h

# API Key authentication (HIGHLY RECOMMENDED for production)
export MAXIMIZE_API_KEY="your-secure-api-key-here"
```

### Optional Configuration

```bash
# Server settings (or use config.json)
export MAXIMIZE_PORT="8081"
export MAXIMIZE_BIND_ADDRESS="0.0.0.0"
```

## Getting Tokens for Production

Since production environments can't use interactive OAuth, you need to get tokens first:

### Method 1: Use CLI Locally

```bash
# Run CLI on your machine
./maximize

# Select option 2 (Login)
# Complete OAuth flow
# Tokens are saved to ~/.maximize/tokens.json

# Extract tokens
cat ~/.maximize/tokens.json
```

### Method 2: Use Existing Installation

If you already have maximize running locally with valid tokens:

```bash
# Read your tokens
cat ~/.maximize/tokens.json

# Copy the access_token and refresh_token values
```

## CapRover Deployment

### Prerequisites

- CapRover installed and configured
- CLI tool installed: `npm install -g caprover`
- Logged into your CapRover instance

### Step 1: Get Your Tokens

Run maximize locally first and authenticate:

```bash
./maximize
# Choose option 2 (Login)
# Complete OAuth flow
```

Extract your tokens:

```bash
cat ~/.maximize/tokens.json
```

You'll see something like:

```json
{
  "access_token": "sk-ant-...",
  "refresh_token": "refresh_...",
  "expires_at": 1234567890
}
```

### Step 2: Deploy to CapRover

```bash
# Initialize CapRover app (first time only)
caprover deploy

# Or deploy to existing app
caprover deploy -a maximize
```

### Step 3: Configure Environment Variables

In CapRover web interface:

1. Go to your app → **App Configs** → **Environment Variables**
2. Add the following variables:

```
MAXIMIZE_ACCESS_TOKEN=sk-ant-your-token-here
MAXIMIZE_REFRESH_TOKEN=refresh-your-token-here
MAXIMIZE_TOKEN_EXPIRES_IN=86400
MAXIMIZE_API_KEY=your-secure-random-key-here
```

3. Click **Save & Update**

### Step 4: Enable HTTPS (Recommended)

1. Go to **HTTP Settings**
2. Enable **HTTPS**
3. Enable **Force HTTPS**
4. Add your domain (e.g., `maximize.yourdomain.com`)

### Step 5: Test Your Deployment

```bash
# Health check
curl https://maximize.yourdomain.com/healthz

# Test API (use your MAXIMIZE_API_KEY)
curl https://maximize.yourdomain.com/v1/messages \
  -H "Authorization: Bearer your-api-key-here" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "l",
    "max_tokens": 1024,
    "messages": [{"role": "user", "content": "Hello!"}]
  }'
```

## Docker Deployment

### Build Image

```bash
docker build -t maximize:latest .
```

### Run Container

```bash
docker run -d \
  --name maximize \
  -p 8081:8081 \
  -e MAXIMIZE_ACCESS_TOKEN="sk-ant-..." \
  -e MAXIMIZE_REFRESH_TOKEN="refresh-..." \
  -e MAXIMIZE_API_KEY="your-api-key" \
  maximize:latest
```

### Docker Compose

```yaml
version: '3.8'

services:
  maximize:
    build: .
    ports:
      - "8081:8081"
    environment:
      - MAXIMIZE_ACCESS_TOKEN=${MAXIMIZE_ACCESS_TOKEN}
      - MAXIMIZE_REFRESH_TOKEN=${MAXIMIZE_REFRESH_TOKEN}
      - MAXIMIZE_API_KEY=${MAXIMIZE_API_KEY}
      - MAXIMIZE_TOKEN_EXPIRES_IN=86400
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8081/healthz"]
      interval: 30s
      timeout: 3s
      retries: 3
```

Run with:

```bash
docker-compose up -d
```

## Security Best Practices

### 1. Always Use API Key Authentication

```bash
# Generate a secure random key
openssl rand -hex 32

# Set it as environment variable
export MAXIMIZE_API_KEY="your-generated-key"
```

### 2. Use HTTPS in Production

- Always enable HTTPS for production deployments
- Use proper SSL certificates (Let's Encrypt via CapRover is perfect)
- Never expose port 8081 directly to the internet without HTTPS

### 3. Token Rotation

Claude Max tokens expire. When they do:

1. Run maximize CLI locally
2. Select option 3 (Refresh Token)
3. Extract new tokens from `~/.maximize/tokens.json`
4. Update environment variables in CapRover
5. Restart the app

### 4. Firewall Rules

- Only expose port 443 (HTTPS) to the internet
- Use internal networking for service-to-service communication
- Consider IP whitelisting if possible

## Monitoring

### Health Endpoint

```bash
curl https://your-domain.com/healthz
```

Returns:

```json
{
  "status": "ok",
  "timestamp": 1234567890
}
```

### Auth Status Endpoint

```bash
curl https://your-domain.com/auth/status
```

Returns token status without requiring API key authentication.

### Logs

CapRover logs are available in the web interface or via CLI:

```bash
caprover logs -a maximize -f
```

## Client Configuration

### Using with Anthropic SDK

```python
from anthropic import Anthropic

client = Anthropic(
    api_key="your-maximize-api-key",  # Your MAXIMIZE_API_KEY
    base_url="https://maximize.yourdomain.com"
)

response = client.messages.create(
    model="l",  # or any model nickname
    max_tokens=1024,
    messages=[{"role": "user", "content": "Hello!"}]
)
```

### Using with OpenAI SDK (Compatible)

```python
from openai import OpenAI

client = OpenAI(
    api_key="your-maximize-api-key",
    base_url="https://maximize.yourdomain.com/v1"
)

# Note: Use Anthropic model names
```

### Using with curl

```bash
curl https://maximize.yourdomain.com/v1/messages \
  -H "Authorization: Bearer your-api-key" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "l",
    "max_tokens": 1024,
    "messages": [
      {"role": "user", "content": "Hello Claude!"}
    ],
    "stream": true
  }'
```

## Troubleshooting

### "No tokens found" Error

Make sure you've set the environment variables:

```bash
MAXIMIZE_ACCESS_TOKEN
MAXIMIZE_REFRESH_TOKEN
```

### "Invalid API key" Error

1. Check that `MAXIMIZE_API_KEY` is set in CapRover
2. Verify you're sending the correct API key in requests
3. Check both `Authorization` and `X-API-Key` headers

### Token Expired

Tokens expire after ~24 hours. To refresh:

1. Use the CLI locally to refresh
2. Or set up automated token rotation (advanced)

### Port Already in Use

Change the port in environment variables:

```bash
MAXIMIZE_PORT=8082
```

## Advanced: Automated Token Rotation

For production, you may want to automate token refreshing. The proxy automatically refreshes tokens when they expire, but you can also set up a cron job:

```bash
# Add to crontab
0 */12 * * * curl -X POST https://maximize.yourdomain.com/auth/refresh
```

## Support

- GitHub Issues: [Your Repo]
- Original Project: [Pimzino's Python Version]

## License

MIT License - Educational purposes only, use at your own risk.
