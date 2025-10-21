# Usage Guide

## Installation

### From Source

```bash
# Clone the repository
git clone <repository-url>
cd maximize

# Build release binary
cargo build --release

# Binary will be at: target/release/maximize (or maximize.exe on Windows)
```

### Using Pre-built Binary

Download the latest binary for your platform from the releases page.

## First Time Setup

1. **Run the CLI:**
   ```bash
   ./target/release/maximize
   ```

2. **Login (Option 2):**
   - Browser will open automatically
   - Login with your Claude Pro/Max account
   - Authorize the application
   - Copy the authorization code from the browser
   - Paste it into the CLI

3. **Start the Proxy (Option 1):**
   - Server will start on http://0.0.0.0:8081 by default
   - Proxy is now ready to receive requests

## Using with API Clients

### Python (with anthropic library)

```python
from anthropic import Anthropic

client = Anthropic(
    api_key="dummy",  # Any non-empty string
    base_url="http://localhost:8081"
)

# Use model nicknames
message = client.messages.create(
    model="l",  # or xs, s, m, xl, xxl
    max_tokens=1024,
    messages=[
        {"role": "user", "content": "Hello!"}
    ]
)

print(message.content)
```

### JavaScript/TypeScript

```javascript
import Anthropic from '@anthropic-ai/sdk';

const client = new Anthropic({
  apiKey: 'dummy',
  baseURL: 'http://localhost:8081',
});

async function main() {
  const message = await client.messages.create({
    model: 'l',  // or xs, s, m, xl, xxl
    max_tokens: 1024,
    messages: [
      { role: 'user', content: 'Hello!' }
    ],
  });
  
  console.log(message.content);
}

main();
```

### cURL

```bash
curl -X POST http://localhost:8081/v1/messages \
  -H "Content-Type: application/json" \
  -H "x-api-key: dummy" \
  -H "anthropic-version: 2023-06-01" \
  -d '{
    "model": "l",
    "max_tokens": 1024,
    "messages": [
      {"role": "user", "content": "Hello!"}
    ]
  }'
```

## Model Nicknames

| Nickname | Full Model Name |
|----------|----------------|
| xs | claude-3-5-haiku-20241022 |
| s | claude-3-5-sonnet-20241022 |
| m | claude-3-7-sonnet-20250219 |
| l | claude-sonnet-4-20250514 |
| xl | claude-opus-4-20250514 |
| xxl | claude-opus-4-1-20250805 |

You can use either the nickname or full model name in your requests.

## Configuration

Create a `config.json` file in the project directory:

```json
{
  "server": {
    "port": 8081,
    "log_level": "info",
    "bind_address": "0.0.0.0"
  },
  "models": {
    "default": "l"
  },
  "api": {
    "request_timeout": 120
  },
  "storage": {
    "token_file": "~/.maximize/tokens.json"
  }
}
```

## Environment Variables

You can also configure via environment variables:

```bash
export PORT=8081
export LOG_LEVEL=debug
export BIND_ADDRESS=127.0.0.1
export DEFAULT_MODEL=l
export REQUEST_TIMEOUT=120
export TOKEN_FILE=~/.maximize/tokens.json

./maximize
```

## Command Line Options

```bash
# Run with debug logging
./maximize --debug

# Bind to specific address
./maximize --bind 127.0.0.1

# Show help
./maximize --help
```

## CLI Menu Options

1. **Start/Stop Proxy Server** - Toggle the proxy server on/off
2. **Login / Re-authenticate** - Perform OAuth authentication
3. **Refresh Token** - Manually refresh your access token
4. **Show Token Status** - Display token information and expiry
5. **Logout (Clear Tokens)** - Remove stored tokens
6. **Exit** - Quit the application

## Streaming Responses

The proxy fully supports streaming:

```python
from anthropic import Anthropic

client = Anthropic(
    api_key="dummy",
    base_url="http://localhost:8081"
)

with client.messages.stream(
    model="l",
    max_tokens=1024,
    messages=[{"role": "user", "content": "Write a story"}]
) as stream:
    for text in stream.text_stream:
        print(text, end="", flush=True)
```

## Extended Thinking

To use Claude's extended thinking:

```python
message = client.messages.create(
    model="l",
    max_tokens=16000,
    thinking={
        "type": "enabled",
        "budget_tokens": 10000
    },
    messages=[
        {"role": "user", "content": "Solve this complex problem..."}
    ]
)
```

## Troubleshooting

### Token Expired
- Use option 3 (Refresh Token) in the CLI
- Or re-authenticate with option 2 (Login)

### Server Won't Start
- Check if another service is using port 8081
- Try binding to a different port: `./maximize --bind 127.0.0.1:8082`
- Check your token status with option 4

### Browser Won't Open
- Manually copy the URL shown in the CLI
- Open it in your browser
- Complete the authentication process

### API Errors
- Verify your Claude Pro/Max subscription is active
- Check token status (option 4)
- Enable debug logging: `./maximize --debug`

## Performance Tips

1. **Use Release Build** - Always compile with `--release` flag
2. **Keep Token Fresh** - The proxy auto-refreshes but manual refresh helps
3. **Monitor Logs** - Use debug mode to troubleshoot issues
4. **Bind Locally** - If only using locally, bind to 127.0.0.1 for security

## Security Notes

- Tokens are stored in `~/.maximize/tokens.json` with restricted permissions (Unix)
- Never share your token file
- Never commit config.json with sensitive data
- The proxy acts as a bridge to Anthropic's servers
- All requests still go through Anthropic's infrastructure
