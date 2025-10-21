# Model Nickname Reference

Quick reference for Maximize model nicknames.

## Available Models

| Nickname | Full Model Name | Release Date | Description |
|----------|----------------|--------------|-------------|
| **xs** | claude-3-5-haiku-20241022 | Oct 2024 | Fastest, most cost-effective |
| **s** | claude-3-5-sonnet-20241022 | Oct 2024 | Balanced speed and intelligence |
| **m** | claude-3-7-sonnet-20250219 | Feb 2025 | Enhanced Sonnet variant |
| **l** | claude-sonnet-4-20250514 | May 2025 | Latest Sonnet, production default |
| **xl** | claude-opus-4-20250514 | May 2025 | Most capable, research quality |
| **xxl** | claude-opus-4-1-20250805 | Aug 2025 | Latest Opus, maximum capability |

## Size Chart

```
xs   ▓░░░░░░░░░  Haiku (Fastest)
s    ▓▓▓░░░░░░░  Sonnet 3.5
m    ▓▓▓▓░░░░░░  Sonnet 3.7
l    ▓▓▓▓▓░░░░░  Sonnet 4 (Default)
xl   ▓▓▓▓▓▓▓▓░░  Opus 4
xxl  ▓▓▓▓▓▓▓▓▓▓  Opus 4.1 (Maximum)
```

## Usage Examples

### Python
```python
from anthropic import Anthropic

client = Anthropic(
    api_key="dummy",
    base_url="http://localhost:8081"
)

# Use nickname
response = client.messages.create(
    model="l",  # or "xl" for most capable
    max_tokens=1024,
    messages=[{"role": "user", "content": "Hello!"}]
)
```

### JavaScript
```javascript
import Anthropic from '@anthropic-ai/sdk';

const client = new Anthropic({
  apiKey: 'dummy',
  baseURL: 'http://localhost:8081',
});

const message = await client.messages.create({
  model: 'l',  // or 'xl' for most capable
  max_tokens: 1024,
  messages: [{ role: 'user', content: 'Hello!' }],
});
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

## Choosing the Right Model

### Use **xs** (Haiku) when:
- Speed is critical
- Simple tasks
- High volume of requests
- Cost is a concern

### Use **s** (Sonnet 3.5) when:
- Balanced performance needed
- Standard conversation
- General purpose tasks

### Use **m** (Sonnet 3.7) when:
- Enhanced reasoning needed
- More complex tasks
- Better than 3.5, faster than 4

### Use **l** (Sonnet 4) when:
- Production workloads
- General purpose default
- Good balance of speed and capability
- **This is the recommended default**

### Use **xl** (Opus 4) when:
- Maximum capability needed
- Complex reasoning required
- Research and analysis
- Deep understanding needed

### Use **xxl** (Opus 4.1) when:
- Absolute best performance required
- Most complex tasks
- Latest improvements needed
- Budget allows for premium

## Backward Compatibility

Full model names still work:

```python
# These are equivalent:
model="l"
model="claude-sonnet-4-20250514"

# These are equivalent:
model="xl"
model="claude-opus-4-20250514"
```

## Configuration Default

Set your default model in `config.json`:

```json
{
  "models": {
    "default": "l"
  }
}
```

Or via environment variable:

```bash
export DEFAULT_MODEL=l
```

## Pro Tips

1. **Start with 'l' (Sonnet 4)** - Best balance for most tasks
2. **Use 'xs' for iterations** - When you need many quick responses
3. **Use 'xl' for final work** - When quality matters most
4. **Mix models** - Use appropriate model for each task
5. **Monitor usage** - Higher models use more resources

## Quick Command

Print this reference anytime:

```bash
# Create alias
alias models='cat ~/maximize/MODELS.md'

# Use it
models
```

---

**Remember:** All models require an active Claude Pro or Max subscription.
The nicknames work exactly like full model names - just shorter!
