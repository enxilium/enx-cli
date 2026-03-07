---
layout: default
title: Shell Integration
nav_order: 6
---

# Shell Integration

{: .no_toc }

How enx integrates with your shell to enable features like `enx cd`.
{: .fs-6 .fw-300 }

## Table of Contents

{: .no_toc .text-delta }

1. TOC
   {:toc}

---

## Why Shell Integration?

A child process cannot change the parent shell's working directory. This means `enx cd` can't directly `cd` you into a project — it needs a shell wrapper function that captures the output and calls the shell's built-in `cd`.

`enx setup` detects your shell and installs this wrapper automatically. If you need to set it up manually, see the snippets below.

---

## Automatic Setup

When you run `enx setup`, it detects your shell and appends the appropriate wrapper function to your shell configuration file:

| Shell              | Config file |
| :----------------- | :---------- |
| Bash               | `~/.bashrc` |
| Zsh                | `~/.zshrc`  |
| Git Bash (Windows) | `~/.bashrc` |

{: .tip }

> If shell integration doesn't take effect immediately, restart your shell or source the config file (e.g. `source ~/.bashrc`).

---

## Manual Setup

### Bash / Zsh

Add this to your `~/.bashrc` or `~/.zshrc`:

```bash
enx() {
    if [ "$1" = "cd" ]; then
        shift
        local dir
        dir=$(command enx cd "$@")
        if [ $? -eq 0 ]; then
            cd "$dir" || return 1
        else
            return 1
        fi
    else
        command enx "$@"
    fi
}
```

---

## How It Works

The wrapper function intercepts calls to `enx`:

1. If the first argument is `cd`, it runs `command enx cd <args>` — the actual binary
2. The binary prints the matched project's path to **stdout**
3. The wrapper captures that path and calls the shell's built-in `cd`
4. For all other subcommands, the wrapper passes through to the binary directly

This means `enx cd` works transparently — you type `enx cd api` and your shell changes directory, even though the enx binary itself can't do that.

{: .note }

> All other enx commands work without shell integration. Only `enx cd` requires the wrapper function.

---

## Verifying Integration

After setup, test that it works:

```sh
# Register a project if you haven't already
enx init

# Navigate away and back
cd /tmp
enx cd my-project
pwd  # should show your project's path
```

If `pwd` shows the correct path, shell integration is working.
