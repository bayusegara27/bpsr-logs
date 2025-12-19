# Web Access Troubleshooting Guide

This guide helps you resolve issues when trying to access BPSR Logs through a web browser.

## Common Issues

### Issue: Cannot access http://localhost:3000/api

**Symptoms:**
- Browser shows "Connection refused" or "Unable to connect"
- API endpoints not responding
- Logs show "App crashed" or HTTP server failed to start

**Solutions:**

1. **Check if the HTTP API server started successfully**
   - Look for this log message: `✅ HTTP API server successfully bound to http://0.0.0.0:3000`
   - If you see: `❌ HTTP API server failed to start`, continue to next steps

2. **Check if ports 3000-3010 are already in use**
   - Close any other applications that might be using these ports
   - Common culprits: other web servers, development tools, or previous instances of the app
   - On Windows, use Task Manager to check for processes using these ports
   
3. **Check your firewall settings**
   - Windows Firewall might be blocking the ports
   - Allow BPSR Logs through Windows Firewall
   - Go to: Windows Security → Firewall & network protection → Allow an app through firewall

4. **Try accessing with different URLs**
   - `http://localhost:3000/api/header-info`
   - `http://127.0.0.1:3000/api/header-info`
   
   Note: If using a tunnel service, use the tunnel URL instead (e.g., `https://xxx.trycloudflare.com/api/header-info`)

5. **Check the application logs**
   - The logs will show detailed information about what went wrong
   - Look for messages starting with `❌` or `⚠️`
   - Common error messages and their solutions:
     - "Port XXX is already in use" → Close the application using that port
     - "Permission denied" → Run the application as administrator
     - "Could not bind to any port" → Check firewall settings

### Issue: Cannot access http://localhost:1420

**Symptoms:**
- Browser shows "Connection refused" when accessing http://localhost:1420
- Static file server not starting
- Logs show "Static file server not started"

**Solutions:**

1. **In Development Mode:**
   - This is normal! The message `ℹ️ Static file server not started - frontend build directory not found` is expected
   - In development, you should use the Vite dev server instead
   - The Vite dev server typically runs on port 5173 or 1420 (check your terminal output)
   - Run `npm run dev` to start the Vite dev server

2. **In Production Mode:**
   - Make sure the application was built correctly with the frontend included
   - The 'build' directory should be bundled with the application
   - Check logs for: `✅ Frontend build directory found at: ...`
   - If missing, rebuild the application: `npm run tauri build`

3. **Port Conflict:**
   - If ports 1420-1430 are in use, the static server won't start
   - Close any applications using these ports
   - Common conflicts: other Vite dev servers, web servers

### Issue: Both servers failing to start

**Symptoms:**
- Both HTTP API server (port 3000) and static file server (port 1420) fail
- Application crashes immediately on startup

**Solutions:**

1. **Restart the application**
   - Sometimes a simple restart fixes initialization issues
   - Close the application completely (check Task Manager)
   - Wait a few seconds
   - Start the application again

2. **Check system requirements**
   - Windows 7 or higher
   - Administrator privileges (for WinDivert packet capture)
   - Required dependencies: Edge WebView2 Runtime

3. **Update dependencies**
   - Update Edge WebView2 Runtime: https://go.microsoft.com/fwlink/p/?LinkId=2124703
   - Make sure Windows is up to date

4. **Reinstall the application**
   - Uninstall completely
   - Delete application data folder
   - Download latest version from releases
   - Install fresh

## Verification Steps

To verify that web access is working correctly:

1. **Start the application**
   - The desktop application must be running

2. **Check the logs**
   - Look for these success messages:
   ```
   ✅ HTTP API server successfully bound to http://0.0.0.0:3000
   🌐 Web browser can access the API at: http://localhost:3000/api
   ```

3. **Test the API**
   - Open your browser
   - Navigate to: `http://localhost:3000/api/header-info`
   - You should see JSON data or an error message (both are valid if no encounter is active)
   
4. **Test the web interface (if in production)**
   - Navigate to: `http://localhost:1420`
   - You should see the BPSR Logs interface

## Understanding the Logs

**Good startup logs should look like this:**
```
[INFO] 🌐 Initializing web servers for browser access...
[INFO] 📡 Starting HTTP API server...
[INFO] 📂 Starting static file server...
[INFO] 🔄 HTTP API server task started
[INFO] 🔧 Building HTTP API server state...
[INFO] 🔧 Configuring CORS...
[INFO] 🔧 Setting up API routes...
[INFO] 🔧 Creating main router with CORS layer...
[INFO] 🚀 Attempting to start HTTP API server...
[INFO] 📡 Attempting to bind HTTP API server to 0.0.0.0:3000...
[INFO] ✅ HTTP API server successfully bound to http://0.0.0.0:3000
[INFO] 🌐 Web browser can access the API at:
[INFO]    http://localhost:3000/api
[INFO] 🎯 HTTP API server is ready - starting to serve requests...
```

**Bad startup logs might show:**
```
[INFO] 📡 Starting HTTP API server...
[INFO] App crashed! Info: PanicHookInfo { ... }
[WARN] ❌ HTTP API server failed to start: <error message>
```

## Still Having Issues?

If you've tried all the above solutions and still can't access the web interface:

1. **Collect information:**
   - Full application logs from startup
   - Operating system version
   - Firewall/antivirus software you're running
   - Other applications that might be using the ports

2. **GitHub Issues:**
   - Check if someone else has reported the same issue
   - Create a new issue with detailed information
   - Include logs and steps to reproduce

## Advanced Debugging

For developers or advanced users:

### Check which process is using a port (Windows)

```cmd
netstat -ano | findstr :3000
netstat -ano | findstr :1420
```

The last column shows the Process ID (PID). Use Task Manager to identify which application has that PID.

### Kill a process using a port (Windows)

```cmd
taskkill /PID <process_id> /F
```

Replace `<process_id>` with the PID from the previous command.

### Enable verbose logging

Look for a way to enable debug-level logging in the application settings (if available).

### Test API directly with curl

```cmd
curl http://localhost:3000/api/header-info
```

This helps determine if the issue is with the browser or the server.

## Safety Notes

- Always make sure you're running the official build from the GitHub releases
- Don't disable your antivirus/firewall completely - just add exceptions for BPSR Logs
- Keep your system and the application updated

## Related Documentation

- [Web Access Setup Guide](WEB_ACCESS.md) - How to set up web access with tunnel services
- [Main README](README.md) - General information about BPSR Logs
- [FAQ](README.md#faq) - Frequently asked questions
