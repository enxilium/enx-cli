---
layout: default
title: enx run
parent: Commands
nav_order: 6
---

# enx run

{: .no_toc }

Run a custom task defined in config.
{: .fs-6 .fw-300 }

## Table of Contents

{: .no_toc .text-delta }

1. TOC
   {:toc}

---

## Usage

```
enx run [task] [-- args...]
```

## Arguments

| Argument | Required | Description                                                     |
| :------- | :------- | :-------------------------------------------------------------- |
| `task`   | No       | Name of the task to run. If omitted, lists all available tasks. |
| `args`   | No       | Additional arguments forwarded to the task command (after `--`) |

## Description

Runs a named task by looking it up in configuration. If no task name is given, displays all available tasks from both the project and global config.

### Task resolution order

1. Look up the task in the current project's `enx.toml` `[tasks]` section
2. If not found, look up in the global `~/.config/enx/config.toml` `[tasks]` section
3. If not found in either, report an error

### Argument forwarding

Everything after `--` is appended to the task's command string:

```sh
enx run deploy -- --dry-run --verbose
# Executes: bash deploy.sh --dry-run --verbose
```

### Shorthand

Any unrecognized subcommand is treated as a task name:

```sh
enx lint          # equivalent to: enx run lint
enx test          # equivalent to: enx run test
```

## Examples

List all tasks:

```
$ enx run
  Available tasks:
    ├─ test        Run the test suite
    ├─ lint        Lint the codebase
    └─ deploy      Deploy the application
```

Run a specific task:

```
$ enx run test
  ▸ Running task "test"
  [npm test output]
```

## See Also

- [Configuration: `[tasks]`]({{ site.baseurl }}/configuration/project#tasks) — defining tasks
- [Configuration: Global tasks]({{ site.baseurl }}/configuration/global#tasks) — global fallback tasks
