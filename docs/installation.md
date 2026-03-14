---
layout: default
title: Installation
nav_order: 2
---

# Installation

{: .no_toc }

enx provides a one-line POSIX-shell installer that works across macOS, Linux, and Windows POSIX environments. You can also build from source.
{: .fs-6 .fw-300 }

## Table of Contents

{: .no_toc .text-delta }

1. TOC
   {:toc}

---

## Install with POSIX shell (all OSes)

Run the installer script from a POSIX shell (`bash` or `zsh`) — it downloads the latest nightly binary and places it in `~/.local/bin`:

```sh
curl -fsSL https://raw.githubusercontent.com/enxilium/enx-cli/main/scripts/install.sh -o install.sh
sh install.sh
enx setup
```

On Windows, use Git Bash, MSYS2, Cygwin, or WSL.

### What the script does

1. Detects your OS and architecture (Linux/macOS: x86_64 or aarch64, Windows POSIX env: x86_64)
2. Downloads the matching binary from the latest `nightly` release on GitHub
3. Installs it to `~/.local/bin` (override with `ENX_INSTALL_DIR`)
4. Prints next steps to run `enx setup` in your current shell

### Environment variables

| Variable          | Default            | Description                              |
| :---------------- | :----------------- | :--------------------------------------- |
| `ENX_REPO`        | `enxilium/enx-cli` | GitHub repo to download from (for forks) |
| `ENX_CHANNEL`     | `nightly`          | Release tag to install                   |
| `ENX_INSTALL_DIR` | `~/.local/bin`     | Where to place the binary directory      |

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

After installation, run `enx setup`. It:

- Creates the global config directory (`~/.config/enx/` on Linux/macOS, `~\AppData\Roaming\enx\` on Windows)
- Initializes `config.toml` and `registry.toml`
- Regenerates shell integration for Bash and Zsh

If shell integration does not take effect immediately, restart your shell or source your shell config file.

{: .tip }

> If `~/.local/bin` is not in your `PATH`, add it to your shell profile.
