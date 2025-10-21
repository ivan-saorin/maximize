"""Quick check if Maximize proxy is running"""
import requests
import sys

try:
    response = requests.get("http://localhost:8081/healthz", timeout=2)
    if response.status_code == 200:
        print("✅ Proxy is running!")
        print(f"Response: {response.json()}")
        sys.exit(0)
    else:
        print(f"⚠️ Proxy responded but with status {response.status_code}")
        sys.exit(1)
except requests.exceptions.ConnectionError:
    print("❌ Proxy is NOT running!")
    print("\nTo start it:")
    print("  cd C:\\projects\\maximize")
    print("  .\\target\\release\\maximize.exe")
    print("  (then choose option 1 - Start Proxy)")
    sys.exit(1)
except Exception as e:
    print(f"❌ Error checking proxy: {e}")
    sys.exit(1)
