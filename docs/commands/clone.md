---
layout: default
title: enx clone
parent: Commands
nav_order: 9
---

# enx clone

{: .no_toc }

Clone a repository and register it.
{: .fs-6 .fw-300 }

---

## Usage

```
enx clone <repo> [path]
```

## Arguments

| Argument | Required | Description                                                              |
| :------- | :------- | :----------------------------------------------------------------------- |
| `repo`   | Yes      | Git repository URL to clone                                              |
| `path`   | No       | Destination path. Defaults to the projects directory from global config. |

## Description

Runs `git clone <repo> [path]`, then automatically registers the cloned project in enx's registry. This is a convenience wrapper that combines cloning with project registration.

A spinner is shown during the clone operation.

## Example

```
$ enx clone https://github.com/myorg/api-service
  ▸ Cloning repository...
    ✓ Cloned to ~/code/api-service
  ✓ Registered "api-service"
```

## See Also

- [`enx init`]({{ site.baseurl }}/commands/init) — register an existing directory
- [`enx remove`]({{ site.baseurl }}/commands/remove) — unregister a project
