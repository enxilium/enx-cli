---
layout: default
title: enx projects
parent: Commands
nav_order: 11
---

# enx projects

{: .no_toc }

List all registered projects.
{: .fs-6 .fw-300 }

---

## Usage

```
enx projects
```

## Description

Displays a tree-style list of all projects registered with enx, showing each project's name and path. Projects whose directory no longer exists on disk are highlighted.

## Example

```
$ enx projects
  ├─ my-app          ~/code/my-app
  ├─ api-service     ~/code/work/api-service
  └─ dotfiles        ~/dotfiles
```

If a project's directory is missing:

```
  └─ old-project     ~/code/old-project (not found)
```

## See Also

- [`enx cd`]({{ site.baseurl }}/commands/cd) — navigate to a project
- [`enx init`]({{ site.baseurl }}/commands/init) — register a project
- [`enx remove`]({{ site.baseurl }}/commands/remove) — unregister a project
