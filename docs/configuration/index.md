---
layout: default
title: Configuration
nav_order: 5
has_children: true
---

# Configuration Reference

{: .no_toc }

enx uses three TOML configuration files to manage projects, environments, and tasks.
{: .fs-6 .fw-300 }

---

## Config Files

| File                                                         | Scope       | Location                      |
| :----------------------------------------------------------- | :---------- | :---------------------------- |
| [`enx.toml`]({{ site.baseurl }}/configuration/project)       | Per-project | Project root directory        |
| [`config.toml`]({{ site.baseurl }}/configuration/global)     | Global      | `~/.config/enx/config.toml`   |
| [`registry.toml`]({{ site.baseurl }}/configuration/registry) | Global      | `~/.config/enx/registry.toml` |

- **`enx.toml`** lives in each project directory and defines everything specific to that project: name, environments, lifecycle steps, tasks, and open targets.
- **`config.toml`** is your global configuration — default settings and tasks available from any project.
- **`registry.toml`** is auto-managed by enx — it tracks which projects you've registered and their paths on disk.
