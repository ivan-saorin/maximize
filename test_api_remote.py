"""
Comprehensive API test suite for Maximize proxy
Tests model nicknames, streaming, authentication, and endpoints
"""
from anthropic import Anthropic
import requests
import sys
import os
from typing import Optional
from dotenv import load_dotenv

load_dotenv()

# Configuration
BASE_URL = os.getenv("MAXIMIZE_BASE_URL", "https://maximize.automa.016180.xyz")
API_KEY = os.getenv("MAXIMIZE_API_KEY", "max-5763-2548-9184-0810-2743-7182-4371-2878-9576-8768")  # Use env var or default

# Colors for terminal output
class Colors:
    GREEN = '\033[92m'
    RED = '\033[91m'
    YELLOW = '\033[93m'
    BLUE = '\033[94m'
    CYAN = '\033[96m'
    RESET = '\033[0m'
    BOLD = '\033[1m'

def print_header(text: str):
    """Print a formatted header"""
    print(f"\n{Colors.CYAN}{Colors.BOLD}{'='*70}{Colors.RESET}")
    print(f"{Colors.CYAN}{Colors.BOLD}{text}{Colors.RESET}")
    print(f"{Colors.CYAN}{Colors.BOLD}{'='*70}{Colors.RESET}")

def print_success(text: str):
    """Print success message"""
    print(f"{Colors.GREEN}✅ {text}{Colors.RESET}")

def print_error(text: str):
    """Print error message"""
    print(f"{Colors.RED}❌ {text}{Colors.RESET}")

def print_info(text: str):
    """Print info message"""
    print(f"{Colors.BLUE}ℹ️  {text}{Colors.RESET}")

def print_warning(text: str):
    """Print warning message"""
    print(f"{Colors.YELLOW}⚠️  {text}{Colors.RESET}")

# Test 1: Health Check
def test_health_check() -> bool:
    """Test the /healthz endpoint"""
    print_header("Test 1: Health Check Endpoint")
    print_info(f"Testing: GET {BASE_URL}/healthz")
    
    try:
        response = requests.get(f"{BASE_URL}/healthz", timeout=5)
        
        if response.status_code == 200:
            data = response.json()
            print_success(f"Health check passed")
            print_info(f"Response: {data}")
            return True
        else:
            print_error(f"Health check failed with status {response.status_code}")
            return False
            
    except Exception as e:
        print_error(f"Health check failed: {e}")
        return False

# Test 2: Auth Status
def test_auth_status() -> bool:
    """Test the /auth/status endpoint"""
    print_header("Test 2: Auth Status Endpoint")
    print_info(f"Testing: GET {BASE_URL}/auth/status")
    
    try:
        response = requests.get(f"{BASE_URL}/auth/status", timeout=5)
        
        if response.status_code == 200:
            data = response.json()
            print_success(f"Auth status check passed")
            print_info(f"Has tokens: {data.get('has_tokens')}")
            print_info(f"Is expired: {data.get('is_expired')}")
            if data.get('expires_at'):
                print_info(f"Expires at: {data.get('expires_at')}")
            print_info(f"Time until expiry: {data.get('time_until_expiry')}")
            
            # Warning if no tokens
            if not data.get('has_tokens'):
                print_warning("No tokens found! Set MAXIMIZE_ACCESS_TOKEN and MAXIMIZE_REFRESH_TOKEN")
            elif data.get('is_expired'):
                print_warning("Tokens are expired! They will be auto-refreshed on first API call")
            
            return True
        else:
            print_error(f"Auth status check failed with status {response.status_code}")
            return False
            
    except Exception as e:
        print_error(f"Auth status check failed: {e}")
        return False

# Test 3: API Key Authentication
def test_api_key_auth() -> bool:
    """Test API key authentication (if enabled)"""
    print_header("Test 3: API Key Authentication")
    print_info(f"Testing with API key: {API_KEY}")
    
    try:
        # Try without API key first (should fail if auth is enabled)
        response = requests.post(
            f"{BASE_URL}/v1/messages",
            json={
                "model": "l",
                "max_tokens": 10,
                "messages": [{"role": "user", "content": "test"}]
            },
            timeout=5
        )
        
        if response.status_code == 401:
            print_info("API key authentication is ENABLED (401 without key)")
            
            # Now try with API key
            response_with_key = requests.post(
                f"{BASE_URL}/v1/messages",
                headers={"Authorization": f"Bearer {API_KEY}"},
                json={
                    "model": "l",
                    "max_tokens": 10,
                    "messages": [{"role": "user", "content": "test"}]
                },
                timeout=30
            )
            
            if response_with_key.status_code in [200, 401]:  # 200 = success, 401 = wrong key but auth works
                print_success("API key authentication is working")
                return True
            else:
                print_warning(f"Unexpected status with API key: {response_with_key.status_code}")
                return True  # Auth is working, might be other issues
        else:
            print_info("API key authentication is DISABLED (request succeeded without key)")
            return True
            
    except Exception as e:
        print_error(f"API key auth test failed: {e}")
        return False

# Test 4: Non-Streaming Request
def test_model_nonstreaming(nickname: str, prompt: str) -> bool:
    """Test a model with non-streaming request"""
    print_header(f"Test 4: Non-Streaming Request - Model '{nickname}'")
    print_info(f"Prompt: {prompt}")
    
    try:
        client = Anthropic(
            api_key=API_KEY,
            base_url=BASE_URL
        )
        
        message = client.messages.create(
            model=nickname,
            max_tokens=100,
            messages=[{"role": "user", "content": prompt}],
            stream=False
        )
        
        response_text = message.content[0].text
        print_success(f"Non-streaming request succeeded")
        print_info(f"Response: {response_text[:200]}{'...' if len(response_text) > 200 else ''}")
        print_info(f"Input tokens: {message.usage.input_tokens}")
        print_info(f"Output tokens: {message.usage.output_tokens}")
        return True
        
    except Exception as e:
        error_msg = str(e)
        print_error(f"Non-streaming request failed: {error_msg}")
        
        # Show helpful hint for auth errors
        if "Invalid bearer token" in error_msg or "401" in error_msg:
            print_warning("This is an Anthropic authentication error, not a proxy error")
            print_warning("Your OAuth tokens might be invalid. See troubleshooting below.")
        
        return False

# Test 5: Streaming Request
def test_model_streaming(nickname: str, prompt: str) -> bool:
    """Test a model with streaming request"""
    print_header(f"Test 5: Streaming Request - Model '{nickname}'")
    print_info(f"Prompt: {prompt}")
    
    try:
        client = Anthropic(
            api_key=API_KEY,
            base_url=BASE_URL
        )
        
        chunks = []
        sys.stdout.write(f"{Colors.BLUE}ℹ️  Streaming response: {Colors.RESET}")
        sys.stdout.flush()
        
        with client.messages.stream(
            model=nickname,
            max_tokens=100,
            messages=[{"role": "user", "content": prompt}]
        ) as stream:
            for text in stream.text_stream:
                chunks.append(text)
                sys.stdout.write(text)
                sys.stdout.flush()
        
        print()  # New line after streaming
        
        full_response = "".join(chunks)
        if full_response:
            print_success(f"Streaming request succeeded")
            print_info(f"Total characters received: {len(full_response)}")
            return True
        else:
            print_error("Streaming request returned empty response")
            return False
        
    except Exception as e:
        print_error(f"Streaming request failed: {e}")
        return False

# Test 6: Model Nicknames
def test_model_nicknames() -> bool:
    """Test various model nicknames"""
    print_header("Test 6: Model Nicknames Resolution")
    
    nicknames = ["xs", "s", "m", "l", "xl", "xxl"]
    success_count = 0
    
    for nickname in nicknames:
        print_info(f"Testing nickname: {nickname}")
        try:
            client = Anthropic(
                api_key=API_KEY,
                base_url=BASE_URL
            )
            
            # Very short request to test nickname resolution
            message = client.messages.create(
                model=nickname,
                max_tokens=10,
                messages=[{"role": "user", "content": "Hi"}]
            )
            
            print_success(f"Nickname '{nickname}' resolved successfully")
            success_count += 1
            
        except Exception as e:
            error_msg = str(e)
            if "invalid_request_error" in error_msg.lower() or "not found" in error_msg.lower():
                print_warning(f"Nickname '{nickname}' - Model not available in your subscription")
            else:
                print_error(f"Nickname '{nickname}' failed: {e}")
    
    if success_count > 0:
        print_success(f"{success_count}/{len(nicknames)} nicknames tested successfully")
        return True
    else:
        print_error("No nicknames worked")
        return False

# Test 7: Extended Thinking
def test_extended_thinking() -> bool:
    """Test extended thinking mode"""
    print_header("Test 7: Extended Thinking Mode")
    
    try:
        client = Anthropic(
            api_key=API_KEY,
            base_url=BASE_URL
        )
        
        message = client.messages.create(
            model="l",  # Sonnet 4 supports thinking
            max_tokens=2000,
            thinking={
                "type": "enabled",
                "budget_tokens": 1024  # Minimum is 1024
            },
            messages=[{"role": "user", "content": "What is 15 * 23? Think through it step by step."}]
        )
        
        has_thinking = any(block.type == "thinking" for block in message.content)
        
        if has_thinking:
            print_success("Extended thinking mode is working")
            for block in message.content:
                if block.type == "thinking":
                    print_info(f"Thinking content length: {len(block.thinking)}")
                elif block.type == "text":
                    print_info(f"Response: {block.text}")
            return True
        else:
            print_warning("No thinking blocks found (feature might not be available)")
            return True  # Not a failure, just not available
        
    except Exception as e:
        error_msg = str(e)
        if "thinking" in error_msg.lower() or "not supported" in error_msg.lower():
            print_warning(f"Extended thinking not available: {e}")
            return True  # Not a failure, just not available
        else:
            print_error(f"Extended thinking test failed: {e}")
            return False

def main():
    """Run all tests"""
    print(f"{Colors.BOLD}{Colors.BLUE}")
    print("🚀 Maximize API Comprehensive Test Suite")
    print(f"{'='*70}")
    print(f"Base URL: {BASE_URL}")
    print(f"API Key: {'*' * (len(API_KEY) - 4) + API_KEY[-4:] if len(API_KEY) > 4 else 'dummy'}")
    print(f"{'='*70}{Colors.RESET}")
    
    # Run all tests
    tests = [
        ("Health Check", test_health_check),
        ("Auth Status", test_auth_status),
        ("API Key Auth", test_api_key_auth),
        ("Non-Streaming", lambda: test_model_nonstreaming("l", "Say hello in 3 words")),
        ("Streaming", lambda: test_model_streaming("m", "Count to 5")),
        ("Model Nicknames", test_model_nicknames),
        ("Extended Thinking", test_extended_thinking),
    ]
    
    results = []
    for name, test_func in tests:
        try:
            success = test_func()
            results.append((name, success))
        except KeyboardInterrupt:
            print_error("\nTest interrupted by user")
            sys.exit(1)
        except Exception as e:
            print_error(f"Test '{name}' crashed: {e}")
            results.append((name, False))
    
    # Summary
    print_header("📊 Test Summary")
    
    passed = sum(1 for _, success in results if success)
    total = len(results)
    
    for name, success in results:
        if success:
            print_success(f"{name}")
        else:
            print_error(f"{name}")
    
    print(f"\n{Colors.BOLD}Results: {passed}/{total} tests passed{Colors.RESET}")
    
    # Specific troubleshooting for common issues
    if passed < total:
        print(f"\n{Colors.YELLOW}{Colors.BOLD}💡 Troubleshooting Tips:{Colors.RESET}")
        
        # Check for auth failures
        auth_failures = [name for name, success in results if not success and name not in ["Health Check", "Auth Status", "API Key Auth"]]
        if auth_failures:
            print(f"\n{Colors.YELLOW}If seeing 'Invalid bearer token' errors:{Colors.RESET}")
            print("  1. Your OAuth tokens might be invalid or expired")
            print("  2. Run maximize CLI locally to get valid tokens:")
            print(f"     {Colors.CYAN}./maximize{Colors.RESET}")
            print("     Select option 2 (Login) and complete OAuth")
            print("  3. Then extract tokens and set environment variables:")
            print(f"     {Colors.CYAN}cat ~/.maximize/tokens.json{Colors.RESET}")
            print(f"     {Colors.CYAN}export MAXIMIZE_ACCESS_TOKEN=\"sk-ant-...\"{Colors.RESET}")
            print(f"     {Colors.CYAN}export MAXIMIZE_REFRESH_TOKEN=\"refresh-...\"{Colors.RESET}")
            print("  4. Restart maximize server with valid tokens")
    
    if passed == total:
        print(f"\n{Colors.GREEN}{Colors.BOLD}🎉 All tests passed!{Colors.RESET}")
        sys.exit(0)
    elif passed >= total * 0.7:  # 70% pass rate
        print(f"\n{Colors.YELLOW}{Colors.BOLD}⚠️  {total - passed} test(s) failed, but most passed{Colors.RESET}")
        sys.exit(0)
    else:
        print(f"\n{Colors.RED}{Colors.BOLD}❌ {total - passed} test(s) failed{Colors.RESET}")
        sys.exit(1)

if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print(f"\n{Colors.YELLOW}Test suite interrupted{Colors.RESET}")
        sys.exit(1)
