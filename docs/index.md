---
layout: default
title: Home
nav_order: 1
permalink: /
---

# enx

{: .fs-9 }

Your developer workflow, one command away.
{: .fs-6 .fw-300 }

enx is a cross-platform CLI that manages project navigation, environment bootstrapping, task running, and service management — so you can focus on writing code.

[Get Started]({{ site.baseurl }}/quick-start){: .btn .btn-primary .fs-5 .mb-4 .mb-md-0 .mr-2 }
[View on GitHub](https://github.com/enxilium/enx-cli){: .btn .fs-5 .mb-4 .mb-md-0 }

[See all my projects →](https://jacemu.xyz){: .fs-4 .fw-300 }

---

<div class="feature-grid" markdown="0">
  <div class="feature-card">
    <h3>🧭 Project Navigation</h3>
    <p>Fuzzy-find any registered project and <code>cd</code> to it instantly with <code>enx cd</code>.</p>
  </div>
  <div class="feature-card">
    <h3>⚙️ Environment Management</h3>
    <p>Switch between development, staging, and production configs with <code>enx env</code>.</p>
  </div>
  <div class="feature-card">
    <h3>🚀 Task Runner</h3>
    <p>Define per-project or global tasks and run them with <code>enx run</code> — or just <code>enx &lt;task&gt;</code>.</p>
  </div>
  <div class="feature-card">
    <h3>🔓 Open Targets</h3>
    <p>Open repo URLs, CI dashboards, or your editor with <code>enx open</code>.</p>
  </div>
</div>

---

## Quick Install

```sh
curl -fsSL https://raw.githubusercontent.com/enxilium/enx-cli/main/scripts/install.sh -o install.sh
sh install.sh
enx setup
```

On Windows, run this from Git Bash, MSYS2, Cygwin, or WSL (bash/zsh environments).

The installer downloads the latest nightly binary. Then run `enx setup` in your current shell.

---

## What Can It Do?

| Command               | Description                                  |
| :-------------------- | :------------------------------------------- |
| `enx cd <query>`      | Fuzzy-find and navigate to a project         |
| `enx up` / `enx down` | Bootstrap or tear down a project environment |
| `enx start`           | Start the project                            |
| `enx run <task>`      | Run a custom task (project or global)        |
| `enx env <name>`      | Switch environment configuration             |
| `enx init`            | Scaffold a new `enx.toml`                    |
| `enx clone <repo>`    | Clone a repo and register it                 |
| `enx projects`        | List all registered projects                 |
| `enx open <target>`   | Open a URL or command target                 |

See the full [Command Reference]({{ site.baseurl }}/commands/) for details.
