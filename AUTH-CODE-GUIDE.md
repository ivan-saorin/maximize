# Using Authorization Code for Auto-Authentication 🔐

## The Problem This Solves

Before, you had to:
1. Run CLI interactively
2. Complete OAuth
3. Extract tokens from file
4. Set env vars manually

**Now you can skip all that!** Just set the authorization code and the server does the rest.

## Quick Start

### Step 1: Get the Authorization Code

Start the server once to see the OAuth URL:

```bash
cargo run --release -- --server-only
```

You'll see:
```
🔗 OAuth URL (for authentication):
   https://claude.ai/oauth/authorize?code=true&client_id=...&state=...
```

**Copy that URL and open it in your browser.**

### Step 2: Authorize and Get the Code

1. Log into your Claude Pro/Max account
2. Authorize the application
3. **Anthropic will show you a code** that looks like:
   ```
   Authentication Code
   Paste this into Claude Code:
   abc123xyz#def456uvw
   ```

4. **Copy the ENTIRE string** (including the `#` and everything after it)

### Step 3: Set the Environment Variable

```bash
# Linux/Mac
export MAXIMIZE_AUTHENTICATION_CODE="abc123xyz#def456uvw"
export MAXIMIZE_API_KEY="your-api-key"

# Windows (PowerShell)
$env:MAXIMIZE_AUTHENTICATION_CODE="abc123xyz#def456uvw"
$env:MAXIMIZE_API_KEY="your-api-key"
```

Or add to `.env` file:

```bash
# .env
MAXIMIZE_AUTHENTICATION_CODE="abc123xyz#def456uvw"
MAXIMIZE_API_KEY="your-api-key"
```

### Step 4: Start the Server

```bash
cargo run --release -- --server-only
```

**The server will automatically:**
1. ✅ Detect the `MAXIMIZE_AUTHENTICATION_CODE`
2. ✅ Exchange it for access and refresh tokens
3. ✅ Save the tokens
4. ✅ Start serving requests

You'll see:
```
🔄 Found MAXIMIZE_AUTHENTICATION_CODE, exchanging for tokens...
✅ Successfully exchanged authorization code for tokens!
💡 Tokens saved. You can now remove MAXIMIZE_AUTHENTICATION_CODE from environment.
✅ Tokens loaded successfully
🚀 Maximize server starting in SERVER-ONLY mode
```

### Step 5: Clean Up (Optional)

After successful exchange, you can remove the auth code:

```bash
# Remove from environment
unset MAXIMIZE_AUTHENTICATION_CODE

# Or remove from .env file
# (The tokens are now saved to ~/.maximize/tokens.json)
```

The server will use the saved tokens on future starts!

## Production Deployment

### Docker

```bash
docker run -d \
  -e MAXIMIZE_AUTHENTICATION_CODE="abc123#xyz456" \
  -e MAXIMIZE_API_KEY="your-api-key" \
  -p 8081:8081 \
  maximize
```

**First run**: Exchanges code for tokens
**Subsequent runs**: Uses saved tokens (or set tokens directly)

### CapRover

1. **First deployment** - Set environment variables:
   ```
   MAXIMIZE_AUTHENTICATION_CODE=abc123#xyz456
   MAXIMIZE_API_KEY=your-api-key
   ```

2. **Deploy** - Server exchanges code and saves tokens

3. **Optional**: Remove `MAXIMIZE_AUTHENTICATION_CODE` after first successful start

### Docker Compose

```yaml
services:
  maximize:
    build: .
    environment:
      # One-time setup - server will exchange and save tokens
      - MAXIMIZE_AUTHENTICATION_CODE=abc123#xyz456
      - MAXIMIZE_API_KEY=your-api-key
    ports:
      - "8081:8081"
    volumes:
      # Optional: persist tokens across container restarts
      - ./tokens:/root/.maximize
```

## Three Ways to Authenticate

### Method 1: Auto-Exchange (NEW! ⭐)

```bash
# Set auth code, server handles the rest
export MAXIMIZE_AUTHENTICATION_CODE="code#state"
./maximize --server-only
# Done! Tokens automatically exchanged and saved
```

✅ **Best for**: Production deployments, automated setups
✅ **Pros**: Simple, no CLI needed, one environment variable
❌ **Cons**: Need to get OAuth code manually first

### Method 2: Direct Tokens

```bash
# Set tokens directly (if you already have them)
export MAXIMIZE_ACCESS_TOKEN="sk-ant-..."
export MAXIMIZE_REFRESH_TOKEN="refresh-..."
./maximize --server-only
```

✅ **Best for**: When you already have tokens
✅ **Pros**: No exchange needed, immediate start
❌ **Cons**: Need to get tokens somehow first

### Method 3: Interactive CLI

```bash
# Use interactive CLI
./maximize
# Select option 2 (Login)
# Server reads tokens from ~/.maximize/tokens.json
```

✅ **Best for**: Local development, initial setup
✅ **Pros**: Easy, guided, browser-based
❌ **Cons**: Requires interactive terminal

## Comparison

| Method | Setup Steps | CLI Needed? | Best For |
|--------|-------------|-------------|----------|
| Auto-Exchange | 1 | ❌ No | Production |
| Direct Tokens | 2 | ❌ No | CI/CD |
| Interactive CLI | 1 | ✅ Yes | Development |

## Troubleshooting

### "Failed to exchange authorization code"

**Problem**: Code is invalid or expired

**Solution**: 
1. Get a fresh auth code from the OAuth URL
2. Make sure you copied the ENTIRE string (including `#state`)
3. Set it immediately (codes expire quickly)

### "No PKCE verifier found"

**Problem**: The state doesn't match

**Solution**:
1. Make sure you include the full `CODE#STATE` format
2. Use the exact code from Anthropic (don't modify it)

### Token Exchange Works But API Still Fails

**Problem**: Tokens were exchanged but not loaded

**Solution**:
1. Restart the server
2. Or set tokens directly from `~/.maximize/tokens.json`

## Security Notes

⚠️ **The authorization code is sensitive!**

- It can be used to get access tokens
- Only use it once, then delete it
- Don't commit it to git
- Use secure environment variable management in production

## Quick Reference

```bash
# Get OAuth URL
curl http://localhost:8081/auth/status

# Or start server to see it
./maximize --server-only

# Set auth code
export MAXIMIZE_AUTHENTICATION_CODE="your-code#state"

# Restart server (auto-exchanges)
./maximize --server-only

# Check if tokens work
python test_api.py

# Clean up (optional)
unset MAXIMIZE_AUTHENTICATION_CODE
```

---

**This is the easiest way to deploy Maximize in production without any interactive steps!** 🚀
