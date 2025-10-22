@echo off
echo.
echo 🔍 Maximize Token Diagnostic Tool
echo ======================================
echo.

echo Checking environment variables:
echo --------------------------------
if defined MAXIMIZE_ACCESS_TOKEN (
    echo ✅ MAXIMIZE_ACCESS_TOKEN is SET
    echo    Value starts with: %MAXIMIZE_ACCESS_TOKEN:~0,20%...
) else (
    echo ❌ MAXIMIZE_ACCESS_TOKEN is NOT SET
)

if defined MAXIMIZE_REFRESH_TOKEN (
    echo ✅ MAXIMIZE_REFRESH_TOKEN is SET  
    echo    Value starts with: %MAXIMIZE_REFRESH_TOKEN:~0,20%...
) else (
    echo ❌ MAXIMIZE_REFRESH_TOKEN is NOT SET
)

if defined MAXIMIZE_API_KEY (
    echo ✅ MAXIMIZE_API_KEY is SET
    echo    Value starts with: %MAXIMIZE_API_KEY:~0,20%...
) else (
    echo ❌ MAXIMIZE_API_KEY is NOT SET
)

echo.
echo Checking token file:
echo --------------------------------
set TOKENS_FILE=%USERPROFILE%\.maximize\tokens.json

if exist "%TOKENS_FILE%" (
    echo ✅ Token file EXISTS at: %TOKENS_FILE%
    echo.
    echo File contents:
    type "%TOKENS_FILE%"
) else (
    echo ❌ Token file NOT FOUND at: %TOKENS_FILE%
)

echo.
echo.
echo 💡 Quick Fix:
echo --------------------------------
echo If environment variables are NOT SET, run:
echo.
echo   $env:MAXIMIZE_ACCESS_TOKEN="sk-ant-your-token-here"
echo   $env:MAXIMIZE_REFRESH_TOKEN="refresh-your-token-here"
echo   $env:MAXIMIZE_API_KEY="your-api-key"
echo.
echo Then restart the server in the SAME PowerShell window
echo.

pause
