# BP:SR Logs

[![GitHub](https://img.shields.io/github/downloads/winjwinj/bpsr-logs/total?style=for-the-badge&color=%23ff9800)](https://github.com/winjwinj/bpsr-logs/releases/latest) [![Discord](https://img.shields.io/discord/1417447600608510015?color=%235865F2&label=Discord&style=for-the-badge)](https://discord.gg/Tcc54ST5BU)

[![GitHub](https://img.shields.io/github/v/release/winjwinj/bpsr-logs?style=flat-square)](https://github.com/winjwinj/bpsr-logs/releases)
[![GitHub](https://img.shields.io/github/license/winjwinj/bpsr-logs?style=flat-square)](https://github.com/winjwinj/bpsr-logs/blob/master/LICENSE)

BPSR Logs is a "blazingly fast" open source Blue Protocol: Star Resonance DPS meter, written in Rust by [winj](https://github.com/winjwinj). It is heavily inspired by [loa-logs](https://github.com/snoww/loa-logs), and uses reverse engineering work done by [StarResonanceDamageCounter](https://github.com/dmlgzs/StarResonanceDamageCounter) and [@Yuerino](https://github.com/Yuerino).

# Features

## 🎯 Core Features
- Real-time DPS and healing tracking
- In-game overlay with live combat statistics
- Skill breakdown and analysis
- Boss-only damage filtering
- Streamer mode for privacy

## 📊 Statistics & Analytics (NEW!)
- **Statistics Dashboard**: View real-time combat statistics with live updates
  - Export data as JSON for detailed analysis
  - Share statistics to clipboard for easy sharing with others
  - DPS and healing rankings with contribution percentages
- **DPS Charts**: Visual analytics with live updating charts
  - Real-time DPS line graphs showing damage over time
  - Player comparison bars with gradient visualizations
  - Peak DPS, average DPS, and total damage summaries

## 🔄 Auto-Updates
- Automatic updates on launch
- Windows auto-release workflow for seamless releases

# Download

https://github.com/winjwinj/bpsr-logs/releases/latest

\*currently only Windows 7 and up is supported

# Web Browser Access

You can access the BPSR Logs interface in a web browser using tunnel services like cloudflared. This allows you to view and control your DPS stats remotely from another device with **full functionality**!

**📖 [See detailed Web Access Guide](WEB_ACCESS.md)** for step-by-step instructions.

## What's New ✨

**Full HTTP API Support** - The app now includes a complete HTTP API server that enables all features to work in web browsers:
- ✅ Real-time DPS and healing stats
- ✅ Player and skill breakdowns
- ✅ Encounter reset and pause controls
- ✅ All interactive features working in browser
- ✅ Automatic environment detection (seamless experience)

## Quick Start

### Using the Helper Script (Easiest)

We provide convenient scripts to set up the tunnel for you:

**Windows:**
```cmd
start-tunnel.bat
```

**Linux/Mac:**
```bash
./start-tunnel.sh
```

These scripts will:
- Check if cloudflared is installed
- Verify BPSR Logs is running
- Start the tunnel automatically
- Display the access URL

### Manual Setup

1. **Start the desktop app** - Run BPSR Logs as normal (required for packet capture)
2. **Install cloudflared**: https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/downloads/
3. **Run tunnel**:
   ```bash
   cloudflared tunnel --url http://localhost:1420
   ```
4. **Access via browser** - Use the provided URL (e.g., `https://xxx.trycloudflare.com`)

## How It Works

The app runs two servers simultaneously:
- **Port 1420**: Web interface (Vite dev server in development, static file server in production)
- **Port 3000**: HTTP API server for data access

When accessed from a browser, the app automatically detects the environment and uses the HTTP API instead of Tauri's native invoke. You get the same experience whether using the desktop app or web browser!

## Notes

⚠️ **Important**: The desktop app must remain running for the web interface to work. The desktop app handles:
- Game packet capture (WinDivert)
- Data processing and statistics calculation
- Serving the HTTP API and web interface

Some desktop-specific features (like window blur effects) are gracefully disabled in browser mode.

## Use Cases

✅ **Works great for:**
- Viewing your DPS stats on a second monitor or device
- Sharing your stats with party members in real-time
- Monitoring combat performance from a mobile device
- Displaying stats on stream using an OBS browser source

⚠️ **Limited support for:**
- Changing settings from the web interface
- Some advanced features that require desktop app integration

# Is it bannable?

![validation.png](readme/validation.png)
![validation_remove_name.png](readme/validation_remove_name.png)
\*name blocked by their request. You can ping me on the Discord and I can give you the full screenshot.

# Does it mine Bitcoin?

No, it doesn’t mine Bitcoin. If it did, I'd be answering this from my private island.

## Serious Answer:

1. Code is open source, you can read it yourself
1. https://www.reddit.com/r/BlueProtocolPC/comments/1o1hhj9/comment/njcducb/
1. https://www.reddit.com/r/BlueProtocolPC/comments/1o1hhj9/comment/njn0xr3/
1. https://www.reddit.com/r/BlueProtocolPC/comments/1o1hhj9/comment/njf92bs/
1. https://tauri.by.simon.hyll.nu/concepts/security/false_positives/
   ![false_positive.png](readme/false_positive.png)

## How to fix:

1. Temporarily Disable Windows Defender
   1. [windowsdefender://threatsettings/](windowsdefender://threatsettings/)
   1. [https://support.microsoft.com/en-us/windows/virus-and-threat-protection-in-the-windows-security-app-1362f4cd-d71a-b52a-0b66-c2820032b65e#ID0EFH](https://support.microsoft.com/en-us/windows/virus-and-threat-protection-in-the-windows-security-app-1362f4cd-d71a-b52a-0b66-c2820032b65e#ID0EFH)
1. Don't use Brave

# Roadmap

https://discord.com/channels/1417447600608510015/1417450802561290280

# Contributing to the Project

1. Framework: [Tauri 2.0](https://v2.tauri.app/start/)
   1. Frontend: [Svelte 5 / SvelteKit v2](https://svelte.dev/docs/svelte/getting-started)
   1. Backend: [Rust](https://www.rust-lang.org/learn)
1. Join the [Discord](https://discord.gg/Tcc54ST5BU)
   1. Get the Developers role and talk in the [chat](https://discord.com/channels/1417447600608510015/1417458452661407754)
   1. Ping me on Discord to make sure I'm not already working on the feature

## Prerequisites

1. Some version of [Node.js](https://nodejs.org/en/download/)
1. tauri & Rust (see [tauri getting started](https://v2.tauri.app/start/prerequisites/))
1. Clone the repository
1. Install dependencies
   ```bash
   npm install
   ```
1. After everything has been installed, you should be able to build the dev version of the meter.
   ```bash
   npm run tauri dev
   ```
1. For IDE, I use
   1. Rust: [Jetbrains RustRover](https://www.jetbrains.com/rust/download/?section=windows)
   1. Svelte: [Visual Studio Code](https://code.visualstudio.com/) w/ [Svelte plugin](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) and lint with [ESLint](https://marketplace.visualstudio.com/items?itemName=dbaeumer.vscode-eslint) (there's a `.code-workspace` committed to the repo)

# FAQ

## Table of Contents

- [Missing `WinDivert64.sys`](#q-missing-windivert64sys)
- [Meter isn't detecting anything...](#q-meter-isnt-detecting-anything)
- [ExitLag? Other ping reducers?](#q-how-do-i-also-use-exitlag-and-other-ping-reducers)
- [Meter window is missing / meter window is tiny](#q-meter-window-is-missing--meter-window-is-tiny)
- [The installer crashes or takes forever to install](#q-the-installer-crashes-or-takes-forever-to-install)
- [EdgeWebview2 Error.](#q-the-meter-crashes-immediately-when-trying-to-open-it-edgewebview2-error)
- [Meter window lagging](#q-the-meter-window-lags-a-lot-when-dragging-around)
- [My information is missing!](#q-why-is-some-of-my-information-missing-when-others-have-it)
- [There are too many/too few columns in the meter.](#q-there-are-too-manytoo-few-columns-in-the-meter)
- [Help, my issue isn't listed here](#q-help-my-issue-isnt-listed-here-or-youve-tried-all-these-solutions-and-it-still-doesnt-work)

### Q: Missing `WinDivert64.sys`

A: You need to reinstall meter. The meter uses the WinDivert driver to listen to game packets. You either removed the file or your antivirus removed it. Please create an exception for the entire meter folder, and then reinstall the meter. After reinstalling, you should restart your computer before launching meter.

### Q: Meter isn't detecting anything...

A: There can be multiple reasons. If you have NordVPN installed, meter will not work due to both apps using WinDivert. You need to uninstall Nord, or completely quit the Nord processes and reboot.

### Q: How do I also use ExitLag (and other ping reducers)?

A: ExitLag recently updated their settings which changed how they redirect packets. Change your ExitLag settings to _Packet redirection method > Legacy - NDIS_.

### Q: Meter window is missing / meter window is tiny

A: TBD: Right-click the taskbar icon (located in the bottom right of your screen, next to the system time), click reset position, or load saved position. Adjust the size of the window and location, and then save the position.

### Q: The installer crashes or takes forever to install

A: Are you trying to install on a custom install folder with different permissions? You might need to run the installer in administrator mode due to permission issues.

### Q: The meter crashes immediately when trying to open it. EdgeWebview2 Error.

A: The meter needs Microsoft Edge Webview2 Runtime to run. Yours is probably missing or out of date. Go uninstall it first (it won't let you install it if you have an older version installed), then download and install from [here](https://go.microsoft.com/fwlink/p/?LinkId=2124703) (https://go.microsoft.com/fwlink/p/?LinkId=2124703).

### Q: The meter window lags a lot when dragging around

A: Are you on Windows 11? Disable blur in the settings (settings > accessibility). If you wish to have a dark background with blur disabled, also disable the transparency setting to have a pseudo dark mode.

### Q: Why is some of my information missing when others have it?

A: You opened the meter too late, and it wasn't able to get your character information. It is doing its best by guessing. You can fix this by: switching lines, moving to a different area, relogging, etc.

### Q: There are too many/too few columns in the meter.

A: TBD: You can change whatever column you want to show in the settings. TIP: you can `SHIFT+SCROLL` to scroll horizontally.

### Q: Help, my issue isn't listed here. Or you've tried all these solutions, and it still doesn't work.

A: Search the message history in the [#i-need-help](https://discord.com/channels/1417447600608510015/1427022345482014900) channel on [Discord](https://discord.gg/Tcc54ST5BU). If you can't find a solution there, please describe your issue.

### Q: Is it really "blazingly fast"?

A: [Yes.](https://i.imgur.com/QsLAntt.png)

# Screenshots

## Statistics Dashboard

View and share your combat statistics with real-time updates, export functionality, and shareable clipboard text.

Features:
- Live updating DPS, damage, duration, and player count
- DPS rankings with contribution percentages
- Healing rankings
- Export data as JSON
- Share to clipboard for easy sharing with friends

## DPS Charts

Real-time visual analytics with live updating charts showing damage over time and player comparisons.

Features:
- Live DPS line chart
- Player comparison bars with gradient visualizations
- Peak DPS, average DPS, and total damage summaries
- Reset encounter functionality

## In-game Overlay

![in_game_overlay.png](readme/in_game_overlay.png)

## Streamer Mode

![streamer_mode.png](readme/streamer_mode.png)

## Damage Breakdown with DPS Charts

TBD

## Skill Breakdown

![skill_breakdown.png](readme/skill_breakdown.png)

## Buff Uptime Tracking

TBD

## Opener Rotation

TBD

## Skill Cast Breakdown

TBD

## Search Filters

TBD
