# Troubleshooting Guide

Common issues and solutions for Maximize.

## Error: "Is a directory (os error 21)"

### Symptom
```
Error: Is a directory (os error 21)
```

This error repeats multiple times in the CLI loop.

### Cause
The token file path is pointing to a directory instead of a file.

### Solutions

#### Solution 1: Check Your Configuration

If you have a `config.json` file, check the `storage.token_file` setting:

**❌ Wrong:**
```json
{
  "storage": {
    "token_file": "~/.maximize"
  }
}
```

**✅ Correct:**
```json
{
  "storage": {
    "token_file": "~/.maximize/tokens.json"
  }
}
```

#### Solution 2: Check Environment Variable

If you set the `TOKEN_FILE` environment variable, ensure it points to a file:

**❌ Wrong:**
```bash
export TOKEN_FILE=~/.maximize
```

**✅ Correct:**
```bash
export TOKEN_FILE=~/.maximize/tokens.json
```

#### Solution 3: Remove Existing Directory

If `.maximize` exists as a file (not a directory), remove it:

**Windows:**
```cmd
del %USERPROFILE%\.maximize
```

**Unix/Linux/macOS:**
```bash
rm ~/.maximize  # If it's a file
```

Then restart Maximize - it will create the proper directory structure.

#### Solution 4: Use Default Path

Remove any custom configuration and let Maximize use defaults:

1. Delete or rename `config.json`
2. Unset `TOKEN_FILE` environment variable
3. Restart Maximize

Default path:
- **Windows:** `C:\Users\<username>\.maximize\tokens.json`
- **Unix/Linux/macOS:** `~/.maximize/tokens.json`

### Prevention

Always specify the **full path including filename** for token storage:
- ✅ `~/.maximize/tokens.json`
- ✅ `/home/user/.maximize/tokens.json`
- ✅ `C:\Users\user\.maximize\tokens.json`
- ❌ `~/.maximize`
- ❌ `/home/user/.maximize`

---

## Error: "Failed to read config file"

### Symptom
```
Error: Failed to read config file: config.json
```

### Cause
The `config.json` file has syntax errors.

### Solution

Validate your JSON:

1. Check for:
   - Missing commas
   - Extra commas (after last item)
   - Unquoted keys
   - Invalid values

2. Use a JSON validator:
   - https://jsonlint.com/
   - Or your code editor's JSON validation

3. Compare with `config.example.json`:
   ```bash
   cat config.example.json
   ```

---

## Error: "No valid token available"

### Symptom
```
Error: OAuth expired; please authenticate using the CLI
```

### Solution

1. Run the CLI:
   ```bash
   ./maximize  # or maximize.exe on Windows
   ```

2. Select option 2: "Login / Re-authenticate"

3. Complete the OAuth flow in your browser

4. Return to CLI and start the proxy (option 1)

### Note
Tokens expire after 1 hour. The proxy will auto-refresh them, but if that fails, you need to re-authenticate.

---

## Docker: Container Exits Immediately

### Symptom
```bash
docker-compose up
# Container exits right away
```

### Cause
The CLI needs an interactive terminal for authentication.

### Solution

Use attach mode for authentication:

```bash
# Start in background
docker-compose up -d

# Attach to container
docker attach maximize

# Complete authentication (option 2)
# Then detach: Ctrl+P, Ctrl+Q
```

Or use exec:

```bash
# Start in background
docker-compose up -d

# Run CLI in container
docker exec -it maximize /app/maximize

# Complete authentication
# Exit when done
```

---

## Port Already in Use

### Symptom
```
Error: Address already in use (os error 48)
```

### Solution

**Option 1: Change Port**

Edit `config.json`:
```json
{
  "server": {
    "port": 8082
  }
}
```

Or use environment variable:
```bash
PORT=8082 ./maximize
```

**Option 2: Kill Existing Process**

Find what's using port 8081:

**Windows:**
```cmd
netstat -ano | findstr :8081
taskkill /PID <PID> /F
```

**Unix/Linux/macOS:**
```bash
lsof -ti:8081 | xargs kill -9
```

---

## Cannot Open Browser

### Symptom
```
Warning: Could not open browser automatically
```

### Solution

The OAuth URL is displayed in the terminal. Copy and paste it into your browser manually:

```
https://claude.ai/oauth/authorize?code=true&client_id=...
```

Complete the authentication in the browser, then paste the authorization code back into the CLI.

---

## Token Refresh Failed

### Symptom
```
Error: Token refresh failed
```

### Cause
Refresh token has expired (usually after ~30 days).

### Solution

Re-authenticate:

1. Select option 2: "Login / Re-authenticate"
2. Complete OAuth flow again
3. New tokens will be issued

---

## Permission Denied (Unix/Linux)

### Symptom
```
Error: Permission denied (os error 13)
```

### Cause
Cannot write to token directory or config file.

### Solution

**Check Permissions:**
```bash
ls -la ~/.maximize/
```

**Fix Permissions:**
```bash
chmod 700 ~/.maximize
chmod 600 ~/.maximize/tokens.json
```

**Or Run as Current User:**
Make sure you're not running with `sudo` unnecessarily.

---

## Model Not Found

### Symptom
```
Error: Model 'xyz' not found
```

### Cause
You don't have access to that model with your Claude subscription.

### Solution

Use a model you have access to:
- `xs` - Claude Haiku (Pro/Max)
- `s` - Claude Sonnet 3.5 (Pro/Max)
- `m` - Claude Sonnet 3.7 (Pro/Max)
- `l` - Claude Sonnet 4 (Pro/Max)
- `xl` - Claude Opus 4 (Max only)
- `xxl` - Claude Opus 4.1 (Max only)

Check your subscription at: https://claude.ai/settings

---

## Debug Mode

When reporting issues, enable debug mode for detailed logs:

```bash
./maximize --debug
```

Or in Docker:
```yaml
environment:
  - LOG_LEVEL=debug
```

The debug output will help diagnose issues.

---

## Getting Help

1. **Check this troubleshooting guide** first
2. **Enable debug mode** to see detailed errors
3. **Check logs** for specific error messages
4. **Search existing issues** on GitHub
5. **Open a new issue** with:
   - Error message (full text)
   - Debug logs
   - Your OS and version
   - Steps to reproduce

---

## Common Pitfalls

### ❌ Don't:
- Point `TOKEN_FILE` to a directory
- Run multiple instances on same port
- Share your `tokens.json` file
- Commit `config.json` with tokens

### ✅ Do:
- Use full file paths with filename
- Keep tokens private
- Re-authenticate when tokens expire
- Use environment variables for sensitive data
- Check file permissions on Unix/Linux

---

**Still having issues?** Open an issue on GitHub with debug logs and we'll help you out!
