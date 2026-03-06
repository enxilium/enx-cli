---
layout: default
title: Quick Start
nav_order: 3
---

# Quick Start

{: .no_toc }

Get from zero to a fully managed project in under five minutes.
{: .fs-6 .fw-300 }

## Table of Contents

{: .no_toc .text-delta }

1. TOC
   {:toc}

---

## 1. Install enx

```sh
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/enxilium/enx-cli/main/scripts/install.sh | sh

# Windows (PowerShell)
iwr -useb https://raw.githubusercontent.com/enxilium/enx-cli/main/scripts/install.ps1 | iex
```

The installer downloads the binary and runs `enx setup` for you. Restart your shell if prompted.

---

## 2. Initialize a project

Navigate to an existing project directory and initialize it:

```sh
cd ~/code/my-app
enx init
```

This creates an `enx.toml` file with sensible defaults and registers the project so enx knows about it.

---

## 3. Configure your project

Open the generated `enx.toml` and customize it:

```toml
[project]
name = "my-app"

[start]
commands = ["npm run dev"]

[tasks.test]
command = "npm test"
description = "Run the test suite"

[tasks.lint]
command = "npm run lint"
description = "Lint the codebase"

[open]
repo = "https://github.com/myorg/my-app"
```

---

## 4. Run tasks

```sh
enx start          # starts the dev server
enx run test       # runs the test task
enx lint            # shorthand — same as `enx run lint`
```

---

## 5. Navigate between projects

Register more projects, then jump between them instantly:

```sh
cd ~/code/api-service
enx init

# Now from anywhere:
enx cd api          # fuzzy-matches "api-service", cd's to it
enx cd my           # fuzzy-matches "my-app", cd's to it
```

List everything you've registered:

```
$ enx projects
  ├─ my-app          ~/code/my-app
  └─ api-service     ~/code/api-service
```

---

## 6. Manage environments

Define environment-specific config files in your `enx.toml`:

```toml
[env]
development = ".env.development"
staging = ".env.staging"
production = ".env.production"
```

Switch between them:

```sh
enx env staging     # switches to the staging environment
enx status          # shows current project, env, and branch
```

---

## 7. Bootstrap the full environment

Define setup and teardown steps:

```toml
[up]
steps = [
    "npm install",
    "enx run db:migrate",
]

[down]
steps = [
    "docker compose down",
]
```

```sh
enx up              # install deps, run migrations
enx down            # tear everything down
```

---

## What's Next?

- Browse the full [Command Reference]({{ site.baseurl }}/commands/) to see everything enx can do
- Learn about [Configuration]({{ site.baseurl }}/configuration/) for the complete `enx.toml` schema
- Set up [Shell Integration]({{ site.baseurl }}/shell-integration) for the best experience
