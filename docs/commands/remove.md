---
layout: default
title: enx remove
parent: Commands
nav_order: 10
---

# enx remove

{: .no_toc }

Remove a project from the registry.
{: .fs-6 .fw-300 }

---

## Usage

```
enx remove <name>
```

## Arguments

| Argument | Required | Description                       |
| :------- | :------- | :-------------------------------- |
| `name`   | Yes      | Name of the project to unregister |

## Description

Removes a project from enx's registry. You will be prompted to confirm before removal.

{: .note }

> This does **not** delete the project directory or any files on disk — it only removes the entry from enx's registry so the project no longer appears in `enx projects` or `enx cd`.

## Example

```
$ enx remove api-service
  ? Remove "api-service" from registry? (y/n) y
  ✓ Removed "api-service"
```

## See Also

- [`enx projects`]({{ site.baseurl }}/commands/projects) — list registered projects
- [`enx init`]({{ site.baseurl }}/commands/init) — register a project
