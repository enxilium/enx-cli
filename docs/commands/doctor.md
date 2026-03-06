---
layout: default
title: enx doctor
parent: Commands
nav_order: 13
---

# enx doctor

{: .no_toc }

Diagnose the project environment.
{: .fs-6 .fw-300 }

---

## Usage

```
enx doctor
```

## Description

Runs a series of diagnostic checks on the current project and reports pass/fail for each. Checks include:

- Whether `enx.toml` exists and is valid
- Whether required environment variables are set
- Whether services are healthy
- Whether expected tools are available on `PATH`

## Example

```
$ enx doctor
  Checking api-service...
  ✓ enx.toml found
  ✓ node 20.11.0
  ✗ python 3.12.1 — found 3.11.4
  ✓ postgres — healthy
  ✓ redis — healthy
  ✓ DATABASE_URL set
  ✗ APP_SECRET not set (expected in staging env)
  Result: 5/7 checks passed
```

## See Also

- [`enx status`]({{ site.baseurl }}/commands/status) — quick status view
- [`enx up`]({{ site.baseurl }}/commands/up) — bootstrap the environment
