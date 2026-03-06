---
layout: default
title: enx start
parent: Commands
nav_order: 4
---

# enx start

{: .no_toc }

Start the current project.
{: .fs-6 .fw-300 }

---

## Usage

```
enx start
```

## Description

Runs the command(s) defined in the `[start]` section of the project's `enx.toml`. Typically used to start a development server or the main application process.

## Example

```toml
# enx.toml
[start]
commands = ["npm run dev"]
```

```
$ enx start
  ▸ Starting "my-app"
  [npm run dev output streams here]
```

## See Also

- [`enx stop`]({{ site.baseurl }}/commands/stop) — stop the project
- [Configuration: `[start]`]({{ site.baseurl }}/configuration/project#start) — full schema
