---
layout: default
title: "Global (config.toml)"
parent: Configuration
nav_order: 2
---

# Global Configuration — `config.toml`

{: .no_toc }

Settings and tasks that apply across all projects.
{: .fs-6 .fw-300 }

## Table of Contents

{: .no_toc .text-delta }

1. TOC
   {:toc}

---

## Location

| OS            | Path                                |
| :------------ | :---------------------------------- |
| Linux / macOS | `~/.config/enx/config.toml`         |
| Windows       | `~\AppData\Roaming\enx\config.toml` |

This file is created automatically by `enx setup`.

---

## `[defaults]`

Optional. Default settings for enx.

| Field          | Type   | Required | Description                                |
| :------------- | :----- | :------- | :----------------------------------------- |
| `projects_dir` | String | No       | Default directory for cloning new projects |

```toml
[defaults]
projects_dir = "~/code"
```

When you run `enx clone <repo>` without specifying a path, the repo is cloned into this directory.

---

## `[tasks]`

Optional. Global tasks available in every project. These serve as a fallback when a task is not found in the project's `enx.toml`.

The syntax is identical to [project tasks]({{ site.baseurl }}/configuration/project#tasks):

```toml
[tasks.scratch]
command = "code ~/scratch.md"
description = "Open scratch notes"

[tasks."git:prune"]
command = "git fetch --prune && git branch -vv | grep gone | awk '{print $1}' | xargs git branch -d"
description = "Prune dead git branches"
```

### Resolution order

When you run `enx run <task>`:

1. The task is looked up in the current project's `enx.toml`
2. If not found, it's looked up in `config.toml`
3. If not found in either, enx reports an error

This means project tasks always take precedence over global tasks of the same name.

---

## `is_configured`

Internal flag set to `true` after `enx setup` completes. Used to detect whether initial setup has been run.

```toml
is_configured = true
```

{: .note }

> You generally don't need to edit this field manually.

---

## Full Example

```toml
is_configured = true

[defaults]
projects_dir = "~/code"

[tasks.scratch]
command = "code ~/scratch.md"
description = "Open scratch notes"

[tasks."git:prune"]
command = "git fetch --prune && git branch -vv | grep gone | awk '{print $1}' | xargs git branch -d"
description = "Prune dead git branches"
```
