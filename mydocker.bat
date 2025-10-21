@echo off
REM Docker management script for Maximize on Windows

if "%1"=="" goto help
if "%1"=="build" goto build
if "%1"=="up" goto up
if "%1"=="down" goto down
if "%1"=="logs" goto logs
if "%1"=="attach" goto attach
if "%1"=="clean" goto clean
if "%1"=="shell" goto shell
if "%1"=="help" goto help
goto unknown

:build
echo ========================================
echo Building Docker image...
echo ========================================
docker-compose build
if %ERRORLEVEL% neq 0 (
    echo Build failed!
    exit /b 1
)
echo Build successful!
goto end

:up
echo ========================================
echo Starting Docker container...
echo ========================================
docker-compose up -d
if %ERRORLEVEL% neq 0 (
    echo Failed to start container!
    exit /b 1
)
echo.
echo Container started successfully!
echo To authenticate, run: docker.bat attach
echo To view logs, run: docker.bat logs
goto end

:down
echo ========================================
echo Stopping Docker container...
echo ========================================
docker-compose down
echo Container stopped.
goto end

:logs
echo ========================================
echo Viewing Docker logs (Ctrl+C to exit)...
echo ========================================
docker-compose logs -f
goto end

:attach
echo ========================================
echo Attaching to container for authentication...
echo After login, press Ctrl+P then Ctrl+Q to detach
echo ========================================
timeout /t 2 >nul
docker attach maximize
goto end

:clean
echo ========================================
echo Removing Docker containers and images...
echo ========================================
docker-compose down -v
docker rmi maximize:latest 2>nul
echo Docker cleanup complete.
goto end

:shell
echo ========================================
echo Opening shell in container...
echo ========================================
docker exec -it maximize sh
goto end

:help
echo ========================================
echo Maximize - Docker Management
echo ========================================
echo.
echo Usage: docker.bat [command]
echo.
echo Available commands:
echo   build   - Build Docker image
echo   up      - Start Docker container
echo   down    - Stop Docker container
echo   logs    - View Docker logs
echo   attach  - Attach to container for authentication
echo   clean   - Remove Docker containers and images
echo   shell   - Open shell in container
echo   help    - Show this help message
echo.
echo Quick Start:
echo   1. docker.bat build
echo   2. docker.bat up
echo   3. docker.bat attach
echo   4. Complete OAuth (option 2)
echo   5. Press Ctrl+P then Ctrl+Q to detach
echo.
goto end

:unknown
echo Unknown command: %1
echo Run 'docker.bat help' for usage information.
exit /b 1

:end
