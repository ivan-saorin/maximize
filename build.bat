@echo off
REM Build script for Maximize on Windows

echo ========================================
echo Building Maximize
echo ========================================

REM Check if cargo is installed
where cargo >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo Error: Cargo is not installed or not in PATH
    echo Please install Rust from https://rustup.rs/
    exit /b 1
)

REM Build release binary
echo.
echo Building release binary...
cargo build --release

if %ERRORLEVEL% neq 0 (
    echo.
    echo Build failed!
    exit /b 1
)

echo.
echo ========================================
echo Build successful!
echo ========================================
echo.
echo Binary location: target\release\maximize.exe
echo.
echo To run:
echo   target\release\maximize.exe
echo.
echo Or with options:
echo   target\release\maximize.exe --debug
echo   target\release\maximize.exe --bind 127.0.0.1
echo.
