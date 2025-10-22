# Fixing "Invalid bearer token" Errors 🔧

## The Problem

You're seeing this error:
```
Error code: 401 - {'error': {'message': 'Invalid bearer token', 'type': 'authentication_error'}}
```

This means:
- ✅ Your proxy is working correctly
- ✅ API key authentication is working
- ❌ Your OAuth access token is **invalid or expired**

## The Solution

### Step 1: Get Fresh Tokens

Run maximize CLI locally to authenticate:

```bash
# Windows
maximize.exe

# Linux/Mac
./maximize
```

**In the CLI menu:**
1. Select option 2 (Login / Re-authenticate)
2. Browser will open to Claude OAuth
3. Complete authentication
4. Paste the authorization code back into CLI

You should see: `✅ Tokens obtained successfully`

### Step 2: Extract Your Tokens

#### Option A: Use Helper Script (Linux/Mac)

```bash
chmod +x extract-tokens.sh
./extract-tokens.sh
```

This will show you the export commands or offer to create a `.env` file.

#### Option B: Use Helper Script (Windows)

```cmd
extract-tokens.bat
```

#### Option C: Manual Extraction

```bash
# Linux/Mac
cat ~/.maximize/tokens.json

# Windows (PowerShell)
cat $env:USERPROFILE\.maximize\tokens.json

# Windows (CMD)
type %USERPROFILE%\.maximize\tokens.json
```

You'll see something like:
```json
{
  "access_token": "sk-ant-sid01-aB...xyz",
  "refresh_token": "refresh_sid01-cD...uvw",
  "expires_at": 1234567890
}
```

### Step 3: Set Environment Variables

Copy your tokens from Step 2, then:

#### Linux/Mac:

```bash
export MAXIMIZE_ACCESS_TOKEN="sk-ant-sid01-aB...xyz"
export MAXIMIZE_REFRESH_TOKEN="refresh_sid01-cD...uvw"
export MAXIMIZE_API_KEY="your-api-key"  # If using API auth
```

Or create `.env` file:

```bash
# .env
MAXIMIZE_ACCESS_TOKEN="sk-ant-sid01-aB...xyz"
MAXIMIZE_REFRESH_TOKEN="refresh_sid01-cD...uvw"
MAXIMIZE_API_KEY="your-api-key"
```

#### Windows (PowerShell):

```powershell
$env:MAXIMIZE_ACCESS_TOKEN="sk-ant-sid01-aB...xyz"
$env:MAXIMIZE_REFRESH_TOKEN="refresh_sid01-cD...uvw"
$env:MAXIMIZE_API_KEY="your-api-key"
```

#### Windows (CMD):

```cmd
set MAXIMIZE_ACCESS_TOKEN="sk-ant-sid01-aB...xyz"
set MAXIMIZE_REFRESH_TOKEN="refresh_sid01-cD...uvw"
set MAXIMIZE_API_KEY="your-api-key"
```

### Step 4: Restart Server

```bash
# Linux/Mac
./maximize --server-only

# Windows
maximize.exe --server-only

# Or with cargo
cargo run --release -- --server-only
```

### Step 5: Test Again

```bash
python test_api.py
```

You should now see: `🎉 All tests passed!`

## Why This Happens

OAuth access tokens expire after ~24 hours. Your proxy will **automatically refresh** them using the refresh token, but if BOTH tokens are invalid (old authentication session), you need to re-authenticate.

## Docker/CapRover Deployments

For production deployments, set environment variables in your deployment platform:

### CapRover

1. Go to Apps → your-app → App Configs
2. Add Environment Variables:
   - `MAXIMIZE_ACCESS_TOKEN`: (your token)
   - `MAXIMIZE_REFRESH_TOKEN`: (your token)
   - `MAXIMIZE_API_KEY`: (your API key)
3. Click "Save & Update"

### Docker

```bash
docker run -d \
  -e MAXIMIZE_ACCESS_TOKEN="sk-ant-..." \
  -e MAXIMIZE_REFRESH_TOKEN="refresh-..." \
  -e MAXIMIZE_API_KEY="your-key" \
  -p 8081:8081 \
  maximize
```

### Docker Compose

```yaml
services:
  maximize:
    image: maximize
    environment:
      - MAXIMIZE_ACCESS_TOKEN=sk-ant-...
      - MAXIMIZE_REFRESH_TOKEN=refresh-...
      - MAXIMIZE_API_KEY=your-key
    ports:
      - "8081:8081"
```

## Quick Reference

| Issue | Solution |
|-------|----------|
| "Invalid bearer token" | Re-authenticate via CLI, get fresh tokens |
| "No tokens found" | Set `MAXIMIZE_ACCESS_TOKEN` and `MAXIMIZE_REFRESH_TOKEN` |
| "Invalid API key" | Check `MAXIMIZE_API_KEY` matches in server and client |
| Server starts but API fails | Verify tokens are valid (not expired) |
| CLI auth works, server doesn't | Make sure to export tokens to environment |

## Need More Help?

1. Check server logs: Look for authentication errors
2. Test health endpoint: `curl http://localhost:8081/healthz`
3. Check auth status: `curl http://localhost:8081/auth/status`
4. See full docs: [DEPLOYMENT.md](DEPLOYMENT.md)

## Token Security

⚠️ **Important**: These tokens give access to your Claude subscription!

- Never commit tokens to git (use `.env` and `.gitignore`)
- Use API keys to protect your proxy
- Rotate tokens periodically
- Use environment variables in production
- Keep token files secure (`chmod 600` on Linux/Mac)
