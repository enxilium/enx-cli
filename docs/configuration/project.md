---
layout: default
title: "Project (enx.toml)"
parent: Configuration
nav_order: 1
---

# Project Configuration — `enx.toml`

{: .no_toc }

The per-project configuration file that defines everything about your project's enx setup.
{: .fs-6 .fw-300 }

## Table of Contents

{: .no_toc .text-delta }

1. TOC
   {:toc}

---

## Overview

`enx.toml` lives in the root of your project directory. It's created by `enx init` and read by most enx commands. Here is a complete example:

```toml
[project]
name = "my-app"

[env]
development = ".env.development"
staging = ".env.staging"
production = ".env.production"

[up]
steps = [
    "npm install",
    "enx run db:migrate",
]

[up.linux]
steps = ["sudo apt-get install -y libssl-dev"]

[up.macos]
steps = ["brew install openssl"]

[down]
steps = [
    "docker compose down",
]

[start]
commands = ["npm run dev"]

[tasks.test]
command = "npm run test"
description = "Run the test suite"

[tasks."db:migrate"]
command = "npx prisma migrate dev"
description = "Run database migrations"

[tasks."db:seed"]
command = "npx prisma db seed"
description = "Seed the database"

[tasks.deploy]
command = "bash deploy.sh"
description = "Deploy the application"

[open]
repo = "https://github.com/myorg/my-app"
ci = "https://github.com/myorg/my-app/actions"
docs = "https://myapp.readme.io"
```

---

## `[project]`

Required. Defines the project identity.

| Field  | Type   | Required | Description                                                                     |
| :----- | :----- | :------- | :------------------------------------------------------------------------------ |
| `name` | String | Yes      | Display name for the project. Used in `enx projects`, `enx cd`, and log output. |

```toml
[project]
name = "my-app"
```

---

## `[env]`

Optional. Maps environment names to dotenv files. Each key is an environment name and its value is the path to a `.env` file (relative to the project root).

```toml
[env]
development = ".env.development"
staging = ".env.staging"
production = ".env.production"
```

When you run `enx env staging`, enx reads `.env.staging` and makes those variables available to subsequent commands.

Environment files use standard dotenv format:

```
DATABASE_URL=postgres://localhost:5432/myapp_staging
APP_SECRET=staging-secret
```

### How environment variables are loaded

When running any task (`enx run`, `enx start`, etc.):

1. The current OS environment is used as a base
2. Variables from the active environment's dotenv file are overlaid
3. `ENX_PROJECT` is set to the project name
4. `ENX_ENV` is set to the active environment name

---

## `[up]` / `[down]`

Optional. Define lifecycle commands for bootstrapping and tearing down the project.

| Field   | Type             | Required | Description                            |
| :------ | :--------------- | :------- | :------------------------------------- |
| `steps` | Array of strings | Yes      | Shell commands to execute sequentially |

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

### Platform overrides

You can define platform-specific steps that replace the base steps on the matching OS:

```toml
[up]
steps = ["echo 'base steps'"]

[up.linux]
steps = ["sudo apt-get install -y libssl-dev"]

[up.macos]
steps = ["brew install openssl"]

[up.windows]
steps = ["choco install openssl"]
```

When running on macOS, only the `[up.macos]` steps run (the base `[up.steps]` are skipped). If no platform override matches the current OS, the base steps run.

---

## `[start]`

Optional. Defines the command(s) to start the project.

| Field      | Type             | Required | Description                               |
| :--------- | :--------------- | :------- | :---------------------------------------- |
| `commands` | Array of strings | Yes      | Commands to run when starting the project |

```toml
[start]
commands = ["npm run dev"]
```

---

## `[tasks]`

Optional. Defines custom tasks that can be run with `enx run <name>` or `enx <name>`.

Each task is a table with the following fields:

| Field         | Type   | Required | Description                                         |
| :------------ | :----- | :------- | :-------------------------------------------------- |
| `command`     | String | Yes      | The shell command to execute                        |
| `description` | String | No       | Human-readable description (shown in task listings) |

```toml
[tasks.test]
command = "npm run test"
description = "Run the test suite"

[tasks."db:migrate"]
command = "npx prisma migrate dev"
description = "Run database migrations"

[tasks."db:reset"]
command = "enx run db:migrate && enx run db:seed"
description = "Reset DB: migrate then seed"
```

{: .tip }

> Task names can contain colons, dashes, and other special characters — just wrap them in quotes in TOML: `[tasks."db:migrate"]`.

### Task nesting

Tasks can call other tasks by invoking `enx run` in their command. Since `enx` is on `$PATH`, this shells out to another enx invocation naturally.

### Argument forwarding

Arguments after `--` in `enx run <task> -- <args>` are appended to the task's command string.

---

## `[open]`

Optional. Defines named URL targets that can be opened with `enx open <target>`.

Each key is a target name and its value is a URL:

```toml
[open]
repo = "https://github.com/myorg/my-app"
ci = "https://github.com/myorg/my-app/actions"
docs = "https://myapp.readme.io"
```

URLs are opened in the system's default browser.
