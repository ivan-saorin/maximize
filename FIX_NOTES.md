## Fixed: "Is a directory (os error 21)"

The error has been fixed with the following improvements:

### Changes Made

1. **Enhanced Path Validation** (`storage.rs`)
   - Added check to detect if token path points to a directory
   - Provides clear error message with suggested fix
   - Validates path on initialization

2. **Tilde Expansion** (`config_loader.rs`)
   - Added `expand_tilde()` function to properly expand `~` in paths
   - Works on all platforms (Windows, Linux, macOS)

3. **Automatic Directory Detection** (`config_loader.rs`)
   - If TOKEN_FILE points to a directory, automatically appends `tokens.json`
   - Warns user about the correction
   - Prevents the error from happening

4. **Config File Validation** (`config_loader.rs`)
   - Checks if config.json is actually a directory
   - Skips gracefully with warning instead of crashing

5. **Better Error Messages** (`storage.rs`)
   - Clear explanation of what went wrong
   - Suggests correct path format
   - Shows platform-specific path separator

### How to Use

After rebuilding, the proxy will:

1. **Auto-fix common mistakes:**
   ```json
   // In config.json - both now work:
   "token_file": "~/.maximize"  // Auto-fixed to ~/.maximize/tokens.json
   "token_file": "~/.maximize/tokens.json"  // Works as-is
   ```

2. **Show helpful warnings:**
   ```
   Warning: TOKEN_FILE '~/.maximize' is a directory. Using '~/.maximize/tokens.json' instead.
   ```

3. **Clear error if unfixable:**
   ```
   Error: Token file path '/path/to/dir' is a directory.
   Please specify a file path like: /path/to/dir/tokens.json
   ```

### Rebuild

```bash
# Clean build
cargo clean

# Rebuild release
cargo build --release

# Or using shortcuts:
make clean && make          # Unix
del /s /q target && build.bat  # Windows
```

### Verify Fix

The error should no longer occur. If you had a misconfigured path:

**Before:**
```
Error: Is a directory (os error 21)
Error: Is a directory (os error 21)
Error: Is a directory (os error 21)
...
```

**After:**
```
Warning: TOKEN_FILE '~/.maximize' is a directory. Using '~/.maximize/tokens.json' instead.
[Normal operation continues]
```

### Documentation Added

Created `TROUBLESHOOTING.md` with comprehensive guide covering:
- "Is a directory" error (your issue)
- Configuration mistakes
- Token issues
- Docker problems
- Port conflicts
- Permission errors
- And more...

### Prevention

The code now:
- ✅ Validates paths on startup
- ✅ Auto-corrects common mistakes
- ✅ Expands `~` properly
- ✅ Provides clear error messages
- ✅ Prevents crashes from misconfiguration

Try rebuilding and running again. The error should be resolved!
