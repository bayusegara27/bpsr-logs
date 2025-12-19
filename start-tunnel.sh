#!/bin/bash
set -e  # Exit on error
# BPSR Logs - Quick Tunnel Setup Script for Linux/Mac
# This script helps you quickly set up a cloudflared tunnel to access BPSR Logs from a web browser

echo "========================================"
echo "BPSR Logs - Web Access Setup"
echo "========================================"
echo ""

# Check if cloudflared is installed
if ! command -v cloudflared &> /dev/null; then
    echo "[ERROR] cloudflared is not installed"
    echo ""
    echo "Please install cloudflared first:"
    echo "- Linux: https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/downloads/"
    echo "- Mac: brew install cloudflared"
    echo ""
    exit 1
fi

echo "[OK] cloudflared is installed"
echo ""

# Check if port 1420 is listening - try multiple methods for portability
port_listening=false

# Method 1: Try lsof (common on Mac/Linux) - use simple syntax for portability
if command -v lsof &> /dev/null; then
    if lsof -i :1420 2>/dev/null | grep -q "LISTEN"; then
        port_listening=true
    fi
fi

# Method 2: Try netstat (fallback)
if [ "$port_listening" = false ] && command -v netstat &> /dev/null; then
    if netstat -an 2>/dev/null | grep -q ":1420 .*LISTEN"; then
        port_listening=true
    fi
fi

# Method 3: Try ss (modern Linux)
if [ "$port_listening" = false ] && command -v ss &> /dev/null; then
    if ss -ln 2>/dev/null | grep -q ":1420 .*LISTEN"; then
        port_listening=true
    fi
fi

if [ "$port_listening" = false ]; then
    echo "[WARNING] BPSR Logs doesn't appear to be running"
    echo "          (Port 1420 is not listening)"
    echo ""
    echo "Please make sure BPSR Logs desktop app is running first!"
    echo ""
    read -p "Do you want to continue anyway? (y/N) " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Exiting..."
        exit 0
    fi
fi

echo "Starting cloudflared tunnel..."
echo ""
echo "Once the tunnel is ready, you'll see a URL like:"
echo "https://xxxxx.trycloudflare.com"
echo ""
echo "Copy that URL and open it in any web browser!"
echo ""
echo "Press Ctrl+C to stop the tunnel when you're done."
echo "========================================"
echo ""

cloudflared tunnel --url http://localhost:1420
