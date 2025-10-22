#!/bin/bash

# Token Extractor for Maximize
# Extracts tokens from ~/.maximize/tokens.json and shows export commands

set -e

TOKENS_FILE="$HOME/.maximize/tokens.json"

echo "🔑 Maximize Token Extractor"
echo "======================================"
echo

# Check if tokens file exists
if [ ! -f "$TOKENS_FILE" ]; then
    echo "❌ No tokens found at $TOKENS_FILE"
    echo
    echo "Run maximize CLI first to authenticate:"
    echo "   ./maximize"
    echo "   Select option 2 (Login)"
    echo
    exit 1
fi

echo "✅ Found tokens file"
echo

# Check if jq is available
if ! command -v jq &> /dev/null; then
    echo "⚠️  jq not found, showing raw JSON:"
    cat "$TOKENS_FILE"
    echo
    echo "Install jq for better output: apt-get install jq (or brew install jq)"
    exit 0
fi

# Extract tokens using jq
ACCESS_TOKEN=$(jq -r '.access_token' "$TOKENS_FILE")
REFRESH_TOKEN=$(jq -r '.refresh_token' "$TOKENS_FILE")
EXPIRES_AT=$(jq -r '.expires_at' "$TOKENS_FILE")

if [ -z "$ACCESS_TOKEN" ] || [ "$ACCESS_TOKEN" == "null" ]; then
    echo "❌ Failed to extract access_token"
    exit 1
fi

if [ -z "$REFRESH_TOKEN" ] || [ "$REFRESH_TOKEN" == "null" ]; then
    echo "❌ Failed to extract refresh_token"
    exit 1
fi

# Show expiration time
if [ "$EXPIRES_AT" != "null" ]; then
    echo "📅 Token expires at:"
    date -d @$EXPIRES_AT 2>/dev/null || date -r $EXPIRES_AT 2>/dev/null || echo "   Timestamp: $EXPIRES_AT"
    echo
fi

# Show export commands
echo "📋 Copy and paste these commands:"
echo "======================================"
echo
echo "export MAXIMIZE_ACCESS_TOKEN=\"$ACCESS_TOKEN\""
echo "export MAXIMIZE_REFRESH_TOKEN=\"$REFRESH_TOKEN\""
echo
echo "======================================"
echo

# Offer to write to .env file
read -p "Would you like to save to .env file? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    cat > .env << EOF
# Maximize Environment Variables
# Generated on $(date)

# OAuth Tokens (from ~/.maximize/tokens.json)
MAXIMIZE_ACCESS_TOKEN="$ACCESS_TOKEN"
MAXIMIZE_REFRESH_TOKEN="$REFRESH_TOKEN"
MAXIMIZE_TOKEN_EXPIRES_IN=86400

# API Key (generate with: openssl rand -hex 32)
# MAXIMIZE_API_KEY="your-api-key-here"

# Server Configuration
# MAXIMIZE_PORT=8081
# MAXIMIZE_BIND_ADDRESS=0.0.0.0
EOF
    
    echo "✅ Tokens saved to .env file"
    echo
    echo "Now run:"
    echo "   source .env"
    echo "   ./maximize --server-only"
else
    echo "Run the export commands above in your terminal, then:"
    echo "   ./maximize --server-only"
fi

echo
