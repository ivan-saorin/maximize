# 🚀 Maximize API Server - Ready for Production!

## ✅ What's Been Added

### 1. **Server-Only Mode** (`--server-only`)
- Runs as a pure API server without interactive CLI
- Perfect for containers and production deployments
- Auto-starts on container launch

### 2. **Environment Variable Token Injection**
- `MAXIMIZE_ACCESS_TOKEN` - Your Claude Max access token
- `MAXIMIZE_REFRESH_TOKEN` - Your Claude Max refresh token  
- `MAXIMIZE_TOKEN_EXPIRES_IN` - Token expiry (default: 24h)
- No file mounting required!

### 3. **API Key Authentication**
- `MAXIMIZE_API_KEY` - Protect your proxy with a secret key
- Supports both `Authorization: Bearer <key>` and `X-Api-Key` headers
- Optional but **strongly recommended** for production

### 4. **Production Docker Setup**
- Multi-stage build for minimal image size
- Non-root user for security
- Health check endpoint
- Optimized Rust release build

### 5. **CapRover Ready**
- `captain-definition` file for easy deployment
- Comprehensive deployment guide (CAPROVER.md)
- Token extraction helper (extract-tokens.bat)

## 📦 New Files Created

```
maximize/
├── Dockerfile              # Production-ready multi-stage build
├── captain-definition      # CapRover deployment config
├── .dockerignore          # Optimized build context
├── CAPROVER.md            # Complete deployment guide
└── extract-tokens.bat     # Token extraction helper
```

## 🎯 Quick Start - Deploy to CapRover

### Step 1: Get Your Tokens

```bash
# Run CLI locally to authenticate
cargo run

# Select option 2: Login
# Complete OAuth flow
```

### Step 2: Extract Tokens

**Windows:**
```cmd
extract-tokens.bat
```

**Linux/Mac:**
```bash
cat ~/.maximize/tokens.json
```

### Step 3: Deploy to CapRover

```bash
# Initialize CapRover (first time only)
npm install -g caprover
caprover login

# Deploy
caprover deploy
```

### Step 4: Configure Environment Variables

In CapRover dashboard → Your App → App Configs → Environment Variables:

```
MAXIMIZE_ACCESS_TOKEN=sk-ant-api03-...
MAXIMIZE_REFRESH_TOKEN=refresh_...
MAXIMIZE_API_KEY=your-super-secret-key-here
```

### Step 5: Enable HTTPS

In CapRover dashboard → Your App → HTTP Settings → Enable HTTPS

## 🔧 Using Your API

### Python Example

```python
import anthropic

client = anthropic.Anthropic(
    api_key="your-super-secret-key",  # Your MAXIMIZE_API_KEY
    base_url="https://maximize.yourdomain.com"
)

response = client.messages.create(
    model="l",  # claude-sonnet-4
    max_tokens=1024,
    messages=[{"role": "user", "content": "Hello!"}]
)
```

### JavaScript Example

```javascript
import Anthropic from '@anthropic-ai/sdk';

const anthropic = new Anthropic({
  apiKey: 'your-super-secret-key',
  baseURL: 'https://maximize.yourdomain.com'
});

const message = await anthropic.messages.create({
  model: 'l',
  max_tokens: 1024,
  messages: [{role: 'user', content: 'Hello!'}]
});
```

### cURL Example

```bash
curl https://maximize.yourdomain.com/v1/messages \
  -H "Authorization: Bearer your-super-secret-key" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "l",
    "max_tokens": 1024,
    "messages": [{"role": "user", "content": "Hello!"}]
  }'
```

## 🎨 Model Nicknames

| Nickname | Full Model Name |
|----------|----------------|
| `xs` | claude-3-5-haiku-20241022 |
| `s` | claude-3-5-sonnet-20241022 |
| `m` | claude-3-7-sonnet-20250219 |
| `l` ⭐ | claude-sonnet-4-20250514 (default) |
| `xl` | claude-opus-4-20250514 |
| `xxl` | claude-opus-4-1-20250805 |

## 🔒 Security Features

✅ **API Key Authentication** - Protect your proxy  
✅ **Environment-based Secrets** - No hardcoded credentials  
✅ **Non-root Container** - Security best practice  
✅ **HTTPS Support** - Encrypted traffic via CapRover  
✅ **Automatic Token Refresh** - No manual intervention  

## 📊 Monitoring

### Health Check
```bash
curl https://maximize.yourdomain.com/healthz
```

### Token Status  
```bash
curl https://maximize.yourdomain.com/auth/status
```

### Logs
```bash
caprover logs -a maximize --lines 100
```

## 🎯 Use Cases Unlocked

1. **Multiple Projects** - Deploy separate instances with different API keys
2. **Team Access** - Share one proxy with different API keys per team
3. **Development** - Local CLI mode for testing, production API for apps
4. **CI/CD** - Automated testing with Claude Max models
5. **Cost Savings** - Use your subscription instead of pay-per-token API

## 🔥 Production Best Practices

1. ✅ Always set `MAXIMIZE_API_KEY`
2. ✅ Enable HTTPS in CapRover
3. ✅ Set resource limits (256MB RAM minimum)
4. ✅ Monitor logs regularly
5. ✅ Rotate tokens every few weeks
6. ✅ Use strong random API keys: `openssl rand -hex 32`

## 📖 Documentation

- **CAPROVER.md** - Complete deployment guide with troubleshooting
- **README.md** - Original CLI documentation
- **Dockerfile** - Container configuration with comments

## 🚀 Next Steps

1. **Deploy to CapRover** - Follow the quick start above
2. **Test the API** - Use the examples to verify it works
3. **Integrate with your projects** - Replace Anthropic API URLs
4. **Share with your team** - Give them the proxy URL and their API keys
5. **Monitor usage** - Check logs and token status regularly

## 💡 Tips

- Generate strong API keys: `openssl rand -hex 32`
- Test locally first: `cargo run --server-only`
- Multiple instances? Deploy with different names: `caprover deploy -a project-a-maximize`
- Token expired? Re-authenticate locally and update env vars

## 🎉 You're All Set!

Your Maximize proxy is now a full-fledged API server ready for production use. Deploy it to CapRover and start using Claude Max models in all your projects! 🚀

---

**Questions?** Check CAPROVER.md for detailed troubleshooting and advanced configuration.
