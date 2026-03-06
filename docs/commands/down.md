---
layout: default
title: enx down
parent: Commands
nav_order: 3
---

# enx down

{: .no_toc }

Tear down the project environment.
{: .fs-6 .fw-300 }

---

## Usage

```
enx down
```

## Description

Must be run inside a directory with an `enx.toml` file. Executes the steps defined in the `[down]` section — stopping services, cleaning up containers, etc.

Each step runs sequentially with a progress spinner.

## Example

```toml
# enx.toml
[down]
steps = [
    "docker compose down",
]
```

```
$ enx down
  ▸ Tearing down "my-app"
    ✓ docker compose down
  ✓ Environment torn down!
```

## See Also

- [`enx up`]({{ site.baseurl }}/commands/up) — bootstrap the environment
- [Configuration: `[down]`]({{ site.baseurl }}/configuration/project#up--down) — full schema
