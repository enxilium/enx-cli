---
layout: default
title: Commands
nav_order: 4
has_children: true
---

# Command Reference

{: .no_toc }

enx provides commands for project navigation, environment management, task running, and more.
{: .fs-6 .fw-300 }

---

## Overview

| Command                                                | Description                             |
| :----------------------------------------------------- | :-------------------------------------- |
| [`enx cd`]({{ site.baseurl }}/commands/cd)             | Navigate to a project directory by name |
| [`enx up`]({{ site.baseurl }}/commands/up)             | Bootstrap the project environment       |
| [`enx down`]({{ site.baseurl }}/commands/down)         | Tear down the project environment       |
| [`enx start`]({{ site.baseurl }}/commands/start)       | Start the project                       |
| [`enx run`]({{ site.baseurl }}/commands/run)           | Run a custom task                       |
| [`enx env`]({{ site.baseurl }}/commands/env)           | Switch environment configuration        |
| [`enx init`]({{ site.baseurl }}/commands/init)         | Initialize a new project                |
| [`enx clone`]({{ site.baseurl }}/commands/clone)       | Clone a repo and register it            |
| [`enx remove`]({{ site.baseurl }}/commands/remove)     | Remove a project from the registry      |
| [`enx projects`]({{ site.baseurl }}/commands/projects) | List registered projects                |
| [`enx open`]({{ site.baseurl }}/commands/open)         | Open a URL or command target            |
| [`enx setup`]({{ site.baseurl }}/commands/setup)       | Initial enx setup and shell integration |

{: .note }

> Any unrecognized subcommand is treated as `enx run <subcommand>`. For example, `enx lint` is equivalent to `enx run lint`.
