---
layout: default
title: enx status
parent: Commands
nav_order: 14
---

# enx status

{: .no_toc }

Show current project status.
{: .fs-6 .fw-300 }

---

## Usage

```
enx status
```

## Description

Displays a summary of the current project, including:

- Project name
- Project path
- Active environment
- Current git branch
- Running services (if any)

## Example

```
$ enx status
  Project:     api-service
  Path:        ~/code/work/api-service
  Environment: staging
  Branch:      feature/new-auth
  Services:    postgres ✓ | redis ✓
```

## See Also

- [`enx doctor`]({{ site.baseurl }}/commands/doctor) — detailed diagnostics
- [`enx env`]({{ site.baseurl }}/commands/env) — switch environments
