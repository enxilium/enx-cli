---
layout: default
title: Installation
nav_order: 2
---

# Installation

{: .no_toc }

enx provides one-line installers for macOS, Linux, and Windows. You can also build from source.
{: .fs-6 .fw-300 }

## Table of Contents

{: .no_toc .text-delta }

1. TOC
   {:toc}

---

## macOS / Linux

Run the installer script — it downloads the latest nightly binary, places it in `~/.local/bin`, and runs `enx setup`:

```sh
curl -fsSL https://raw.githubusercontent.com/enxilium/enx-cli/main/scripts/install.sh | sh
```

### What the script does

1. Detects your OS and architecture (x86_64 / aarch64, Linux / macOS)
2. Downloads the matching binary from the latest `nightly` release on GitHub
3. Installs it to `~/.local/bin/enx` (override with `ENX_INSTALL_DIR`)
4. Runs `enx setup` to create your global config and configure shell integration

### Environment variables

| Variable          | Default            | Description                              |
| :---------------- | :----------------- | :--------------------------------------- |
| `ENX_REPO`        | `enxilium/enx-cli` | GitHub repo to download from (for forks) |
| `ENX_CHANNEL`     | `nightly`          | Release tag to install                   |
| `ENX_INSTALL_DIR` | `~/.local/bin`     | Where to place the binary                |

---

## Windows (PowerShell)

```powershell
iwr -useb https://raw.githubusercontent.com/enxilium/enx-cli/main/scripts/install.ps1 | iex
```

### What the script does

1. Detects architecture (AMD64)
2. Downloads the Windows binary from the latest `nightly` release
3. Installs to `~\AppData\Local\enx\bin\enx.exe`
4. Adds the install directory to your user `PATH` if it isn't already there
5. Runs `enx setup`

### Environment variables

| Variable          | Default                   | Description                  |
| :---------------- | :------------------------ | :--------------------------- |
| `ENX_REPO`        | `enxilium/enx-cli`        | GitHub repo to download from |
| `ENX_CHANNEL`     | `nightly`                 | Release tag to install       |
| `ENX_INSTALL_DIR` | `~\AppData\Local\enx\bin` | Where to place the binary    |

---

## Building from Source

Requires [Rust](https://www.rust-lang.org/tools/install) (stable).

```sh
git clone https://github.com/enxilium/enx-cli.git
cd enx-cli
cargo build --release
```

The binary will be at `target/release/enx`. Copy it somewhere on your `PATH` and run `enx setup`.

---

## Post-Install

After installation, `enx setup` runs automatically. It:

- Creates the global config directory (`~/.config/enx/` on Linux/macOS, `~\AppData\Roaming\enx\` on Windows)
- Initializes `config.toml` and `registry.toml`
- Configures shell integration for your detected shell (Bash, Zsh, Fish, or PowerShell)

If shell integration does not take effect immediately, restart your shell or source your shell config file.

{: .tip }

> If `~/.local/bin` (or the Windows equivalent) is not in your `PATH`, you'll need to add it manually to your shell profile.
