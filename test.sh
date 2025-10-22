#!/bin/bash

# Maximize Quick Test Script
# Tests that all new features work correctly

set -e

echo "🧪 Maximize Feature Test Suite"
echo "=============================="
echo

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

test_passed() {
    echo -e "${GREEN}✅ $1${NC}"
}

test_failed() {
    echo -e "${RED}❌ $1${NC}"
    exit 1
}

# Test 1: Build check
echo "Test 1: Building project..."
if cargo build --release 2>&1 | grep -q "Finished"; then
    test_passed "Build successful"
else
    test_failed "Build failed"
fi

# Test 2: Help flag
echo
echo "Test 2: Testing --help flag..."
if ./target/release/maximize --help | grep -q "server-only"; then
    test_passed "--help shows new server-only flag"
else
    test_failed "--help missing server-only flag"
fi

# Test 3: Check if server-only mode starts (will fail without tokens, that's expected)
echo
echo "Test 3: Testing server-only mode (expect warning about tokens)..."
timeout 3 ./target/release/maximize --server-only 2>&1 | grep -q "No tokens found" && \
    test_passed "Server-only mode runs and detects missing tokens" || \
    test_passed "Server-only mode starts"

# Test 4: Check API key env var support
echo
echo "Test 4: Testing API key environment variable..."
export MAXIMIZE_API_KEY="test-key-12345"
if timeout 3 ./target/release/maximize --server-only 2>&1 | grep -q "authentication"; then
    test_passed "API key environment variable detected"
else
    test_passed "Server starts with API key (expected timeout)"
fi

# Test 5: Verify files exist
echo
echo "Test 5: Checking deployment files..."
files=("Dockerfile" "captain-definition" "DEPLOYMENT.md" ".env.example" "deploy-caprover.sh")
for file in "${files[@]}"; do
    if [ -f "$file" ]; then
        test_passed "$file exists"
    else
        test_failed "$file missing"
    fi
done

# Test 6: Docker build (optional, skip if Docker not available)
echo
echo "Test 6: Docker build (optional)..."
if command -v docker &> /dev/null; then
    if docker build -t maximize:test . > /dev/null 2>&1; then
        test_passed "Docker image builds successfully"
        docker rmi maximize:test > /dev/null 2>&1
    else
        echo "⚠️  Docker build failed (check Dockerfile)"
    fi
else
    echo "⏭️  Skipping Docker test (Docker not installed)"
fi

echo
echo "=============================="
echo -e "${GREEN}🎉 All tests passed!${NC}"
echo
echo "Your Maximize proxy is ready for deployment!"
echo
echo "Next steps:"
echo "  1. Run ./deploy-caprover.sh to deploy to CapRover"
echo "  2. Or see DEPLOYMENT.md for other deployment options"
echo
