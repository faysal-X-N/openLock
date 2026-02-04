# Shield 密码管理器

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-blue.svg)]()

[English](README.md) | [中文](README_CN.md)

Shield 是一款基于 Rust 构建的安全、本地优先的密码管理器。它提供强大的加密保护、现代化的用户界面以及命令行工具，确保您的凭据安全且易于访问。

## 主要特性

- **🔒 强安全性**:
  - 数据存储采用 **AES-256-GCM** 加密。
  - 主密码使用 **Argon2id** 进行密钥派生。
  - 使用 **Secrecy** 库进行内存安全保护。
  
- **💻 多端支持**:
  - **GUI**: 基于 `egui` 构建的现代化桌面应用 (Windows/Linux/macOS)。
  - **CLI**: 强大的命令行工具，适合脚本自动化和极客用户。
  - **TUI**: 终端用户界面，适合在服务器或无头环境中使用。

- **🌐 国际化支持**:
  - 完整支持英文和简体中文。
  - 支持自动检测系统语言，并提供手动切换选项。

- **🚀 高性能**:
  - 基于 Rust 构建，启动速度极快，内存占用极低。
  - 使用本地 SQLite 数据库，数据持久化安全可靠。

## 安装

### 预编译包
请从 [Releases](https://github.com/shield/shield/releases) 页面下载最新的安装包。

### 源码编译

前置要求:
- [Rust 工具链](https://rustup.rs/) (1.70+)

```bash
# 克隆仓库
git clone https://github.com/your-username/shield.git
cd shield

# 编译所有组件
cargo build --release

# 运行 GUI
cargo run -p shield-gui --release

# 运行 CLI
cargo run -p shield-cli --release
```

## 使用指南

### GUI (图形界面)
启动 `Shield.exe` (Windows) 或 `shield-gui` (Linux/macOS)。
1. **初始化**: 首次启动时设置主密码。
2. **管理**: 添加、编辑、删除和搜索密码条目。
3. **剪贴板**: 一键复制用户名和密码。

### CLI (命令行)
```bash
# 初始化密码库
shield-cli init

# 添加条目
shield-cli add "GitHub" --username "myuser"

# 获取密码
shield-cli get "GitHub"
```

## 项目结构

- `shield-core`: 核心库，包含加密逻辑、数据库操作和数据模型。
- `shield-gui`: 桌面应用程序 (eframe/egui)。
- `shield-cli`: 命令行工具 (clap)。
- `shield-tui`: 终端界面 (ratatui)。
- `shield-android`: Android 客户端 (Kotlin/Compose)。

## 许可证

本项目基于 MIT 许可证开源 - 详见 [LICENSE](LICENSE) 文件。
