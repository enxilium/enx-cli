---
layout: default
title: enx up
parent: Commands
nav_order: 2
---

# enx up

{: .no_toc }

Bootstrap the project environment.
{: .fs-6 .fw-300 }

## Table of Contents

{: .no_toc .text-delta }

1. TOC
   {:toc}

---

## Usage

```
enx up
```

## Description

Must be run inside a directory with an `enx.toml` file. Executes the steps defined in the `[up]` section to bootstrap the project environment — installing dependencies, running migrations, starting services, etc.

### Platform overrides

If platform-specific steps are defined (e.g. `[up.linux]`, `[up.macos]`, `[up.windows]`), those steps are used instead of the base `[up.steps]` on the matching platform.

### Step execution

Each step is executed sequentially as a shell command. A spinner is displayed for each step, and it reports success or failure when it completes.

## Example

```toml
# enx.toml
[up]
steps = [
    "npm install",
    "enx run db:migrate",
]

[up.linux]
steps = ["sudo apt-get install -y libssl-dev"]

[up.macos]
steps = ["brew install openssl"]
```

```
$ enx up
  ▸ Running setup for "my-app"
    ✓ npm install
    ✓ enx run db:migrate
  ✓ Environment ready!
```

{: .tip }

> `enx up` is designed to be idempotent — safe to run multiple times.

## See Also

- [`enx down`]({{ site.baseurl }}/commands/down) — tear down the environment
- [Configuration: `[up]`]({{ site.baseurl }}/configuration/project#up--down) — full schema
