@echo off
setlocal enabledelayedexpansion

REM Token Extractor for Maximize (Windows)
REM Extracts tokens from %USERPROFILE%\.maximize\tokens.json

echo.
echo 🔑 Maximize Token Extractor
echo ======================================
echo.

set TOKENS_FILE=%USERPROFILE%\.maximize\tokens.json

REM Check if tokens file exists
if not exist "%TOKENS_FILE%" (
    echo ❌ No tokens found at %TOKENS_FILE%
    echo.
    echo Run maximize CLI first to authenticate:
    echo    maximize.exe
    echo    Select option 2 (Login^)
    echo.
    pause
    exit /b 1
)

echo ✅ Found tokens file
echo.

REM Read the tokens file
for /f "delims=" %%i in ('type "%TOKENS_FILE%"') do set JSON_CONTENT=%%i

REM Try to extract tokens (basic parsing, might need adjustment)
echo 📋 Tokens found in: %TOKENS_FILE%
echo.
echo Copy the values from the file and set:
echo.
type "%TOKENS_FILE%"
echo.
echo.
echo ====================================
echo Manually extract the values and run:
echo ====================================
echo.
echo set MAXIMIZE_ACCESS_TOKEN=^"sk-ant-...^"
echo set MAXIMIZE_REFRESH_TOKEN=^"refresh-...^"
echo.
echo Or create a .env file with:
echo.
echo MAXIMIZE_ACCESS_TOKEN="sk-ant-..."
echo MAXIMIZE_REFRESH_TOKEN="refresh-..."
echo MAXIMIZE_API_KEY="your-api-key"
echo.
echo Then run: maximize.exe --server-only
echo.

pause
