# Maximize Token Diagnostic Tool
Write-Host "`n🔍 Maximize Token Diagnostic Tool" -ForegroundColor Cyan
Write-Host "======================================" -ForegroundColor Cyan
Write-Host ""

Write-Host "Checking environment variables:" -ForegroundColor Yellow
Write-Host "--------------------------------"

# Check MAXIMIZE_ACCESS_TOKEN
if ($env:MAXIMIZE_ACCESS_TOKEN) {
    Write-Host "✅ MAXIMIZE_ACCESS_TOKEN is SET" -ForegroundColor Green
    $preview = $env:MAXIMIZE_ACCESS_TOKEN.Substring(0, [Math]::Min(20, $env:MAXIMIZE_ACCESS_TOKEN.Length))
    Write-Host "   Value starts with: $preview..." -ForegroundColor Gray
} else {
    Write-Host "❌ MAXIMIZE_ACCESS_TOKEN is NOT SET" -ForegroundColor Red
}

# Check MAXIMIZE_REFRESH_TOKEN
if ($env:MAXIMIZE_REFRESH_TOKEN) {
    Write-Host "✅ MAXIMIZE_REFRESH_TOKEN is SET" -ForegroundColor Green
    $preview = $env:MAXIMIZE_REFRESH_TOKEN.Substring(0, [Math]::Min(20, $env:MAXIMIZE_REFRESH_TOKEN.Length))
    Write-Host "   Value starts with: $preview..." -ForegroundColor Gray
} else {
    Write-Host "❌ MAXIMIZE_REFRESH_TOKEN is NOT SET" -ForegroundColor Red
}

# Check MAXIMIZE_API_KEY
if ($env:MAXIMIZE_API_KEY) {
    Write-Host "✅ MAXIMIZE_API_KEY is SET" -ForegroundColor Green
    $preview = $env:MAXIMIZE_API_KEY.Substring(0, [Math]::Min(20, $env:MAXIMIZE_API_KEY.Length))
    Write-Host "   Value starts with: $preview..." -ForegroundColor Gray
} else {
    Write-Host "❌ MAXIMIZE_API_KEY is NOT SET" -ForegroundColor Red
}

Write-Host ""
Write-Host "Checking token file:" -ForegroundColor Yellow
Write-Host "--------------------------------"

$tokensFile = Join-Path $env:USERPROFILE ".maximize\tokens.json"

if (Test-Path $tokensFile) {
    Write-Host "✅ Token file EXISTS at: $tokensFile" -ForegroundColor Green
    Write-Host ""
    Write-Host "File contents:" -ForegroundColor Gray
    Get-Content $tokensFile | Write-Host -ForegroundColor Gray
} else {
    Write-Host "❌ Token file NOT FOUND at: $tokensFile" -ForegroundColor Red
}

Write-Host ""
Write-Host ""
Write-Host "💡 Quick Fix:" -ForegroundColor Cyan
Write-Host "--------------------------------"

if (!$env:MAXIMIZE_ACCESS_TOKEN -or !$env:MAXIMIZE_REFRESH_TOKEN) {
    Write-Host "Environment variables are NOT SET. Run:" -ForegroundColor Yellow
    Write-Host ""
    
    if (Test-Path $tokensFile) {
        Write-Host "Extract from file with:" -ForegroundColor White
        Write-Host '  $tokens = Get-Content "$env:USERPROFILE\.maximize\tokens.json" | ConvertFrom-Json' -ForegroundColor Cyan
        Write-Host '  $env:MAXIMIZE_ACCESS_TOKEN = $tokens.access_token' -ForegroundColor Cyan
        Write-Host '  $env:MAXIMIZE_REFRESH_TOKEN = $tokens.refresh_token' -ForegroundColor Cyan
        Write-Host '  $env:MAXIMIZE_API_KEY = "your-api-key"' -ForegroundColor Cyan
    } else {
        Write-Host "  Run maximize CLI first to get tokens:" -ForegroundColor White
        Write-Host "    .\maximize.exe" -ForegroundColor Cyan
        Write-Host "    Select option 2 (Login)" -ForegroundColor Cyan
    }
    
    Write-Host ""
    Write-Host "Then restart the server in the SAME PowerShell window:" -ForegroundColor Yellow
    Write-Host "  cargo run --release -- --server-only" -ForegroundColor Cyan
} else {
    Write-Host "✅ Environment variables are set!" -ForegroundColor Green
    Write-Host "   If you're still having issues, check the server logs with --debug flag" -ForegroundColor Gray
}

Write-Host ""
