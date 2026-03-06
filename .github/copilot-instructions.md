# enx — Agent Instructions

## 1. Project Overview

**enx** is a cross-platform CLI developer workflow tool built in Rust. It serves as:
- A global project navigator (`enx cd`)
- A per-project environment bootstrapper (`enx up` / `enx down`)
- A per-project and global custom task runner (`enx <task>`)
- A service manager, environment switcher, and general developer quality-of-life tool

**Target platforms:** Linux, macOS, Windows.
**Language:** Rust (this is also a learning project — treat every implementation decision as a teaching opportunity).

---

## 2. Tutoring Philosophy

The user is learning Rust through this project. When writing or suggesting code:

1. **Explain every new Rust concept the first time it appears.** This includes ownership, borrowing, lifetimes, traits, enums, pattern matching, error handling (`Result`, `?` operator, `thiserror`/`anyhow`), iterators, closures, generics, trait objects, `async`/`await`, modules, crates, macros, smart pointers, and any other concept as it naturally arises.
2. **Prefer idiomatic Rust.** Never write "C-style" Rust. Use iterators over manual loops where appropriate. Use `Option` and `Result` instead of sentinel values. Use enums over stringly-typed logic. Derive traits. Use `impl` blocks. Pattern match exhaustively.
3. **Introduce concepts incrementally.** Don't dump every Rust concept at once. Introduce them as the feature being built requires them.
4. **Explain *why*, not just *what*.** When suggesting a pattern, explain why Rust does it that way (e.g., "we use `&str` here instead of `String` because we don't need ownership — we're just reading").
5. **Call out common gotchas.** Borrow checker issues, `String` vs `&str`, `clone()` overuse, lifetime elision, `move` closures, etc.
6. **Show idiomatic error handling.** Start with `anyhow` for the application layer. Introduce `thiserror` for library-like internal modules. Explain when to use which.
7. **Teach testing as you go.** Show unit tests for logic, integration tests for CLI behavior. Teach (test)]`, `mod tests`, `assert_eq!`, `assert!(matches!(...))`, and test organization.
8. **Teach project structure.** Explain `lib.rs` vs `main.rs`, module hierarchy, `pub` visibility, re-exports, and when to split into separate crates in a workspace.
9. **Code quality.** Use `clippy` and `rustfmt`. Explain what `clippy` lints mean when they fire. Write doc comments (`///`) on all public items.

---

## 3. Tech Stack & Key Crates

| Purpose | Crate | Notes |
|---|---|---|
| CLI argument parsing | `clap` (derive API) | Teach derive macros through this |
| Error handling (app) | `anyhow` | For main application error propagation |
| Error handling (lib) | `thiserror` | For custom error enums in internal modules |
| TOML parsing | `toml` + `serde` | For `enx.toml` / global config |
| Fuzzy matching | `nucleo` or `fuzzy-matcher` | For `enx cd` and `enx env` fuzzy search |
| Colored terminal output | `owo-colors` or `colored` | For pretty CLI output |
| Directory traversal | `walkdir` | If needed for project scanning |
| Path handling | `dirs` | For `~/.config/enx/` cross-platform paths |
| Process spawning | `std::process::Command` | For running tasks, services |
| Async (if needed) | `tokio` | Only introduce if service management demands it |
| Shell completions | `clap_complete` | Generate completions from clap definitions |
| Cross-platform open | `open` | For `enx open repo`, `enx open code` |
| HTTP (for clone) | `std::process::Command` wrapping `git` | Shell out to `git clone` |
| Glob / file watching | `notify` | If watch mode is ever added later |

---

## 4. Project Structure

```
enx/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs              # Entry point: parse CLI args, dispatch commands
│   ├── cli.rs               # Clap CLI definition (all commands/subcommands)
│   ├── config/
│   │   ├── mod.rs            # Re-exports
│   │   ├── project.rs        # Per-project enx.toml schema & parsing
│   │   ├── global.rs         # s       # Project registry (list of known projects + paths)
│   ├── commands/
│   │   ├── mod.rs            # Re-exports
│   │   ├── cd.rs             # `enx cd <name>` — fuzzy find + output path
│   │   ├── up.rs             # `enx up` — bootstrap environment
│   │   ├── down.rs           # `enx down` — tear down environment
│   │   ├── start.rs          # `enx start` — start the project
│   │   ├── doctor.rs         # `enx doctor` — diagnose environment
│   │   ├── projects.rs       # `enx projects` — list registered projects
│   │   ├── add.rs            # `enx add` — register a project
│   │   ├── remove.rs         # `enx remove` — unregister a project
│   │   ├── clone.rs          # `enx clone` — git clone + register
│   │   ├── init.rs           # `enx init` — scaffold enx.toml
│   │   ├── env.rs            # `enx env <name>` — switch environment
│   â── run.rs            # `enx <task>` — custom task execution engine
│   │   ├── completions.rs    # `enx completions <shell>` — generate shell completions
│   │   └── self_update.rs    # `enx self-update`
│   ├── task/
│   │   ├── mod.rs
│   │   ├── runner.rs         # Task execution logic (nesting, args, env injection)
│   │   └── resolver.rs       # Resolve task by name (project -> global fallback)
│   ├── services/
│   │   ├── mod.rs
│   │   └── manager.rs        # Start/stop/status background services
│   ├── fuzzy.rs              # Fuzzy matching utility (shared by cd, env, etc.)
│   ├── shell.rs              # Shell integration helpers (cd hack, env export)
│   └── output.rs             # Colored, formatted terminal output helpers
├──  ...
```

---

## 5. Configuration File Schemas

### 5.1 Per-Project: `enx.toml`

```toml
[project]
name = "my-app"                          # Project display name (used in `enx projects`)

REDIS_URL = "redis://localhost:6379"
APP_SECRET = "dev-secret-key"

# Named environment overrides (switched via `enx env <name>`)
[env.staging]
DATABASE_URL = "postgres://staging-host:5432/myapp_staging"
APP_SECRET = "staging-secret"

[env.production]
DATABASE_URL = "postgres://prod-host:5432/myapp_prod"
APP_SECRET = "prod-secret"

# Dotenv files to auto-load (in order; later files override earlier)
[env.dotfiles]
files = [".env", ".env.local"]

# ──────────────────────────────────────
# Services (background processes managed by enx)
# ──────────────────────────────────────
[[services]]
name = "postgres"
up = "docker run -d --name enx-postgres -p 5432:5432 -e POSTGRES_PASSWORD=dev postgres:16"
down = "docker stop enx-postgres && docker rm enx-postgres"
health = "pg_isready -h localhost -p 5432"             # Command to check if healthy

[[services]]
name = "redis"
up = "docker run -d --name enx-redis -p 6379:6379 redis:7"
down = "docker stop enx-redis && docker rm enx-redis"
health = "redis-cli ping"

# ─────────────────────

# Platform-specific overrides (merged on top of base steps)
[up.linux]
steps = ["sudo apt-get install -y libssl-dev"]

[up.macos]
steps = ["brew install openssl"]

[up.windows]
steps = ["choco install openssl"]

[down]
steps = [
    "docker compose down",
]

[start]
command = "npm run dev"

# ──────────────────────────────────────
# Custom tasks
# ───────────────────────= "npm run test"
description = "Run the test suite"

[tasks."db:migrate"]
command = "npx prisma migrate dev"
description = "Run database migrations"

[tasks."db:seed"]
command = "npx prisma db seed"
description = "Seed the database"

[tasks."db:reset"]
command = "enx run db:migrate && enx run db:seed"      # Task nesting via shell
description = "Reset DB: migrate then seed"

[tasks.deploy]
command = "bash deploy.sh --env $ENX_ENV"
description = "Deploy the application"

# Tasks can accept arguments — anything after `--` is forwarded
# Usage: `e run deploy -- --dry-run`
[tasks.deploy-custom]
command = "bash deploy.sh"
description = "Deploy with custom flags"

# ──────────────────────────────────────
# Open targets
# ──────────────────────────────────────
[open]
repo = "https://github.com/myorg/my-app"               # `enx open repo`
code = "code ."                                          # `enx open code` (default: vscode)
ci = "https://github.com/myorg/my-app/actions"          # `enx open ci`
docs = "https://myapp.readme.io"                         # `enx open docs`
```

### 5.2 Global Config: `~/.config/enx/config.toml`

```toml
# Default editor command (used by `enx open code` if not overridden per-project)
[defaults]
editor = "code ."

# Global tasks available in every project
[tasks.scratch]
command = "code ~/scratch.md"
description = "Open scratch notes"

[tasks."git:prune"]
command = "git f```

### 5.3 Project Registry: `~/.config/enx/registry.toml`

```toml
# Auto-managed by `enx add`, `enx remove`, `enx clone`
[[projects]]
name = "my-app"
path = "/home/user/code/my-app"

[[projects]]
name = "api-service"
path = "/home/user/code/work/api-service"

[[projects]]
name = "dotfiles"
path = "/home/user/dotfiles"
```

---

## 6. Command Reference

| Command | Behavior |
|---|---|
| `enx cd <query>` | Fuzzy-match `<query>` against registered project names. Print the matched project's path to stdout (consumed by a shell function to actually `cd`). If no match or ambiguous, print error to stderr. No args = error. |
| `enx up` | Must be inside a project with `enx.toml`. Start services, load env, run `[up]` steps (with platform overrides merged). |
| `enx down` | Run `[down]` steps, stop managed services. |
| `enx start` | Run the `[start]` command from `enx.toml`. |
| `enx doctor` | Check: enx.toml exists, services healthy, env vars set. Report pass/fail per check. |
| `enx projects` | List all registered projects: name, path, whether path exists on disk. |
| `enx add [path]` | Register `[path]` (default `.`) as a project. Read `enx.toml` for the name, or prompt/infer. |
| `enx remove <name>` | Unregister a project by name (does NOT delete files). |
| `enx clone <repo> [path]` | `git clone <repo> [path]`, then auto-register. Optionally auto-run `enx up`. |
| `enx init` | Scaffold an `enx.toml` in the current directory with commented-out examples. |
| `enx env <query>` | Fuzzy-match `<query>` against environment names defined in `enx.toml`. Switch active env vars. |
| `enx run <task> [-- args...]` | Run a named task. Resolution order: project `enx.toml` → global `config.toml`. Forward extra args. |
| `enx open <target>` | Open a named target from `[open]`. If URL, open in browser. If command, execute it. Defaults: `repo` → browser, `code` → editor. |
| `enx status` | Show current project name, active environment, running services, git branch. |
| `enx completions <shell>` | Print shell completion script for `bash`, `zsh`, `fish`, or `powershell`. |
| `enx self-update` | Download and replace the current binary with the latest release. |
| `enx <anything_else>` | Treated as `enx run <anything_else>`. This allows bare `enx lint`, `enx test`, etc. |

---

## 7. Shell Integration

### The `cd` Problem

A child process cannot change the parent shell's working directory. `enx cd` must work via a **shell function** that calls the binary, captures stdout, and `cd`s to it.

The user must add a shell function to their shell config. `enx init-shell` or `enx completions` should output this.

**Bash/Zsh:**
```bash
enx() {
    if [ "$1" = "cd" ]; then
        shift
        local dir
        dir=$(command enx cd "$@")
        if [ $? -eq 0 ]; then
            cd "$dir" || return 1
        else
            return 1
        fi
    else
        command enx "$@"
    fi
}
```

**PowerShell:**
```powershell
function enx {
    if ($args[0] -eq "cd") {
        $dir = & enx.exe cd @($args[1..($args.Length-1)])
        if ($LASTEXITCODE -eq 0) { Set-Location $dir }
    } else {
        & enx.exe @args
    }
}
```

**Fish:**
```fish
function enx
    if test "$argv[1]" = "cd"
        set dir (command enx cd $argv[2..])
        and cd $dir
    else
        command enx $argv
    end
end
```

---

## 8. Environment Switching Design

When the user runs `enx env staging`:
1. Fuzzy-match "staging" against keys under `[env.*]` in `enx.toml`.
2. Write the active environment name to a local state file (`.enx/state.toml` in the project root, gitignored).
3. When running any task (`enx run`, `enx start`, etc.), the task runner loads base `[env]` vars first, then overlays the active named environment on top.
4. `enx status` shows which environment is active.

---

## 9. Task Execution Engine

### Resolution Order
1. Look up task name in the current project's `enx.toml` `[tasks]`.
2. If not found, look up in `~/.config/enx/config.toml` `[tasks]`.
3. If not found, error.

### Nesting
Task commands can invoke `enx run <other_task>`. Since `enx` is on `$PATH`, this naturally shells out to another `enx` invocation. No special recursive handling needed — the shell does it.

### Argument Forwarding
For `enx run deploy -- --dry-run --verbose`:
- Everything after `--` is appended to the task's `command` string.

### Environment Injection
Every task runs with:
1. The curnt OS environment.
2. `[env]` vars from `enx.toml` overlaid.
3. Active named environment overlaid on top of that.
4. `ENX_PROJECT` set to the project name.
5. `ENX_ENV` set to the active environment name (or "default").

---

## 10. Implementation Phases

Build the project in this order. Each phase should be fully functional and tested before moving on.

### Phase 1 — Skeleton & Config Parsing
**Rust concepts introduced:** project setup, modules, `serde`, `derive` macros, `struct`, `enum`, `Result`, `anyhow`, `String` vs `&str`, file I/O, `Option`.

- `cargo init`, set up project structure.
- Define all TOML config structs with `serde::Deserialize`.
- Parse a sample `enx.toml`, global `config.toml`, and `registry.toml`.
- Write unit tests for config parsing.
- Set up `clap` CLI skeleton with all subcommands (they can just print "not implemented yet").

### Phase 2 — Project Registry & Navigation
**Rust concepts introduced:** `Vec`, iterators, closures, `fuzzy-matcher` / string matching, `PathBuf` vs `Path`, cross-platform path handling, `sfs`, `stdin`/`stdout`/`stderr`.

- Implement `enx add`, `enx remove`, `enx projects`.
- Implement `enx cd <query>` with fuzzy matching.
- Implement `enx clone`.
- Write integration tests.

### Phase 3 — Task Runner
**Rust concepts introduced:** `std::process::Command`, environment variable manipulation, `HashMap`, string interpolation, exit codes, builder pattern.

- Implement `enx run <task>` with resolution order (project → global).
- Implement argument forwarding.
- Implement environmenjection.
- Implement bare `enx <task>` fallback.
- Implement `enx init` (scaffold a template `enx.toml`).

### Phase 4 — Environment Management
**Rust concepts introduced:** nested deserialization with serde, `BTreeMap`, file watching, state persistence.

- Implement `enx env <query>` with fuzzy matching and state persistence.
-mplement `.env` file loading (with override order).
- Wire environment into the task runner.

### Phase 5 — Lifecycle Commands
**Rust concepts introduced:** `Vec<Box<dyn ...>>` (trait objects, if applicable), sequencing, platform detection (`cfg!(target_os)`), conditional compilation.

- Implement `enx up` (service start, step execution, platform overres).
- Implement `enx down`.
- Implement `enx start`.

### Phase 6 — Services
**Rust concepts introduced:** process management, PID files, health checks, `std::thread` or `tokio` for polling, `Duration`, `Instant`.

- Implement service start/stop/health-check.
- Track running services in `.enx/services.json` (PIDs, container names).
- Wire into `enx up`, `enx down`, `enx doctor`.

### Phase 8 — Doctor, Status, Open
**Rust concepts introduced:** trait-based check system, `open` crate, dynamic dispatch.

- Implement `enx doctor` (check services, env vars).
- Int `enx status`.
- Implement `enx open <target>`.

### Phase 9 — Shell Completions & Polish
**Rust concepts introduced:** `clap_complete`, `build.rs` (if generating at compile time), `include_str!`.

- Generate shell completions.
- Shell wrapper function generation (`enx init-shell`).
- Colored output everywhere.
- Comprehensive error messages with suggestions.

### Phase 10 — Self-Update & Distribution
**Rust concepts introduced:** HTTP client (`reqwest`), file replacement, platform detec, `cfg!` macros, GitHub releases API.

- Implement `enx self-update` (download latest release from GitHub).
- Set up CI/CD for cross-platform builds (GitHub Actions).
- Create install script.

---

## 11. Code Style & Conventions

- **Error messages:** Always lowercase, no trailing period. Example: `error: project "foo" not found in registry`.
- **Output prefixes:** Use colored prefixes: `▸ ` (cyan) for info, `✓ ` (green) for success, `✗ ` (red) for errors, `⚠ ` (yellow) for warnings.
- **Naming:** Rust snake_case everywhere. Structs are Pa. Constants are SCREAMING_SNAKE.
- **No `unwrap()` in production code.** Use `?`, `.context("...")`, or explicit match. `unwrap()` is only acceptable in tests.
- **Every public function has a doc comment.**
- **Every module has a top-level doc comment** (`//! This module handles...`).

---

## 12. Example Interactions

```
$ enx projects
  my-app          ~/code/my-app
  api-service     ~/code/work/api-service
  dotfiles        ~/dotfiles

$ enx cd api
  ▸ Matched "api-service" (~/code/work/api-service)
  [shell cd's to that directory]

$ enx up
  ▸ Detected enx.toml for "api-service"
  ▸ Starting services...
    ✓ postgres (healthy)
    ✓ redis (healthy)
  ▸ Running setup steps...
    ✓ npm install (3.2s)
    ✓ enx run db:migrate (1.1s)
  ✓ Environment ready!

$ enx env stag
  ▸ Matched "staging"
  ✓ Switched to staging environment

$ enx status
  Project:     api-service
  Path:        ~/code/: staging
  Branch:      feature/new-auth
  Services:    postgres ✓ | redis ✓

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

$ enx lint
  ▸ Running task "lint"...
  [npm run lint output]

$ enx open repo
  ▸ Openinervice in browser...

$ enx open code
  ▸ Running: code .
```

---

## 13. Testing Strategy

| Type | What | How |
|---|---|---|
| Unit tests | Config parsing, fuzzy matching, env overlay logic, task resolution | `#[cfg(test)] mod tests` in each module |
| Integration tests | Full CLI invocations, `enx add` + `enx projects` routrip, `enx run` | `tests/` directory, `assert_cmd` crate, temp directories (`tempfile` crate) |
| Snapshot tests | CLI output formatting (help text, project list, doctor output) | `insta` crate (optional but recommended) |

---

## 14. Open Design Questions (Decide During Implementation)

1. **Service process tracking:** Docker container names? PID files? A local `.enx/state.toml`?
2. **`enx cd` fuzzy threshold:** How fuzzy is too fuzzy? Should it require confirmation if the match score is low?
3. **`enx up` idempotency:** Should `enx up` be safe to run repeatedly? (Yes — design for it.)
4. **Self-update source:** GitHub Releases? A custom server? Cargo install?
