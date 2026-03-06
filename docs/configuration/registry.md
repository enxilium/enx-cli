---
layout: default
title: "Registry (registry.toml)"
parent: Configuration
nav_order: 3
---

# Project Registry — `registry.toml`

{: .no_toc }

Automatically managed list of all registered projects and their paths.
{: .fs-6 .fw-300 }

---

## Location

| OS            | Path                                  |
| :------------ | :------------------------------------ |
| Linux / macOS | `~/.config/enx/registry.toml`         |
| Windows       | `~\AppData\Roaming\enx\registry.toml` |

---

## Description

The registry is a simple mapping of project names to their filesystem paths. It is automatically managed by enx — you typically don't need to edit it by hand.

### How entries are added

- `enx init` registers the current (or specified) directory
- `enx clone` clones a repo and registers it

### How entries are removed

- `enx remove <name>` unregisters a project (does not delete files)

---

## Format

The registry uses a TOML table under `[projects]` where each key is a project name and each value is the absolute path:

```toml
[projects]
my-app = "/home/user/code/my-app"
api-service = "/home/user/code/work/api-service"
dotfiles = "/home/user/dotfiles"
```

---

## Which commands use the registry?

| Command        | How it uses the registry          |
| :------------- | :-------------------------------- |
| `enx cd`       | Looks up the project path by name |
| `enx projects` | Lists all entries                 |
| `enx init`     | Adds a new entry                  |
| `enx clone`    | Adds a new entry after cloning    |
| `enx remove`   | Removes an entry                  |

{: .warning }

> If you manually edit the registry and introduce invalid TOML or duplicate keys, enx commands that read the registry will fail. Let enx manage this file when possible.
