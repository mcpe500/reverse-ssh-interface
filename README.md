# 🔗 Reverse SSH Interface

<div align="center">

![Rust](https://img.shields.io/badge/Rust-1.70+-orange?style=flat-square&logo=rust)
![Tauri](https://img.shields.io/badge/Tauri-2.0-blue?style=flat-square&logo=tauri)
![License](https://img.shields.io/badge/License-MIT-blue?style=flat-square)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey?style=flat-square)

**A powerful, cross-platform tool for managing reverse SSH tunnels with ease.**

*Connect to any device behind firewalls, NAT, or restricted networks—without complex router configurations.*

[Features](#-features) • [Installation](#-installation) • [Quick Start](#-quick-start) • [CLI Reference](#-cli-reference) • [Web Interface](#-web-interface) • [Desktop GUI](#-desktop-gui) • [API Reference](#-api-reference) • [Configuration](#-configuration)

</div>

---

## 📖 The Story

### The Problem

Have you ever faced these frustrating scenarios?

- 🏠 **Home Server Access**: You're running a web server, game server, or home automation system on your home network, but your ISP uses CGNAT (Carrier-Grade NAT), giving you no public IP address
- 🔒 **Corporate Firewall**: Your development machine is behind a strict corporate firewall that blocks all incoming connections
- 🌐 **IoT Device Management**: You have IoT devices deployed across different locations, all behind different NATs, and need remote access for maintenance
- 💻 **Remote Development**: You want to SSH into your laptop from anywhere, but it's always on different networks (home, coffee shop, coworking space)
- 🏢 **Client Site Access**: You've deployed equipment at a client's site and need remote access, but they won't (or can't) configure their firewall

Traditional solutions require:
- Router access for port forwarding (often not available)
- Static public IP addresses (expensive or impossible with CGNAT)
- VPN setup (complex and requires infrastructure)
- Third-party services like ngrok (subscription costs, data privacy concerns)

### The Solution

**Reverse SSH tunnels** flip the problem on its head. Instead of trying to connect *into* your device through a firewall, your device initiates an *outbound* connection to a public server you control. Since outbound connections are almost always allowed, this works from virtually anywhere.

Once established, the tunnel acts as a secure pathway—allowing you to connect to your hidden device as if it were directly accessible on the public server.

**Reverse SSH Interface** makes this powerful technique accessible to everyone:

```
Your Device                        Your VPS/Server                    You
(behind NAT)                       (public IP)                        (anywhere)
     |                                   |                                |
     |──── Outbound SSH Connection ────►│                                |
     |◄════════ Reverse Tunnel ═════════│                                |
     |                                   │◄──── SSH to VPS port ──────────|
     |                                   |                                |
     └────────── Secure Access ──────────┴────────────────────────────────┘
```

### Why This Tool?

While you *can* set up reverse SSH tunnels manually with commands like:

```bash
ssh -R 8080:localhost:3000 user@server.com -N -o ServerAliveInterval=20
```

This approach has significant drawbacks:

| Manual SSH | Reverse SSH Interface |
|------------|----------------------|
| ❌ Connection drops silently | ✅ Automatic reconnection with intelligent backoff |
| ❌ Must remember complex arguments | ✅ Save profiles with easy names |
| ❌ No visibility into tunnel status | ✅ Real-time monitoring dashboard |
| ❌ Single tunnel per command | ✅ Multiple tunnels per connection |
| ❌ No GUI for non-terminal users | ✅ Web UI + Desktop GUI options |
| ❌ Logs scattered or lost | ✅ Centralized logging with history |
| ❌ Manual restart on boot | ✅ System tray with auto-start |

---

## ✨ Features

### 🎯 Core Capabilities

#### Multi-Tunnel Support
Create multiple reverse tunnels through a single SSH connection. Expose your web server on port 8080, your API on port 8443, and your database on port 5432—all through one secure session.

```toml
# Example: Multiple tunnels in one profile
[[tunnels]]
remote_port = 8080
local_port = 3000    # Web app

[[tunnels]]
remote_port = 8443
local_port = 443     # HTTPS API

[[tunnels]]
remote_port = 5432
local_port = 5432    # PostgreSQL
```

#### Intelligent Auto-Reconnection
Network connections fail—that's a fact of life. Wi-Fi drops, ISPs have outages, and laptops go to sleep. Reverse SSH Interface handles this gracefully with:

- **Exponential Backoff**: Starts with 1-second retry, gradually increases to avoid hammering the server
- **Configurable Maximum Delay**: Caps at 5 minutes by default
- **Unlimited Retries**: By default, never gives up (configurable)
- **Smart Detection**: Distinguishes between network issues and authentication failures

```
Connection lost → Wait 1s → Retry
Failed → Wait 2s → Retry  
Failed → Wait 4s → Retry
Failed → Wait 8s → Retry
...continues up to max_delay (300s)...
Reconnected! → Reset backoff to 1s
```

#### Profile Management
Save your connection configurations as named profiles. No more typing long commands or remembering port numbers.

- **TOML Format**: Human-readable configuration files
- **Independent Files**: Each profile is a separate `.toml` file—easy to backup, sync, or share
- **Full Customization**: Every SSH option is configurable
- **Validation**: Profiles are validated on load to catch errors early

#### Cross-Platform Native
Works identically on Windows, Linux, and macOS:

- **Windows**: Uses built-in OpenSSH or Git Bash SSH
- **Linux**: Uses system SSH from any distribution
- **macOS**: Uses built-in SSH or Homebrew OpenSSH
- **Automatic Detection**: Finds SSH binary automatically, or specify a custom path

#### Secure by Default
Security is not an afterthought:

- **SSH Agent Support**: Keys never leave the agent—the most secure option
- **Known Hosts Verification**: Three modes: strict, accept-new (default), or disabled
- **No Password Storage**: Password auth requires external `sshpass` and is not recommended
- **Key File Support**: Specify custom identity files per profile
- **Credential Redaction**: Sensitive data is automatically redacted in logs

### 🖥️ Three Interfaces, One Core

| Interface | Best For | Features |
|-----------|----------|----------|
| **CLI (`rssh`)** | Developers, scripters, automation | Fast, scriptable, shell completions |
| **Web UI** | Remote management, teams | Browser access, real-time WebSocket updates |
| **Desktop GUI** | End users, visual preference | Native app, system tray, notifications |

All three interfaces share the same core library, ensuring consistent behavior and feature parity.

---

## 🔧 Installation

### Prerequisites

#### 1. Rust Toolchain (1.70+)

**Windows (PowerShell):**
```powershell
winget install Rustlang.Rust.MSVC
# Or download from https://rustup.rs
```

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### 2. OpenSSH Client

**Windows 10/11:**
```powershell
# Check if installed
ssh -V

# If not installed, enable via Settings:
# Settings → Apps → Optional Features → Add a feature → OpenSSH Client
# Or via PowerShell (Admin):
Add-WindowsCapability -Online -Name OpenSSH.Client~~~~0.0.1.0
```

**Debian/Ubuntu:**
```bash
sudo apt update && sudo apt install openssh-client
```

**Fedora/RHEL:**
```bash
sudo dnf install openssh-clients
```

**macOS:**
```bash
# Pre-installed, verify with:
ssh -V
```

#### 3. A Public SSH Server

You need a server with:
- A public IP address (or domain name)
- SSH server running (port 22 or custom)
- GatewayPorts enabled for binding to `0.0.0.0` (optional but recommended)

**Configure GatewayPorts on your server** (`/etc/ssh/sshd_config`):
```
GatewayPorts yes
# Or for specific users:
# Match User tunneluser
#     GatewayPorts yes
```

### Building from Source

```bash
# Clone the repository
git clone https://github.com/mcpe500/reverse-ssh-interface.git
cd reverse-ssh-interface

# Build everything in release mode
cargo build --release --workspace

# Binaries are in target/release/
ls target/release/
# - rssh (or rssh.exe on Windows)
# - rssh-web (or rssh-web.exe)
# - reverse_ssh_gui (or reverse_ssh_gui.exe)
```

### Installing Individual Components

```bash
# Install CLI globally
cargo install --path cli

# Verify
rssh --help
```

### Building the Desktop GUI

The GUI requires Tauri CLI:

```bash
# Install Tauri CLI
cargo install tauri-cli

# Build the GUI
cd gui
cargo tauri build

# The installer will be in:
# - Windows: gui/target/release/bundle/msi/
# - macOS: gui/target/release/bundle/dmg/
# - Linux: gui/target/release/bundle/deb/ or /appimage/
```

---

## 🚀 Quick Start

### Scenario: Access Your Home Web Server from Anywhere

Let's say you're running a web application on port 3000 at home. You want to access it from work or while traveling.

**Step 1: Create a Profile**

```bash
rssh profile add home-web \
    --host your-vps.example.com \
    --user deploy \
    --tunnel 8080:localhost:3000
```

This creates a profile that will:
- Connect to `your-vps.example.com` as user `deploy`
- Create a reverse tunnel: port 8080 on the VPS → port 3000 on your home machine

**Step 2: Start the Tunnel**

```bash
rssh up home-web
```

Output:
```
✓ Session started: a1b2c3d4-e5f6-7890-abcd-ef1234567890
  Profile: home-web
  Host: your-vps.example.com:22
  Tunnels: 
    • 8080 → localhost:3000
  Status: Connected
```

**Step 3: Access Your Server**

From anywhere in the world:
```bash
# Access via your VPS
curl http://your-vps.example.com:8080

# Or SSH to VPS and access localhost
ssh your-vps.example.com
curl http://localhost:8080
```

**Step 4: Check Status**

```bash
rssh status
```

Output:
```
Active Sessions (1):
┌──────────────────────────────────────┬───────────┬───────────┬────────────────────┐
│ Session ID                           │ Profile   │ Status    │ Tunnels            │
├──────────────────────────────────────┼───────────┼───────────┼────────────────────┤
│ a1b2c3d4-e5f6-7890-abcd-ef12345678   │ home-web  │ Connected │ :8080→localhost:30 │
└──────────────────────────────────────┴───────────┴───────────┴────────────────────┘
```

**Step 5: Stop When Done**

```bash
rssh down a1b2c3d4-e5f6-7890-abcd-ef1234567890
```

---

## 💻 CLI Reference

The CLI (`rssh`) is the primary interface for power users and automation.

### Global Options

```bash
rssh [OPTIONS] <COMMAND>

Options:
    -h, --help       Print help information
    -V, --version    Print version information
    -v, --verbose    Increase logging verbosity
    -q, --quiet      Suppress non-essential output
```

### Commands Overview

| Command | Description |
|---------|-------------|
| `up <profile>` | Start a tunnel session |
| `down <session-id>` | Stop a running session |
| `status` | Show all active sessions |
| `logs` | View session logs |
| `profile` | Manage connection profiles |

---

### `rssh up` — Start a Tunnel

Starts a reverse SSH tunnel using a saved profile.

```bash
rssh up <PROFILE_NAME>
```

**Arguments:**
- `<PROFILE_NAME>`: Name of the profile to use (required)

**What Happens:**
1. Loads the profile configuration from disk
2. Detects the SSH binary on your system
3. Builds SSH command with proper arguments
4. Spawns the SSH process
5. Monitors the connection
6. Handles reconnection if enabled

**Example:**
```bash
rssh up production-server
```

**Output (Success):**
```
✓ Session started: f47ac10b-58cc-4372-a567-0e02b2c3d479
  Profile: production-server
  Host: prod.example.com:22
  User: deploy
  Tunnels:
    • localhost:8080 → localhost:3000
    • localhost:8443 → localhost:443
  Auto-reconnect: Enabled
```

**Output (Error):**
```
✗ Failed to start session
  Profile: production-server
  Error: Connection refused (host unreachable)
```

---

### `rssh down` — Stop a Tunnel

Stops a running tunnel session by its UUID.

```bash
rssh down <SESSION_ID>
```

**Arguments:**
- `<SESSION_ID>`: The UUID of the session to stop (from `rssh status`)

**Example:**
```bash
rssh down f47ac10b-58cc-4372-a567-0e02b2c3d479
```

**Output:**
```
✓ Session stopped: f47ac10b-58cc-4372-a567-0e02b2c3d479
```

**Tip:** You can use partial UUIDs if unambiguous:
```bash
rssh down f47ac  # Works if only one session starts with "f47ac"
```

---

### `rssh status` — View Session Status

Displays the status of all active tunnel sessions.

```bash
rssh status [OPTIONS]
```

**Options:**
| Option | Short | Description |
|--------|-------|-------------|
| `--format <FORMAT>` | `-f` | Output format: `human` (default) or `json` |
| `--session <ID>` | `-s` | Show only a specific session |

**Example (Human-Readable):**
```bash
rssh status
```

**Output:**
```
Active Sessions (2):
┌──────────────────────────────────────┬─────────────────┬─────────────┬───────────────────────┐
│ Session ID                           │ Profile         │ Status      │ Tunnels               │
├──────────────────────────────────────┼─────────────────┼─────────────┼───────────────────────┤
│ f47ac10b-58cc-4372-a567-0e02b2c3d479 │ home-web        │ Connected   │ :8080→localhost:3000  │
│ 550e8400-e29b-41d4-a716-446655440000 │ office-db       │ Reconnecting│ :5432→localhost:5432  │
└──────────────────────────────────────┴─────────────────┴─────────────┴───────────────────────┘
```

**Example (JSON for Scripting):**
```bash
rssh status --format json
```

**Output:**
```json
[
  {
    "id": "f47ac10b-58cc-4372-a567-0e02b2c3d479",
    "profile_name": "home-web",
    "status": "Connected",
    "pid": 12345,
    "started_at": "2026-01-15T10:30:00Z",
    "reconnect_count": 0
  },
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "profile_name": "office-db",
    "status": "Reconnecting",
    "pid": null,
    "started_at": "2026-01-15T09:00:00Z",
    "reconnect_count": 3
  }
]
```

**Session Statuses:**

| Status | Description |
|--------|-------------|
| `Starting` | Session is initializing, SSH process spawning |
| `Connected` | Tunnel is active and working |
| `Reconnecting` | Connection lost, attempting to reconnect |
| `Disconnected` | Session ended normally |
| `Failed` | Session failed (auth error, host unreachable, etc.) |

---

### `rssh logs` — View Session Logs

View SSH output and application logs for sessions.

```bash
rssh logs [SESSION_ID] [OPTIONS]
```

**Arguments:**
- `[SESSION_ID]`: Optional—show logs for a specific session

**Options:**
| Option | Short | Description |
|--------|-------|-------------|
| `--follow` | `-f` | Stream logs in real-time (like `tail -f`) |
| `--lines <N>` | `-n` | Show last N lines (default: 50) |

**Examples:**
```bash
# View recent logs for all sessions
rssh logs

# Follow logs in real-time
rssh logs --follow

# View last 100 lines of a specific session
rssh logs f47ac10b -n 100
```

**Output:**
```
[2026-01-15 10:30:00] [INFO] Session started: f47ac10b-58cc-4372-a567-0e02b2c3d479
[2026-01-15 10:30:01] [DEBUG] SSH binary detected: C:\Windows\System32\OpenSSH\ssh.exe
[2026-01-15 10:30:01] [DEBUG] Spawning SSH process with args: -R localhost:8080:localhost:3000 ...
[2026-01-15 10:30:02] [INFO] Session connected
[2026-01-15 10:35:15] [WARN] Connection lost, reconnecting in 1s...
[2026-01-15 10:35:17] [INFO] Session reconnected (attempt 1)
```

---

### `rssh profile` — Manage Profiles

Profile management commands.

#### `rssh profile list`

List all saved profiles.

```bash
rssh profile list [OPTIONS]
```

**Options:**
| Option | Short | Description |
|--------|-------|-------------|
| `--format <FORMAT>` | `-f` | Output format: `human` (default) or `json` |

**Output:**
```
Profiles (3):
  • home-web
    Host: vps.example.com:22 | User: deploy
    Tunnels: 1 | Auto-reconnect: Yes

  • office-db
    Host: jump.company.com:22 | User: dbadmin
    Tunnels: 1 | Auto-reconnect: Yes

  • multi-tunnel
    Host: server.io:2222 | User: admin
    Tunnels: 3 | Auto-reconnect: Yes
```

---

#### `rssh profile show <NAME>`

Show detailed information about a profile.

```bash
rssh profile show <NAME>
```

**Output:**
```
Profile: home-web
══════════════════════════════════════════════════════════

Connection:
  Host:              vps.example.com
  Port:              22
  User:              deploy
  Authentication:    SSH Agent

Tunnels:
  ┌────────────────────┬────────────────────┐
  │ Remote             │ Local              │
  ├────────────────────┼────────────────────┤
  │ localhost:8080     │ localhost:3000     │
  └────────────────────┴────────────────────┘

Settings:
  Keep-alive Interval: 20 seconds
  Keep-alive Count:    3 (max missed)
  Auto-reconnect:      Yes
  Max Reconnect:       Unlimited

File: ~/.config/reverse-ssh-interface/profiles/home-web.toml
```

---

#### `rssh profile add`

Create a new profile interactively or via command-line options.

```bash
rssh profile add <NAME> [OPTIONS]
```

**Arguments:**
- `<NAME>`: Unique name for the profile (required)

**Options:**
| Option | Short | Description | Required |
|--------|-------|-------------|----------|
| `--host <HOST>` | `-H` | SSH server hostname or IP | Yes |
| `--user <USER>` | `-u` | SSH username | Yes |
| `--port <PORT>` | `-p` | SSH port (default: 22) | No |
| `--tunnel <SPEC>` | `-t` | Tunnel specification (can be repeated) | Yes (at least one) |
| `--key <PATH>` | `-k` | Path to SSH private key | No |
| `--no-reconnect` | | Disable auto-reconnection | No |

**Tunnel Specification Format:**
```
remote_port:local_host:local_port
```

**Examples:**

```bash
# Basic profile with one tunnel
rssh profile add webserver \
    --host vps.example.com \
    --user deploy \
    --tunnel 8080:localhost:3000

# Profile with custom port and key file
rssh profile add secure-server \
    --host secure.example.com \
    --port 2222 \
    --user admin \
    --key ~/.ssh/id_ed25519_secure \
    --tunnel 9000:localhost:8000

# Profile with multiple tunnels
rssh profile add dev-environment \
    --host dev.example.com \
    --user developer \
    --tunnel 8080:localhost:3000 \
    --tunnel 8443:localhost:443 \
    --tunnel 5432:localhost:5432 \
    --tunnel 6379:localhost:6379
```

---

#### `rssh profile remove <NAME>`

Delete a profile.

```bash
rssh profile remove <NAME>
```

**Example:**
```bash
rssh profile remove old-server
```

**Output:**
```
✓ Profile 'old-server' has been removed
```

**Note:** This only removes the profile configuration. It does not affect any active sessions using this profile.

---

## 🌐 Web Interface

The web interface provides a modern, responsive dashboard accessible from any browser.

### Starting the Web Server

```bash
# Using the binary
rssh-web

# Or with cargo
cargo run -p reverse_ssh_web_server

# Custom host and port
RSSH_WEB_HOST=0.0.0.0 RSSH_WEB_PORT=8080 rssh-web
```

**Default URL:** http://127.0.0.1:3000

### Web Dashboard Features

#### 📊 Dashboard Panel
- **Statistics Cards**: Active sessions, total profiles, active tunnels
- **Quick Actions**: One-click connect to saved profiles
- **Session List**: All active sessions with status indicators and stop buttons

#### 👤 Profiles Panel
- **Profile Grid**: Card layout showing all profiles
- **Profile Details**: Click to view full configuration
- **Create Profile**: Form-based profile creation
- **Delete Profile**: Remove profiles with confirmation

#### 📡 Sessions Panel
- **Session Table**: Detailed view with columns for status, profile, ID, duration, PID, reconnect count
- **Live Updates**: WebSocket connection for real-time status changes
- **Session Control**: Start/stop individual sessions
- **Stop All**: Emergency stop all sessions button

#### 📜 Logs Panel
- **Real-time Logs**: WebSocket-streamed log output
- **Log Levels**: Color-coded INFO, WARN, ERROR, DEBUG messages
- **Auto-scroll**: Automatically scrolls to newest entries
- **Search/Filter**: Find specific log entries

#### ⚙️ Settings Panel
- **Configuration Path**: View current config file location
- **SSH Settings**: View SSH binary path and options
- **About**: Version and system information

### Session Status Indicators

| Color | Status | Description |
|-------|--------|-------------|
| 🟡 Yellow | Starting | Session initializing |
| 🟢 Green | Connected | Tunnel active and working |
| 🟠 Orange | Reconnecting | Lost connection, retrying |
| 🔴 Red | Disconnected | Session ended |
| ⚫ Gray | Failed | Session failed to start |

### WebSocket Events

The web UI connects to `ws://host:port/ws` for real-time updates:

```javascript
// Event types received:
{
  "type": "SessionStarted",
  "session_id": "uuid",
  "profile_name": "home-web"
}

{
  "type": "SessionConnected",
  "session_id": "uuid"
}

{
  "type": "SessionDisconnected",
  "session_id": "uuid",
  "reason": "Connection reset by peer"
}

{
  "type": "SessionReconnecting",
  "session_id": "uuid",
  "attempt": 1,
  "delay_secs": 2
}

{
  "type": "SessionFailed",
  "session_id": "uuid",
  "error": "Authentication failed"
}
```

---

## 🖥️ Desktop GUI

The desktop GUI is a native application built with [Tauri](https://tauri.app/), providing a full-featured graphical interface with system integration.

### Features

#### 🎨 Modern Dark Theme
- Easy-on-the-eyes dark color scheme
- Consistent design language
- Responsive layout that works on various screen sizes

#### 🗂️ Sidebar Navigation
Five main sections:
- **Dashboard**: Overview with stats, quick actions, and active sessions
- **Profiles**: Grid view of all profiles with create/edit/delete
- **Sessions**: Detailed table of all sessions with controls
- **Logs**: Real-time log viewer with auto-scroll
- **Settings**: Configuration options and paths

#### 📊 Dashboard
- **Stats Cards**: Active sessions count, total profiles, active tunnels
- **Quick Actions**: Top 6 profiles for one-click connection
- **Active Sessions**: Live list with status indicators and stop buttons

#### 👤 Profiles Management
- **Profile Cards**: Visual grid showing all profiles
- **Profile Details Modal**: Full configuration view when clicked
- **Create Profile Modal**: Form with all options:
  - Name, Host, Port, User
  - Authentication type (SSH Agent, Key File, Password)
  - Tunnel editor with add/remove rows
  - Auto-reconnect toggle

#### 📡 Session Control
- **Start Session**: Click "Connect" on any profile card
- **Stop Session**: Click "Stop" button on active session
- **Stop All**: Stop all sessions at once
- **Auto-refresh**: Sessions list updates every 5 seconds

#### 📜 Real-time Logs
- **Log Levels**: Color-coded (success=green, warning=yellow, error=red, info=blue)
- **Timestamps**: Each entry shows time
- **Auto-scroll**: Toggle to follow new entries
- **Clear Logs**: Reset log view

#### 📌 System Tray
- **Tray Icon**: App runs in system tray when minimized
- **Tray Menu**:
  - Show/Hide Window
  - Stop All Sessions
  - Quit Application
- **Click to Open**: Single click shows main window

#### 🔔 Desktop Notifications
- **Connection Events**: Notified when sessions connect/disconnect
- **Error Alerts**: Notified when sessions fail
- **Non-intrusive**: Uses native OS notification system

#### ⌨️ Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+N` | Open Create Profile modal |
| `Ctrl+K` | Open Quick Connect modal |
| `Escape` | Close any open modal |

### Running the GUI

**Development Mode:**
```bash
cd gui
cargo tauri dev
```

**Production Build:**
```bash
cd gui
cargo tauri build
```

**Installers Location:**
- Windows: `gui/target/release/bundle/msi/*.msi`
- macOS: `gui/target/release/bundle/dmg/*.dmg`
- Linux: `gui/target/release/bundle/deb/*.deb` or `appimage/*.AppImage`

---

## 🔌 API Reference

The web server exposes a RESTful API with OpenAPI/Swagger documentation.

### Swagger UI

Interactive API documentation is available at:

**http://127.0.0.1:3000/swagger-ui/**

### Endpoints Summary

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/health` | Health check |
| `GET` | `/api/profiles` | List all profiles |
| `GET` | `/api/profiles/{name}` | Get profile by name |
| `POST` | `/api/profiles` | Create new profile |
| `DELETE` | `/api/profiles/{name}` | Delete profile |
| `GET` | `/api/sessions` | List all sessions |
| `POST` | `/api/sessions/{profile}/start` | Start session |
| `POST` | `/api/sessions/{session_id}/stop` | Stop session |
| `POST` | `/api/sessions/stop-all` | Stop all sessions |
| `WS` | `/ws` | WebSocket for events |

### Detailed API Documentation

#### Health Check

```http
GET /health
```

**Response:**
```json
{
  "status": "healthy"
}
```

---

#### List Profiles

```http
GET /api/profiles
```

**Response (200 OK):**
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "home-web",
    "host": "vps.example.com",
    "port": 22,
    "user": "deploy",
    "auth": "agent",
    "tunnels": [
      {
        "remote_bind": "localhost",
        "remote_port": 8080,
        "local_host": "localhost",
        "local_port": 3000
      }
    ],
    "auto_reconnect": true
  }
]
```

---

#### Get Profile

```http
GET /api/profiles/{name}
```

**Parameters:**
- `name`: Profile name

**Response (200 OK):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "home-web",
  "host": "vps.example.com",
  "port": 22,
  "user": "deploy",
  "auth": "agent",
  "tunnels": [...],
  "auto_reconnect": true,
  "keepalive_interval": 20
}
```

**Response (404 Not Found):**
```json
{
  "error": "Profile 'nonexistent' not found"
}
```

---

#### Create Profile

```http
POST /api/profiles
Content-Type: application/json
```

**Request Body:**
```json
{
  "name": "new-profile",
  "host": "server.example.com",
  "port": 22,
  "user": "admin",
  "auth": {
    "type": "agent"
  },
  "tunnels": [
    {
      "remote_port": 9000,
      "local_host": "localhost",
      "local_port": 3000
    }
  ]
}
```

**Response (201 Created):**
```json
{
  "id": "generated-uuid",
  "name": "new-profile",
  ...
}
```

**Response (400 Bad Request):**
```json
{
  "error": "At least one tunnel is required"
}
```

**Response (409 Conflict):**
```json
{
  "error": "Profile 'new-profile' already exists"
}
```

---

#### Delete Profile

```http
DELETE /api/profiles/{name}
```

**Response (200 OK):**
```json
{
  "message": "Profile deleted"
}
```

---

#### List Sessions

```http
GET /api/sessions
```

**Response (200 OK):**
```json
[
  {
    "id": "f47ac10b-58cc-4372-a567-0e02b2c3d479",
    "profile_name": "home-web",
    "status": "Connected",
    "started_at": "2026-01-15T10:30:00Z",
    "pid": 12345,
    "reconnect_count": 0
  }
]
```

---

#### Start Session

```http
POST /api/sessions/{profile_name}/start
```

**Parameters:**
- `profile_name`: Name of the profile to start

**Response (200 OK):**
```json
{
  "id": "new-session-uuid",
  "profile_name": "home-web",
  "status": "Starting"
}
```

---

#### Stop Session

```http
POST /api/sessions/{session_id}/stop
```

**Parameters:**
- `session_id`: UUID of the session to stop

**Response (200 OK):**
```json
{
  "message": "Session stopped"
}
```

---

#### Stop All Sessions

```http
POST /api/sessions/stop-all
```

**Response (200 OK):**
```json
{
  "message": "All sessions stopped",
  "count": 3
}
```

---

## ⚙️ Configuration

### Configuration Locations

Configuration files are stored in platform-specific directories:

| Platform | Directory |
|----------|-----------|
| **Linux** | `~/.config/reverse-ssh-interface/` |
| **macOS** | `~/Library/Application Support/com.reverse-ssh.reverse-ssh-interface/` |
| **Windows** | `%APPDATA%\reverse-ssh\reverse-ssh-interface\config\` |

### Directory Structure

```
<config-dir>/
├── config.toml          # Main application configuration
├── profiles/            # Profile configurations
│   ├── home-web.toml
│   ├── office-db.toml
│   └── ...
├── known_hosts          # Application-managed known hosts (if enabled)
└── logs/                # Log files (if file logging enabled)
    └── rssh.log
```

### Main Configuration (`config.toml`)

```toml
[general]
# Auto-start sessions from last run on application launch
auto_start_sessions = false

# Start GUI minimized to system tray
start_minimized = false

[ssh]
# Custom SSH binary path (auto-detected if not set)
# binary_path = "/usr/bin/ssh"

# Keep-alive interval in seconds (sent to prevent connection drops)
default_keepalive_interval = 20

# Maximum missed keep-alives before considering connection dead
default_keepalive_count = 3

# Host key verification mode
# Options:
#   "yes"        - Strict mode: reject unknown hosts, reject changed keys
#   "accept_new" - Accept new hosts on first connect, reject changed keys (default)
#   "no"         - Accept all hosts (insecure, for testing only)
strict_host_key_checking = "accept_new"

# Use application-managed known_hosts file instead of system default
# This isolates RSSH's known hosts from your regular SSH usage
use_app_known_hosts = true

[logging]
# Log level: trace, debug, info, warn, error
level = "info"

# Enable logging to file
file_logging = true

# Maximum log file size in megabytes
max_file_size_mb = 10

# Maximum number of log files to keep (older ones are deleted)
max_files = 5
```

### Profile Configuration

Each profile is stored as a separate TOML file in the `profiles/` directory.

**Full Profile Example (`profiles/production-server.toml`):**

```toml
# Unique identifier (auto-generated)
id = "550e8400-e29b-41d4-a716-446655440000"

# Profile name (must match filename without .toml)
name = "production-server"

# SSH server hostname or IP
host = "prod.example.com"

# SSH port (default: 22)
port = 22

# SSH username
user = "deploy"

# =============================================================================
# Authentication
# =============================================================================

[auth]
# Authentication type: "agent", "key_file", or "password"
type = "agent"

# For key_file authentication:
# type = "key_file"
# path = "/home/user/.ssh/id_ed25519"

# For password authentication (not recommended):
# type = "password"
# Note: Requires `sshpass` to be installed. Password is not stored.

# =============================================================================
# Tunnels
# =============================================================================

# You can define multiple tunnels. Each tunnel creates a reverse port forward.

[[tunnels]]
# Bind address on the remote server (usually "localhost" or "0.0.0.0")
remote_bind = "localhost"

# Port on the remote server that will accept connections
remote_port = 8080

# Local address to forward to
local_host = "localhost"

# Local port to forward to
local_port = 3000

[[tunnels]]
# Bind to all interfaces (requires GatewayPorts yes on server)
remote_bind = "0.0.0.0"
remote_port = 8443
local_host = "localhost"
local_port = 443

[[tunnels]]
# Forward to a different local machine
remote_bind = "localhost"
remote_port = 5432
local_host = "192.168.1.100"  # Internal database server
local_port = 5432

# =============================================================================
# Connection Settings
# =============================================================================

# Seconds between keep-alive messages
keepalive_interval = 20

# Max missed keep-alives before disconnect
keepalive_count = 3

# Automatically reconnect on connection loss
auto_reconnect = true

# Maximum reconnection attempts (0 = unlimited)
max_reconnect_attempts = 0

# =============================================================================
# Advanced SSH Options
# =============================================================================

# Custom SSH binary path (overrides global setting)
# ssh_path = "/usr/local/bin/ssh"

# Custom known_hosts file (overrides global setting)
# known_hosts_file = "/path/to/custom/known_hosts"

# Custom identity file (alternative to auth.path)
# identity_file = "/path/to/key"

# Extra SSH options passed directly to ssh command
[extra_options]
# Compression = "yes"
# TCPKeepAlive = "yes"
# ConnectTimeout = "10"
# StrictHostKeyChecking = "no"  # Override global setting for this profile
```

### Tunnel Binding Options

| remote_bind | Effect |
|-------------|--------|
| `localhost` | Only accessible from the SSH server itself |
| `127.0.0.1` | Same as localhost |
| `0.0.0.0` | Accessible from any IP (requires `GatewayPorts yes` on server) |
| `specific-ip` | Bind to a specific interface on the server |

### Authentication Methods Comparison

| Method | Security | Setup | Use Case |
|--------|----------|-------|----------|
| **SSH Agent** | ✅ Best | Add key to agent with `ssh-add` | Recommended for daily use |
| **Key File** | ⚠️ Good | Specify path to private key | When agent isn't available |
| **Password** | ❌ Poor | Requires `sshpass` | Legacy systems only |

---

## 🔒 Security Best Practices

### 1. Use SSH Agent Authentication

The SSH agent keeps your private keys in memory, never exposing them to disk or other processes:

```bash
# Start agent (usually automatic on modern systems)
eval $(ssh-agent)

# Add your key
ssh-add ~/.ssh/id_ed25519

# Verify
ssh-add -l
```

### 2. Use Ed25519 Keys

Ed25519 keys are faster and more secure than RSA:

```bash
# Generate a new Ed25519 key
ssh-keygen -t ed25519 -C "your-email@example.com"
```

### 3. Enable Host Key Verification

Always verify server identity to prevent man-in-the-middle attacks:

```toml
# config.toml
[ssh]
strict_host_key_checking = "accept_new"  # Or "yes" for maximum security
```

### 4. Configure Server-Side Security

On your SSH server, consider:

```bash
# /etc/ssh/sshd_config

# Only allow specific users to create tunnels
AllowTcpForwarding yes
PermitOpen any

# Limit to specific user
Match User tunneluser
    AllowTcpForwarding remote
    PermitOpen localhost:8080 localhost:8443
    X11Forwarding no
    PermitTTY no

# Enable GatewayPorts only if needed
GatewayPorts clientspecified
```

### 5. Use Firewall Rules

Only expose necessary ports:

```bash
# Example: UFW on Ubuntu
sudo ufw allow 22/tcp      # SSH
sudo ufw allow 8080/tcp    # Your tunnel port
sudo ufw enable
```

### 6. Monitor Access

Regularly check who's accessing your tunnels:

```bash
# View active connections on server
ss -tlnp | grep 8080

# Check SSH logs
sudo tail -f /var/log/auth.log
```

---

## 🏗️ Architecture

### Project Structure

```
reverse-ssh-interface/
├── core/                    # Shared Rust library
│   └── src/
│       ├── lib.rs           # Library entry point
│       ├── error.rs         # Error types
│       ├── prelude.rs       # Common imports
│       ├── config/          # Configuration management
│       │   ├── load.rs      # Config loading
│       │   ├── model.rs     # Config data structures
│       │   └── paths.rs     # Platform-specific paths
│       ├── ssh/             # SSH handling
│       │   ├── detect.rs    # SSH binary detection
│       │   ├── args.rs      # Command argument building
│       │   ├── spawn.rs     # Process spawning
│       │   └── known_hosts.rs # Host key management
│       ├── supervisor/      # Session management
│       │   ├── manager.rs   # Central session controller
│       │   ├── monitor.rs   # Connection monitoring
│       │   └── backoff.rs   # Reconnection logic
│       ├── storage/         # Data persistence
│       │   ├── state.rs     # Runtime state
│       │   └── keyring.rs   # Credential storage (planned)
│       ├── types/           # Data structures
│       │   ├── profile.rs   # Profile type
│       │   ├── session.rs   # Session type
│       │   └── events.rs    # Event system
│       └── util/            # Utilities
│           └── redact.rs    # Log redaction
├── cli/                     # Command-line interface
│   └── src/
│       ├── main.rs          # Entry point
│       ├── cmd/             # Subcommands
│       │   ├── up.rs        # Start tunnel
│       │   ├── down.rs      # Stop tunnel
│       │   ├── status.rs    # Show status
│       │   ├── logs.rs      # View logs
│       │   └── profile.rs   # Profile management
│       └── output/          # Output formatting
│           └── format.rs    # Human/JSON output
├── web/
│   └── server/              # REST API + WebSocket
│       └── src/
│           ├── main.rs      # Server entry
│           ├── state.rs     # Shared state
│           ├── static_files.rs # Web UI serving
│           └── routes/      # API endpoints
│               ├── health.rs
│               ├── profiles.rs
│               ├── sessions.rs
│               └── ws.rs    # WebSocket handler
├── gui/                     # Desktop application
│   ├── src/
│   │   └── main.rs          # Tauri backend
│   ├── ui/                  # Frontend
│   │   ├── index.html       # Main HTML
│   │   ├── styles.css       # Dark theme CSS
│   │   └── app.js           # Application logic
│   ├── icons/               # App icons (all platforms)
│   └── tauri.conf.json      # Tauri configuration
├── assets/                  # Default configs
├── docs/                    # Documentation
├── scripts/                 # Build scripts
└── tests/                   # Integration tests
```

### Data Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              User Interfaces                                 │
├─────────────────┬─────────────────────────┬────────────────────────────────┤
│      CLI        │        Web UI           │          Desktop GUI           │
│   (rssh)        │  (Browser + WebSocket)  │         (Tauri)                │
└────────┬────────┴───────────┬─────────────┴──────────────┬─────────────────┘
         │                    │                            │
         │     ┌──────────────┴──────────────┐             │
         │     │       Web Server            │             │
         │     │  (REST API + WebSocket)     │             │
         │     └──────────────┬──────────────┘             │
         │                    │                            │
         └────────────────────┼────────────────────────────┘
                              │
                    ┌─────────┴─────────┐
                    │   Core Library    │
                    │ (reverse_ssh_core)│
                    ├───────────────────┤
                    │ • SessionManager  │
                    │ • Config Loading  │
                    │ • SSH Detection   │
                    │ • Event System    │
                    └─────────┬─────────┘
                              │
                    ┌─────────┴─────────┐
                    │   SSH Process     │
                    │   (OpenSSH)       │
                    └─────────┬─────────┘
                              │
                    ┌─────────┴─────────┐
                    │   Your Server     │
                    │   (VPS/Cloud)     │
                    └───────────────────┘
```

### Event System

The core library uses a broadcast event system for real-time updates:

```rust
pub enum Event {
    SessionStarted { session_id: Uuid, profile_name: String },
    SessionConnected { session_id: Uuid },
    SessionDisconnected { session_id: Uuid, reason: String },
    SessionReconnecting { session_id: Uuid, attempt: u32, delay_secs: u64 },
    SessionFailed { session_id: Uuid, error: String },
    SessionStopped { session_id: Uuid },
    SessionOutput { session_id: Uuid, output: String },
}
```

All interfaces can subscribe to these events for real-time updates.

---

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run with output
cargo test --workspace -- --nocapture

# Run specific package tests
cargo test -p reverse_ssh_core

# Run specific test
cargo test -p reverse_ssh_core test_profile_serialization

# Run integration tests
cargo test -p integration_tests
```

### Test Categories

- **Unit Tests**: In `src/` files, testing individual functions
- **Integration Tests**: In `tests/` directory, testing component interaction
- **Manual Tests**: Scripts in `scripts/` for manual verification

---

## 🛠️ Development

### Development Scripts

```bash
# Build in development mode
./scripts/dev.sh

# Build for release
./scripts/build.sh

# Generate shell completions
./scripts/gen-completions.sh bash > completions/rssh.bash
./scripts/gen-completions.sh zsh > completions/_rssh
./scripts/gen-completions.sh fish > completions/rssh.fish
./scripts/gen-completions.sh powershell > completions/_rssh.ps1

# Package for distribution
./scripts/package.sh
```

### Adding a New Feature

1. **Core First**: Implement in `core/` library
2. **Test**: Add unit tests for the feature
3. **CLI**: Add command/option in `cli/`
4. **Web API**: Add endpoint in `web/server/`
5. **GUI**: Add UI in `gui/`
6. **Document**: Update README

---

## ❓ FAQ

### General

**Q: Why use this instead of just running SSH manually?**

A: Manual SSH (`ssh -R ...`) works but lacks:
- Automatic reconnection on network failures
- Profile management for saved configurations
- Real-time monitoring and logging
- GUI for non-terminal users

**Q: Is my traffic encrypted?**

A: Yes. All traffic through the tunnel is encrypted by SSH using strong cryptography (AES-256, ChaCha20-Poly1305, etc.).

**Q: Can I tunnel UDP traffic?**

A: No, SSH only supports TCP tunneling. For UDP, consider WireGuard or a dedicated UDP tunnel solution.

**Q: Does this work with non-standard SSH ports?**

A: Yes. Set `port = 2222` (or any port) in your profile configuration.

### Platform-Specific

**Q: Does this work on Windows?**

A: Yes! Windows 10/11 includes OpenSSH. The application auto-detects the SSH binary.

**Q: Can I use this on Android/Termux?**

A: The CLI works on Termux. Install Rust toolchain and build as usual.

**Q: Does the GUI work on Wayland?**

A: Yes, Tauri supports both X11 and Wayland on Linux.

### Troubleshooting

**Q: "Permission denied" when connecting?**

A: Check:
1. SSH key is added to agent: `ssh-add -l`
2. Public key is in server's `~/.ssh/authorized_keys`
3. Correct username in profile

**Q: Tunnel connects but I can't access the port?**

A: Check:
1. Local service is running on the specified port
2. Server firewall allows the remote port
3. If using `0.0.0.0`, ensure `GatewayPorts yes` on server

**Q: Connection keeps dropping?**

A: Try:
1. Increase `keepalive_interval` (default 20s)
2. Increase `keepalive_count` (default 3)
3. Check network stability

---

## 🗺️ Roadmap

### ✅ Completed

- [x] Core library with session management
- [x] CLI with all essential commands
- [x] Web server with REST API
- [x] Web UI with real-time updates
- [x] Desktop GUI (Tauri)
- [x] Profile management (CRUD)
- [x] Auto-reconnection with exponential backoff
- [x] WebSocket event streaming
- [x] OpenAPI/Swagger documentation
- [x] System tray integration (GUI)
- [x] Desktop notifications (GUI)
- [x] Cross-platform icons

### 🚧 Planned

- [ ] Session persistence across restarts
- [ ] Startup on boot option
- [ ] Keyring integration for secure credential storage
- [ ] Connection statistics and bandwidth monitoring
- [ ] Export/Import profiles (backup/restore)
- [ ] Multi-language support
- [ ] Mobile push notifications
- [ ] Profile sharing (encrypted)
- [ ] SSH multiplexing support
- [ ] Port forwarding presets (common services)

---

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. **Fork** the repository
2. **Create** a feature branch: `git checkout -b feature/amazing-feature`
3. **Write** tests for your changes
4. **Ensure** all tests pass: `cargo test --workspace`
5. **Format** code: `cargo fmt`
6. **Lint** code: `cargo clippy`
7. **Commit** with a descriptive message
8. **Push** to your fork
9. **Open** a Pull Request

### Code Style

- Follow Rust idioms and conventions
- Use meaningful variable names
- Add documentation comments for public APIs
- Keep functions focused and small

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- **Rust Community** - For the excellent async runtime (Tokio) and ecosystem
- **OpenSSH** - For the rock-solid SSH implementation
- **Tauri** - For the amazing cross-platform GUI framework
- **Axum** - For the ergonomic web framework
- **All Contributors** - For their time and effort

---

<div align="center">

**Made with ❤️ and 🦀 Rust**

[⬆ Back to Top](#-reverse-ssh-interface)

</div>
