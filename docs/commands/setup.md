---
layout: default
title: enx setup
parent: Commands
nav_order: 15
---

# enx setup

{: .no_toc }

Initial setup for enx.
{: .fs-6 .fw-300 }

---

## Usage

```
enx setup
```

## Description

Performs first-time setup for enx. Run this after installation (and any time you want to fully reset shell integration). It:

1. Creates the global config directory:
    - Linux/macOS: `~/.config/enx/`
    - Windows: `~\AppData\Roaming\enx\`
2. Initializes `config.toml` with default settings
3. Creates an empty `registry.toml`
4. Rebuilds bash/zsh shell integration scripts and repairs source lines in shell rc files
5. Prompts for a default projects directory

{: .tip }

> `enx setup` is safe to run again — it won't overwrite existing configuration.

## Example

```
$ enx setup
  ▸ Setting up enx...
    ✓ Created ~/.config/enx/config.toml
    ✓ Created ~/.config/enx/registry.toml
    ✓ Shell integration installed for bash
  ✓ Setup complete!
```

## See Also

- [Installation]({{ site.baseurl }}/installation) — full install guide
- [Shell Integration]({{ site.baseurl }}/shell-integration) — how the shell wrapper works
