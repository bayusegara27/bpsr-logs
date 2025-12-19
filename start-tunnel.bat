@echo off
REM BPSR Logs - Quick Tunnel Setup Script for Windows
REM This script helps you quickly set up a cloudflared tunnel to access BPSR Logs from a web browser

echo ========================================
echo BPSR Logs - Web Access Setup
echo ========================================
echo.

REM Check if cloudflared is installed
where cloudflared >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] cloudflared is not installed or not in PATH
    echo.
    echo Please install cloudflared first:
    echo 1. Download from: https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/downloads/
    echo 2. Or install via winget: winget install --id Cloudflare.cloudflared
    echo 3. Or install via Scoop: scoop install cloudflared
    echo.
    pause
    exit /b 1
)

echo [OK] cloudflared is installed
echo.

REM Check if BPSR Logs is running (checking if port 1420 is in use)
netstat -an | findstr ":1420" | findstr "LISTENING" >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo [WARNING] BPSR Logs doesn't appear to be running
    echo          (Port 1420 is not listening)
    echo.
    echo Please make sure BPSR Logs desktop app is running first!
    echo.
    echo Do you want to continue anyway? (Y/N)
    set /p continue=
    if /i not "%continue%"=="Y" (
        echo.
        echo Exiting...
        pause
        exit /b 0
    )
)

echo Starting cloudflared tunnel...
echo.
echo Once the tunnel is ready, you'll see a URL like:
echo https://xxxxx.trycloudflare.com
echo.
echo Copy that URL and open it in any web browser!
echo.
echo Press Ctrl+C to stop the tunnel when you're done.
echo ========================================
echo.

cloudflared tunnel --url http://localhost:1420
