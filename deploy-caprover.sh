#!/bin/bash

# Maximize CapRover Deployment Helper
# This script helps you deploy Maximize to CapRover with proper configuration

set -e

echo "🚀 Maximize CapRover Deployment Helper"
echo "======================================"
echo

# Check if caprover CLI is installed
if ! command -v caprover &> /dev/null; then
    echo "❌ CapRover CLI not found. Install it with:"
    echo "   npm install -g caprover"
    exit 1
fi

# Check for tokens file
TOKENS_FILE="$HOME/.maximize/tokens.json"
if [ ! -f "$TOKENS_FILE" ]; then
    echo "❌ No tokens found at $TOKENS_FILE"
    echo
    echo "Please run maximize locally first to authenticate:"
    echo "   ./maximize"
    echo "   Select option 2 (Login)"
    echo
    exit 1
fi

echo "✅ Found tokens file"
echo

# Extract tokens
ACCESS_TOKEN=$(jq -r '.access_token' "$TOKENS_FILE")
REFRESH_TOKEN=$(jq -r '.refresh_token' "$TOKENS_FILE")
EXPIRES_AT=$(jq -r '.expires_at' "$TOKENS_FILE")

if [ -z "$ACCESS_TOKEN" ] || [ "$ACCESS_TOKEN" == "null" ]; then
    echo "❌ Failed to extract access_token from $TOKENS_FILE"
    exit 1
fi

echo "✅ Tokens extracted successfully"
echo "   Expires at: $(date -d @$EXPIRES_AT 2>/dev/null || date -r $EXPIRES_AT 2>/dev/null || echo $EXPIRES_AT)"
echo

# Generate API key if not provided
if [ -z "$MAXIMIZE_API_KEY" ]; then
    API_KEY=$(openssl rand -hex 32)
    echo "🔐 Generated new API key:"
    echo "   $API_KEY"
    echo
    echo "   Save this key securely! You'll need it for API requests."
    echo
else
    API_KEY="$MAXIMIZE_API_KEY"
    echo "🔐 Using provided API key"
    echo
fi

# Ask for app name
read -p "Enter CapRover app name (default: maximize): " APP_NAME
APP_NAME=${APP_NAME:-maximize}

echo
echo "📦 Deploying to CapRover..."
echo "   App name: $APP_NAME"
echo

# Deploy
caprover deploy -a "$APP_NAME"

echo
echo "✅ Deployment initiated!"
echo
echo "📋 Next steps:"
echo "   1. Go to CapRover web interface"
echo "   2. Navigate to Apps → $APP_NAME → App Configs"
echo "   3. Add these environment variables:"
echo
echo "      MAXIMIZE_ACCESS_TOKEN=$ACCESS_TOKEN"
echo "      MAXIMIZE_REFRESH_TOKEN=$REFRESH_TOKEN"
echo "      MAXIMIZE_TOKEN_EXPIRES_IN=86400"
echo "      MAXIMIZE_API_KEY=$API_KEY"
echo
echo "   4. Click 'Save & Update'"
echo "   5. Enable HTTPS in HTTP Settings (recommended)"
echo "   6. Test with: curl https://$APP_NAME.yourdomain.com/healthz"
echo
echo "🎉 Done! Your Maximize proxy will be live shortly."
echo
echo "📝 Client configuration:"
echo "   Base URL: https://$APP_NAME.yourdomain.com"
echo "   API Key: $API_KEY"
echo "   Model: l (or xs, s, m, xl, xxl)"
echo
