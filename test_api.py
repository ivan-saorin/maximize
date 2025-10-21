"""
Quick API test for Maximize proxy
Tests model nicknames and basic functionality
"""
from anthropic import Anthropic
import sys

# Connect to local proxy
client = Anthropic(
    api_key="dummy",  # Proxy doesn't check this
    base_url="http://localhost:8081"
)

def test_model(nickname: str, prompt: str):
    """Test a single model with a simple prompt"""
    print(f"\n{'='*60}")
    print(f"Testing model: {nickname}")
    print(f"Prompt: {prompt}")
    print(f"{'='*60}")
    
    try:
        message = client.messages.create(
            model=nickname,
            max_tokens=100,
            messages=[{"role": "user", "content": prompt}]
        )
        
        response_text = message.content[0].text
        print(f"✅ SUCCESS")
        print(f"Response: {response_text}")
        print(f"Usage: {message.usage}")
        return True
        
    except Exception as e:
        print(f"❌ FAILED: {e}")
        return False

def main():
    print("🚀 Maximize API Test Suite")
    print("=" * 60)
    
    # Test cases: (nickname, prompt)
    tests = [
        ("l", "Say 'Hello from Claude Sonnet 4' in exactly 5 words"),
        ("m", "What's 15 * 7? Answer with just the number."),
        ("s", "Name one color. One word only."),
    ]
    
    results = []
    for nickname, prompt in tests:
        success = test_model(nickname, prompt)
        results.append((nickname, success))
    
    # Summary
    print(f"\n{'='*60}")
    print("📊 Test Summary")
    print(f"{'='*60}")
    
    passed = sum(1 for _, success in results if success)
    total = len(results)
    
    for nickname, success in results:
        status = "✅ PASS" if success else "❌ FAIL"
        print(f"{status} - Model: {nickname}")
    
    print(f"\nResults: {passed}/{total} tests passed")
    
    if passed == total:
        print("\n🎉 All tests passed!")
        sys.exit(0)
    else:
        print(f"\n⚠️ {total - passed} test(s) failed")
        sys.exit(1)

if __name__ == "__main__":
    main()
