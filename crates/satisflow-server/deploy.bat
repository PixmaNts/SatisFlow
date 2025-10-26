@echo off
REM Satisflow Server Deployment Script for Windows
REM This script helps deploy the Satisflow server in production

setlocal enabledelayedexpansion

REM Configuration
set IMAGE_NAME=satisflow-server
set CONTAINER_NAME=satisflow-server-prod
if "%PORT%"=="" set PORT=3000
if "%HOST%"=="" set HOST=0.0.0.0
if "%ENVIRONMENT%"=="" set ENVIRONMENT=production
if "%RUST_LOG%"=="" set RUST_LOG=info
if "%CORS_ORIGINS%"=="" set CORS_ORIGINS=https://yourdomain.com

REM Parse command line arguments
set COMMAND=%1
if "%COMMAND%"=="" set COMMAND=help

if "%COMMAND%"=="build" (
    echo [INFO] Building Docker image...
    docker build -t %IMAGE_NAME%:latest -f crates/satisflow-server/Dockerfile .
    if !errorlevel! equ 0 (
        echo [INFO] Build completed successfully!
    ) else (
        echo [ERROR] Build failed!
        exit /b 1
    )
    goto :end
)

if "%COMMAND%"=="run" (
    echo [INFO] Starting container...
    
    REM Check if container exists and stop it
    docker ps -a --format "table {{.Names}}" | findstr /C:"%CONTAINER_NAME%" >nul
    if !errorlevel! equ 0 (
        echo [WARN] Stopping existing container...
        docker stop %CONTAINER_NAME% >nul 2>&1
        docker rm %CONTAINER_NAME% >nul 2>&1
    )
    
    REM Run new container
    docker run -d ^
        --name %CONTAINER_NAME% ^
        -p %PORT%:3000 ^
        -e PORT=3000 ^
        -e HOST=%HOST% ^
        -e ENVIRONMENT=%ENVIRONMENT% ^
        -e RUST_LOG=%RUST_LOG% ^
        -e CORS_ORIGINS=%CORS_ORIGINS% ^
        --restart unless-stopped ^
        %IMAGE_NAME%:latest
    
    if !errorlevel! equ 0 (
        echo [INFO] Container started successfully!
        echo [INFO] Server is running on http://%HOST%:%PORT%
    ) else (
        echo [ERROR] Failed to start container!
        exit /b 1
    )
    goto :end
)

if "%COMMAND%"=="stop" (
    echo [INFO] Stopping container...
    docker ps --format "table {{.Names}}" | findstr /C:"%CONTAINER_NAME%" >nul
    if !errorlevel! equ 0 (
        docker stop %CONTAINER_NAME%
        if !errorlevel! equ 0 (
            echo [INFO] Container stopped!
        ) else (
            echo [ERROR] Failed to stop container!
            exit /b 1
        )
    ) else (
        echo [WARN] Container is not running.
    )
    goto :end
)

if "%COMMAND%"=="restart" (
    echo [INFO] Restarting container...
    call %0 stop
    timeout /t 2 /nobreak >nul
    call %0 run
    goto :end
)

if "%COMMAND%"=="logs" (
    echo [INFO] Showing container logs...
    docker logs -f %CONTAINER_NAME%
    goto :end
)

if "%COMMAND%"=="status" (
    echo [INFO] Container status:
    docker ps -a --filter name=%CONTAINER_NAME% --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"
    goto :end
)

if "%COMMAND%"=="health" (
    echo [INFO] Checking health...
    curl -f http://localhost:%PORT%/health >nul 2>&1
    if !errorlevel! equ 0 (
        echo [INFO] Service is healthy!
    ) else (
        echo [ERROR] Service is unhealthy or not responding!
        exit /b 1
    )
    goto :end
)

if "%COMMAND%"=="clean" (
    echo [WARN] Cleaning up...
    docker stop %CONTAINER_NAME% >nul 2>&1
    docker rm %CONTAINER_NAME% >nul 2>&1
    docker rmi %IMAGE_NAME%:latest >nul 2>&1
    echo [INFO] Cleanup completed!
    goto :end
)

if "%COMMAND%"=="help" (
    echo Satisflow Server Deployment Script
    echo.
    echo Usage: %0 [COMMAND]
    echo.
    echo Commands:
    echo   build    Build the Docker image
    echo   run      Run the container in production mode
    echo   stop     Stop the running container
    echo   restart  Restart the container
    echo   logs     Show container logs
    echo   status   Show container status
    echo   health   Check service health
    echo   clean    Remove container and image
    echo   help     Show this help message
    echo.
    echo Environment Variables:
    echo   PORT           Server port (default: 3000)
    echo   HOST           Server host (default: 0.0.0.0)
    echo   ENVIRONMENT    Environment mode (default: production)
    echo   RUST_LOG       Log level (default: info)
    echo   CORS_ORIGINS   Allowed CORS origins (default: https://yourdomain.com)
    echo.
    echo Examples:
    echo   %0 build                    # Build the image
    echo   %0 run                      # Run with defaults
    echo   set PORT=8080 ^&^& %0 run    # Run on port 8080
    echo   set RUST_LOG=debug ^&^& %0 run # Run with debug logging
    goto :end
)

echo [ERROR] Unknown command: %COMMAND%
echo Use '%0 help' for available commands.
exit /b 1

:end