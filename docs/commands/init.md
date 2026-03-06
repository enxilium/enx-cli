---
layout: default
title: enx init
parent: Commands
nav_order: 8
---

# enx init

{: .no_toc }

Initialize a new project.
{: .fs-6 .fw-300 }

---

## Usage

```
enx init [path]
```

## Arguments

| Argument | Required | Description                                            |
| :------- | :------- | :----------------------------------------------------- |
| `path`   | No       | Path to initialize. Defaults to the current directory. |

## Description

Creates an `enx.toml` file with commented-out examples showing all available configuration sections, and registers the project in enx's registry.

If the directory already contains an `enx.toml`, enx will skip file creation but still register the project if it isn't already registered.

## Example

```
$ cd ~/code/my-new-project
$ enx init
  ✓ Created enx.toml
  ✓ Registered "my-new-project"
```

## See Also

- [`enx clone`]({{ site.baseurl }}/commands/clone) — clone and register in one step
- [Configuration: Project]({{ site.baseurl }}/configuration/project) — full `enx.toml` reference
