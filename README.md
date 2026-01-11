# Reverse SSH Interface

A fast, portable, and maintainable Reverse SSH Interface solution. This project aims to provide a unified core logic shared across multiple frontends (CLI, Web, and GUI), allowing for easy management of reverse SSH tunnels.

## Project Structure

This project is organized as a monorepo with a Rust workspace:

- **`core/`**: The heart of the application. A Rust library containing all shared logic, including SSH detection, process orchestration (`tokio`), configuration management, and state supervision.
- **`cli/`**: A lightweight Rust binary that exposes the core functionality via the command line.
- **`web/`**:
    - **`server/`**: A Rust `axum` server that acts as a bridge between the core and the web UI.
    - **`ui/`**: The frontend application (React/Svelte) that interacts with the server.
- **`gui/`**: A Tauri application that wraps the Web UI and bundles the Rust core for a native desktop experience.
- **`docs/`**: Project documentation.

## Prerequisites

- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install)
- **Node.js**: [Install Node.js](https://nodejs.org/) (for Web UI and Tauri)
- **OpenSSH**: An SSH client must be installed on the system (e.g., `openssh-client` on Linux/Termux, built-in SSH on Windows 10/11).

## Getting Started

1.  **Build the Core and CLI:**
    ```bash
    cargo build --workspace
    ```

2.  **Run the CLI:**
    ```bash
    cargo run -p reverse_ssh_cli -- --help
    ```

3.  **Run the Web Server:**
    ```bash
    cargo run -p reverse_ssh_web_server
    ```

## Documentation

See the `docs/` directory for detailed design documents:
- [Start Guide](docs/00.start.md)
- [Project Structure](docs/01.structure.md)

## License

[License Name]
