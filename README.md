# Shield Password Manager

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-blue.svg)]()

[English](README.md) | [中文](README_CN.md)

Shield is a secure, local-first password manager built with Rust. It provides robust encryption, a modern user interface, and command-line tools, ensuring your credentials are safe and accessible.

## Features

- **🔒 Strong Security**:
  - **AES-256-GCM** encryption for vault data.
  - **Argon2id** for master password key derivation.
  - **Secrecy** crate for protected memory handling.
  
- **💻 Multiple Interfaces**:
  - **GUI**: A modern desktop application built with `egui` (Windows/Linux/macOS).
  - **CLI**: A powerful command-line interface for scripting and power users.
  - **TUI**: A terminal-based user interface for quick access in headless environments.

- **🌐 Internationalization**:
  - Full English and Chinese (Simplified) support.
  - Automatic system language detection with manual override.

- **🚀 Performance**:
  - Built with Rust for blazing fast startup and low memory footprint.
  - Local SQLite database for reliable data persistence.

## Installation

### Pre-built Binaries
Download the latest installer from the [Releases](https://github.com/shield/shield/releases) page.

### Build from Source

Requirements:
- [Rust Toolchain](https://rustup.rs/) (1.70+)

```bash
# Clone the repository
git clone https://github.com/your-username/shield.git
cd shield

# Build all components
cargo build --release

# Run GUI
cargo run -p shield-gui --release

# Run CLI
cargo run -p shield-cli --release
```

## Usage

### GUI
Launch `Shield.exe` (Windows) or `shield-gui` (Linux/macOS).
1. **Initialize**: Set a master password on first launch.
2. **Manage**: Add, edit, delete, and search for entries.
3. **Clipboard**: One-click copy for usernames and passwords.

### CLI
```bash
# Initialize vault
shield-cli init

# Add entry
shield-cli add "GitHub" --username "myuser"

# Get password
shield-cli get "GitHub"
```

## Project Structure

- `shield-core`: Core library containing encryption, database, and models.
- `shield-gui`: Desktop application (eframe/egui).
- `shield-cli`: Command-line tool (clap).
- `shield-tui`: Terminal UI (ratatui).
- `shield-android`: Android client (Kotlin/Compose).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
