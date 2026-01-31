# 🔗 Reverse SSH Interface

<div align="center">

![Rust](https://img.shields.io/badge/Rust-1.70+-orange?style=flat-square&logo=rust)
![License](https://img.shields.io/badge/License-MIT-blue?style=flat-square)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey?style=flat-square)

**A powerful, cross-platform tool for managing reverse SSH tunnels with ease.**

[Features](#-features) • [Installation](#-installation) • [Quick Start](#-quick-start) • [Documentation](#-documentation) • [API Reference](#-api-reference)

</div>

---

## 📖 The Story

Have you ever needed to access a device behind a firewall or NAT? Perhaps you're running a web server on your home network, managing IoT devices, or need remote access to a development machine. Traditional port forwarding requires router access and a static IP—but what if you don't have either?

**Reverse SSH tunnels** solve this problem elegantly. Instead of connecting *to* your device, your device connects *out* to a public server, creating a secure tunnel that allows inbound connections to reach your hidden machine.

**Reverse SSH Interface** takes this concept and wraps it in a beautiful, user-friendly package. Whether you prefer the command line, a web dashboard, or (coming soon) a native desktop application, this tool provides:

- 🔐 **Secure connections** using industry-standard SSH encryption
- 🔄 **Automatic reconnection** when connections drop
- 📊 **Real-time monitoring** of tunnel status
- 💾 **Profile management** for saving and reusing connection configurations
- 🌐 **Multiple interfaces** (CLI, Web UI, and future GUI) all powered by the same robust core

---

## ✨ Features

### Core Capabilities

| Feature | Description |
|---------|-------------|
| **Multi-Tunnel Support** | Create multiple reverse tunnels per connection, exposing different local services through a single SSH session |
| **Auto-Reconnection** | Intelligent reconnection with exponential backoff ensures your tunnels stay up even through network interruptions |
| **Profile Management** | Save, load, and organize your connection configurations as reusable profiles |
| **Cross-Platform** | Works on Windows, Linux, and macOS with automatic SSH binary detection |
| **Secure by Default** | Uses SSH agent authentication by default, supports key files, and proper known_hosts verification |
| **Real-Time Events** | WebSocket-based event streaming for live status updates |
| **OpenAPI/Swagger** | Full API documentation with interactive testing interface |

### Available Interfaces

| Interface | Status | Description |
|-----------|--------|-------------|
| **CLI (`rssh`)** | ✅ Complete | Full-featured command-line interface for power users |
| **Web UI** | ✅ Complete | Modern, responsive web dashboard with real-time updates |
| **Desktop GUI** | 🚧 Planned | Native Tauri application (coming soon) |

---

## 🔧 Installation

### Prerequisites

Before installing Reverse SSH Interface, ensure you have:

1. **Rust Toolchain** (1.70 or later)
   ```bash
   # Install via rustup (recommended)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **OpenSSH Client**
   - **Windows 10/11**: Built-in (Settings → Apps → Optional Features → OpenSSH Client)
   - **Linux**: `sudo apt install openssh-client` (Debian/Ubuntu) or `sudo dnf install openssh-clients` (Fedora)
   - **macOS**: Pre-installed

3. **A Public SSH Server** (for tunneling)
   - Any VPS or cloud server with SSH access
   - Your target machine must be able to make outbound SSH connections

### Building from Source

```bash
# Clone the repository
git clone https://github.com/mcpe500/reverse-ssh-interface.git
cd reverse-ssh-interface

# Build all components (release mode)
cargo build --release --workspace

# The binaries will be in target/release/
# - rssh (CLI)
# - rssh-web (Web Server)
```

### Installing the CLI

```bash
# Install the CLI globally
cargo install --path cli

# Verify installation
rssh --help
```

---

## 🚀 Quick Start

### 1. Create Your First Profile

Profiles define your SSH connection settings. Create one using the CLI:

```bash
# Create a profile named "my-server"
rssh profile add my-server \
    --host your-server.com \
    --user your-username \
    --tunnel 8080:localhost:3000
```

This creates a profile that:
- Connects to `your-server.com` as `your-username`
- Creates a reverse tunnel from port 8080 on the server to port 3000 on your local machine

### 2. Start the Tunnel

```bash
# Start the tunnel
rssh up my-server

# Check the status
rssh status
```

When connected, anyone accessing `your-server.com:8080` will be tunneled to your local port 3000!

### 3. Stop the Tunnel

```bash
# List running sessions to get the session ID
rssh status

# Stop by session ID
rssh down <session-id>
```

### 4. Using the Web Interface

```bash
# Start the web server
rssh-web

# Or with cargo
cargo run -p reverse_ssh_web_server
```

Open your browser to **http://127.0.0.1:3000** to access the web dashboard.

---

## 📚 Documentation

### Project Architecture

```
reverse-ssh-interface/
├── core/                    # Shared Rust library (the heart of the application)
│   └── src/
│       ├── config/          # Configuration loading and management
│       ├── ssh/             # SSH detection, argument building, process spawning
│       ├── supervisor/      # Session management and auto-reconnection
│       ├── storage/         # Profile storage and state persistence
│       ├── types/           # Data types (Profile, Session, Events)
│       └── util/            # Utility functions (logging, redaction)
├── cli/                     # Command-line interface (rssh binary)
├── web/
│   └── server/              # REST API + WebSocket server (rssh-web binary)
├── gui/                     # Desktop application (Tauri - planned)
├── assets/                  # Default configs and examples
├── docs/                    # Additional documentation
└── tests/                   # Integration tests
```

### Configuration Files

Configuration files are stored in platform-specific directories:

| Platform | Configuration Directory |
|----------|------------------------|
| **Linux** | `~/.config/reverse-ssh-interface/` |
| **macOS** | `~/Library/Application Support/com.reverse-ssh.reverse-ssh-interface/` |
| **Windows** | `%APPDATA%\reverse-ssh\reverse-ssh-interface\config\` |

#### Main Configuration (`config.toml`)

```toml
[general]
# Auto-start sessions from last run
auto_start_sessions = false

[ssh]
# Custom SSH binary path (auto-detected if not set)
# binary_path = "/usr/bin/ssh"

# Keep-alive settings
default_keepalive_interval = 20  # seconds
default_keepalive_count = 3      # max missed before disconnect

# Host key verification mode
# Options: "yes", "accept_new", "no"
strict_host_key_checking = "accept_new"

# Use app-managed known_hosts file
use_app_known_hosts = true

[logging]
level = "info"                   # trace, debug, info, warn, error
file_logging = true
max_file_size_mb = 10
max_files = 5
```

#### Profile Files (`profiles/*.toml`)

Each profile is stored as a separate TOML file:

```toml
# Profile: my-server.toml
name = "my-server"
host = "example.com"
port = 22
user = "myuser"

# Authentication method
[auth]
type = "agent"  # Recommended: uses SSH agent
# Alternative: type = "key_file", path = "/path/to/key"

# Tunnel specifications
[[tunnels]]
remote_bind = "localhost"
remote_port = 8080
local_host = "localhost"
local_port = 3000

[[tunnels]]
remote_bind = "0.0.0.0"    # Bind to all interfaces on server
remote_port = 8443
local_host = "localhost"
local_port = 443

# Connection settings
keepalive_interval = 20
keepalive_count = 3
auto_reconnect = true
max_reconnect_attempts = 0  # 0 = unlimited

# Optional: Additional SSH options
[extra_options]
Compression = "yes"
TCPKeepAlive = "yes"
```

---

## 💻 CLI Reference

The CLI binary is called `rssh` and provides the following commands:

### `rssh up <profile>` — Start a Tunnel

Starts a reverse SSH tunnel using the specified profile.

```bash
rssh up my-server
```

**Output:**
```
✓ Session started: a1b2c3d4-e5f6-7890-abcd-ef1234567890
  Profile: my-server
  Tunnels: localhost:8080 → localhost:3000
```

**What happens:**
1. Loads the profile configuration
2. Detects the SSH binary on your system
3. Spawns an SSH process with the appropriate arguments
4. Monitors the connection and handles reconnection if enabled

---

### `rssh down <session-id>` — Stop a Tunnel

Stops a running tunnel session by its UUID.

```bash
rssh down a1b2c3d4-e5f6-7890-abcd-ef1234567890
```

**Output:**
```
✓ Session stopped: a1b2c3d4-e5f6-7890-abcd-ef1234567890
```

---

### `rssh status` — View Tunnel Status

Displays the status of all active tunnel sessions.

```bash
# Human-readable format (default)
rssh status

# JSON format (for scripting)
rssh status --format json

# Specific session
rssh status --session a1b2c3d4-e5f6-7890-abcd-ef1234567890
```

**Output (Human):**
```
Active Sessions:
┌─────────────────────────────────────┬────────────┬──────────┬───────────────────────────┐
│ Session ID                          │ Profile    │ Status   │ Tunnels                   │
├─────────────────────────────────────┼────────────┼──────────┼───────────────────────────┤
│ a1b2c3d4-e5f6-7890-abcd-ef12345678  │ my-server  │ Connected│ :8080 → localhost:3000    │
└─────────────────────────────────────┴────────────┴──────────┴───────────────────────────┘
```

**Output (JSON):**
```json
[
  {
    "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "profile_name": "my-server",
    "status": "Connected",
    "pid": 12345,
    "started_at": "2024-01-15T10:30:00Z"
  }
]
```

---

### `rssh logs [session-id]` — View Logs

View SSH output and logs for sessions.

```bash
# View all logs
rssh logs

# View logs for a specific session
rssh logs a1b2c3d4-e5f6-7890-abcd-ef1234567890

# Follow logs in real-time
rssh logs --follow

# Show last 100 lines
rssh logs -n 100
```

---

### `rssh profile` — Manage Profiles

Subcommands for managing connection profiles.

#### `rssh profile list` — List All Profiles

```bash
rssh profile list
rssh profile list --format json
```

**Output:**
```
Profiles:
  • my-server (example.com:22)
  • home-pi (192.168.1.100:22)
  • work-server (office.example.com:2222)
```

#### `rssh profile show <name>` — Show Profile Details

```bash
rssh profile show my-server
```

**Output:**
```
Profile: my-server
─────────────────────────────────────
Host:     example.com
Port:     22
User:     myuser
Auth:     SSH Agent

Tunnels:
  • localhost:8080 → localhost:3000
  • 0.0.0.0:8443 → localhost:443

Settings:
  Keep-alive:    20s (max 3 failures)
  Auto-reconnect: Yes
  Max attempts:  Unlimited
```

#### `rssh profile add` — Create a New Profile

```bash
rssh profile add <name> --host <host> --user <user> [options]
```

**Options:**
| Option | Short | Description |
|--------|-------|-------------|
| `--host <host>` | `-H` | SSH server hostname or IP (required) |
| `--user <user>` | `-u` | SSH username (required) |
| `--port <port>` | `-p` | SSH port (default: 22) |
| `--tunnel <spec>` | `-t` | Tunnel spec: `remote_port:local_host:local_port` |
| `--key <path>` | `-k` | Path to SSH private key file |

**Examples:**

```bash
# Basic profile with one tunnel
rssh profile add webdev \
    --host vps.example.com \
    --user deploy \
    --tunnel 8080:localhost:3000

# Profile with multiple tunnels
rssh profile add fullstack \
    --host server.com \
    --user admin \
    --port 2222 \
    --tunnel 8080:localhost:3000 \
    --tunnel 8443:localhost:443 \
    --key ~/.ssh/id_ed25519
```

#### `rssh profile remove <name>` — Delete a Profile

```bash
rssh profile remove my-server
```

**Output:**
```
✓ Profile 'my-server' removed
```

---

## 🌐 Web Interface

The web interface provides a modern, responsive dashboard for managing your reverse SSH tunnels.

### Starting the Web Server

```bash
# Using the binary directly
rssh-web

# Or with cargo
cargo run -p reverse_ssh_web_server

# Custom port/host
RSSH_WEB_HOST=0.0.0.0 RSSH_WEB_PORT=8080 rssh-web
```

**Default URL:** http://127.0.0.1:3000

### Web Dashboard Features

| Feature | Description |
|---------|-------------|
| **Profile Panel** | View all saved profiles, create new ones, delete existing profiles |
| **Session Panel** | Start/stop tunnels, view real-time status with colored indicators |
| **Live Updates** | WebSocket connection provides instant status changes |
| **Toast Notifications** | Visual feedback for all actions (success/error) |
| **Responsive Design** | Works on desktop and mobile browsers |
| **Dark Theme** | Easy on the eyes with a modern dark color scheme |

### Session Status Indicators

| Color | Status | Description |
|-------|--------|-------------|
| 🟡 Yellow | Starting | Session is initializing |
| 🟢 Green | Connected | Tunnel is active and working |
| 🟠 Orange | Reconnecting | Lost connection, attempting to reconnect |
| 🔴 Red | Disconnected | Session has ended |
| ⚫ Gray | Failed | Session failed to start or crashed |

---

## 🔌 API Reference

The web server exposes a RESTful API with full OpenAPI/Swagger documentation.

### Swagger UI

Access the interactive API documentation at:

**http://127.0.0.1:3000/swagger-ui/**

### API Endpoints

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

#### Profiles

##### List All Profiles

```http
GET /api/profiles
```

**Response:**
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "my-server",
    "host": "example.com",
    "port": 22,
    "user": "myuser",
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

##### Get Profile by Name

```http
GET /api/profiles/{name}
```

##### Create Profile

```http
POST /api/profiles
Content-Type: application/json

{
  "name": "new-profile",
  "host": "server.example.com",
  "port": 22,
  "user": "admin",
  "tunnels": [
    {
      "remote_port": 9000,
      "local_port": 3000
    }
  ]
}
```

**Response:**
```json
{
  "id": "generated-uuid",
  "name": "new-profile",
  ...
}
```

##### Delete Profile

```http
DELETE /api/profiles/{name}
```

---

#### Sessions

##### List All Sessions

```http
GET /api/sessions
```

**Response:**
```json
[
  {
    "id": "session-uuid",
    "profile_name": "my-server",
    "status": "Connected",
    "started_at": "2024-01-15T10:30:00Z",
    "pid": 12345,
    "reconnect_count": 0
  }
]
```

##### Start Session

```http
POST /api/sessions/{profile_name}/start
```

**Response:**
```json
{
  "id": "new-session-uuid",
  "profile_name": "my-server",
  "status": "Starting"
}
```

##### Stop Session

```http
POST /api/sessions/{session_id}/stop
```

---

#### WebSocket Events

Connect to the WebSocket endpoint for real-time updates:

```
ws://127.0.0.1:3000/ws
```

**Event Types:**

```json
// Session started
{
  "type": "SessionStarted",
  "session_id": "uuid",
  "profile_name": "my-server"
}

// Session connected
{
  "type": "SessionConnected",
  "session_id": "uuid"
}

// Session disconnected
{
  "type": "SessionDisconnected",
  "session_id": "uuid",
  "reason": "Connection lost"
}

// Session reconnecting
{
  "type": "SessionReconnecting",
  "session_id": "uuid",
  "attempt": 1
}

// Session stopped
{
  "type": "SessionStopped",
  "session_id": "uuid"
}

// Session error
{
  "type": "SessionError",
  "session_id": "uuid",
  "error": "SSH process exited with code 255"
}
```

---

## 🔒 Security Considerations

### Authentication Methods

| Method | Security | Recommendation |
|--------|----------|----------------|
| **SSH Agent** | ✅ High | **Recommended** - Keys never leave the agent |
| **Key File** | ⚠️ Medium | Ensure proper file permissions (600) |
| **Password** | ❌ Low | Avoid - requires `sshpass` and stores credentials |

### Host Key Verification

The `strict_host_key_checking` setting controls how unknown hosts are handled:

| Mode | Behavior | Use Case |
|------|----------|----------|
| `yes` | Reject unknown hosts | Production environments |
| `accept_new` | Accept new hosts, reject changed keys | **Default** - Good balance |
| `no` | Accept all hosts | Testing only (insecure!) |

### Best Practices

1. **Use SSH Agent**: Let the agent manage your keys
2. **Use Ed25519 Keys**: More secure and faster than RSA
3. **Enable Known Hosts**: Always verify server identity
4. **Restrict Server Access**: Only allow specific ports/users on your SSH server
5. **Use Strong Passwords**: For your SSH keys and server accounts
6. **Firewall Rules**: Only expose necessary ports on the public server

---

## 🏗️ Development

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run with output
cargo test --workspace -- --nocapture

# Run specific test
cargo test --package reverse_ssh_core test_profile_serialization
```

### Development Scripts

```bash
# Development build with hot reload (if available)
./scripts/dev.sh

# Production build
./scripts/build.sh

# Generate shell completions
./scripts/gen-completions.sh

# Package for distribution
./scripts/package.sh
```

### Code Structure

The project follows a clean architecture with separation of concerns:

- **Core Library** (`reverse_ssh_core`): Zero dependencies on frontends, fully testable
- **CLI** (`reverse_ssh_cli`): Thin wrapper that calls core functions
- **Web Server** (`reverse_ssh_web_server`): REST API + WebSocket, uses core

---

## 🗺️ Roadmap

### Completed ✅

- [x] Core library with full session management
- [x] CLI with all essential commands
- [x] Web server with REST API
- [x] Web UI with real-time updates
- [x] Profile management (CRUD)
- [x] Auto-reconnection with exponential backoff
- [x] WebSocket event streaming
- [x] OpenAPI/Swagger documentation

### Planned 🚧

- [ ] **Desktop GUI** (Tauri application)
- [ ] **System tray** integration
- [ ] **Startup on boot** option
- [ ] **Session persistence** across restarts
- [ ] **Keyring integration** for secure credential storage
- [ ] **Connection statistics** and bandwidth monitoring
- [ ] **Export/Import** profiles
- [ ] **Multi-language** support
- [ ] **Notifications** (desktop/mobile push)

---

## ❓ FAQ

### Why use this instead of just running SSH manually?

While you *can* run `ssh -R ...` directly, Reverse SSH Interface provides:
- **Persistence**: Automatic reconnection when connections drop
- **Management**: Easy profile creation and organization
- **Monitoring**: Real-time status and logging
- **Convenience**: No need to remember complex SSH arguments

### Can I use this on Android/Termux?

Yes! The CLI works on Termux. Install the Rust toolchain and build as usual.

### Does this work with non-standard SSH ports?

Absolutely. Set the `port` field in your profile configuration.

### Can I tunnel UDP traffic?

No, SSH only supports TCP tunneling. For UDP, consider alternatives like WireGuard.

### Is my traffic encrypted?

Yes, all traffic through the tunnel is encrypted by SSH.

---

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- The Rust community for excellent async runtime (Tokio)
- OpenSSH for the rock-solid SSH implementation
- Axum for the ergonomic web framework
- All contributors and users of this project

---

<div align="center">

**Made with ❤️ and 🦀 Rust**

[⬆ Back to Top](#-reverse-ssh-interface)

</div>
