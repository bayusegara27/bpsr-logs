# BPSR Logs - Web Access Setup

This document explains how to access BPSR Logs through a web browser using tunnel services.

## Prerequisites

1. BPSR Logs desktop application must be running
2. Install a tunnel service (cloudflared, ngrok, or localtunnel)

## Option 1: Cloudflared (Recommended)

Cloudflared is free and doesn't require an account.

### Installation

Download cloudflared from: https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/downloads/

Or use one of these methods:

**Windows (winget):**
```cmd
winget install --id Cloudflare.cloudflared
```

**Windows (Scoop):**
```cmd
scoop install cloudflared
```

**Linux (Debian/Ubuntu):**
```bash
wget https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-amd64.deb
sudo dpkg -i cloudflared-linux-amd64.deb
```

### Usage

1. Start BPSR Logs desktop app (this starts the server on port 1420)
2. Open a command prompt/terminal
3. Run:
```bash
cloudflared tunnel --url http://localhost:1420
```
4. You'll see output like:
```
Your quick Tunnel has been created! Visit it at (it may take some time to be reachable):
https://xxxxx-xxx-xxx.trycloudflare.com
```
5. Open that URL in any web browser!

## Option 2: ngrok

Ngrok is easy to use but requires a free account.

### Installation

Download from: https://ngrok.com/download

### Usage

1. Start BPSR Logs desktop app
2. Run:
```bash
ngrok http 1420
```
3. Copy the forwarding URL (e.g., `https://xxxx.ngrok.io`)
4. Open in browser

## Option 3: localtunnel

localtunnel is npm-based and free.

### Installation

```bash
npm install -g localtunnel
```

### Usage

1. Start BPSR Logs desktop app
2. Run:
```bash
lt --port 1420
```
3. Copy the URL provided
4. Open in browser

## Features Available in Web Browser

When accessing through a web browser, you'll have access to:

### ✅ Fully Supported
- Real-time DPS meter display
- Live statistics dashboard  
- DPS charts and analytics
- Player and skill breakdowns
- All overlay views (DPS, healing, etc.)
- Live data updates (via polling)

### ⚠️ Limited/Experimental
- Settings and configuration (some features may not work)
- Encounter reset and control functions (may require desktop app)

**Note**: The desktop app provides the backend functionality. All data processing, packet capture, and computation happens in the desktop app. The web interface is primarily for viewing/monitoring purposes.

### Future Improvements

In future versions, we plan to add full HTTP API support to enable all features in web browser mode, including:
- Full settings management from browser
- Encounter control (reset, pause, etc.)
- Advanced configuration options

## Sharing Your Stats

You can share your tunnel URL with others to let them view your DPS stats in real-time! Just keep in mind:

- 🔒 Only share with people you trust
- ⏱️ The tunnel URL is temporary (will change when you restart the tunnel)
- 💻 Your desktop app must stay running for others to see your stats
- 🌐 Free tunnel services may have rate limits

## Troubleshooting

### "Connection refused" or "Cannot connect"

- Make sure the BPSR Logs desktop app is running
- Verify the tunnel is active (check the terminal where you ran the tunnel command)
- Try accessing http://localhost:1420 directly on the same computer first

### "Page not loading" or "ERR_CONNECTION_TIMED_OUT"

- Wait a minute - free tunnel services can take time to establish connection
- Try refreshing the page
- Check if your firewall is blocking the tunnel application

### Stats not updating

- The desktop app must be running and capturing game packets
- Make sure you're in game and generating combat activity
- Check that the packet capture is working in the desktop app first

## For Developers

### Running with Custom Configuration

You can customize the server host/port by setting environment variables:

```bash
# Run on a different port
TAURI_DEV_HOST=0.0.0.0 npm run tauri dev
```

### Development Mode

When running `npm run tauri dev`, the Vite dev server automatically binds to `0.0.0.0:1420`, making it accessible for tunneling.

## Security Notes

⚠️ **Important Security Information:**

- Tunnel services expose your local application to the internet
- Anyone with the tunnel URL can access your meter
- Free tunnel URLs are not password protected by default
- Do not share tunnel URLs publicly or on untrusted platforms
- Some tunnel services (like ngrok) offer password protection in paid tiers

For maximum security:
- Only use tunnels when needed
- Stop the tunnel when not in use (Ctrl+C in the terminal)
- Use a paid tunnel service with password protection if sharing publicly
- Consider using a VPN or private network instead of public tunnels

## FAQ

**Q: Do I need to keep the terminal/command prompt window open?**  
A: Yes, the tunnel runs in that window. If you close it, the tunnel stops.

**Q: Will this work if I'm playing on a different computer?**  
A: Yes! That's the main benefit - you can view your stats on a phone, tablet, or another computer.

**Q: Does this affect game performance?**  
A: No, the tunnel only exposes the web interface. The packet capture and processing still happens locally.

**Q: Can I use this to stream my stats?**  
A: Yes! You can use an OBS browser source with the tunnel URL to show your stats on stream.

**Q: Is my game data secure?**  
A: The tunnel exposes the DPS meter interface, which shows combat statistics. It does not expose your game credentials or personal information.
