---
layout: default
title: enx open
parent: Commands
nav_order: 12
---

# enx open

{: .no_toc }

Open a URL or command target.
{: .fs-6 .fw-300 }

---

## Usage

```
enx open <target>
```

## Arguments

| Argument | Required | Description                                        |
| :------- | :------- | :------------------------------------------------- |
| `target` | Yes      | Name of the target to open, as defined in `[open]` |

## Description

Opens a named target from the `[open]` section of `enx.toml`. Targets must be URLs — they are opened in the system's default browser.

## Example

```toml
# enx.toml
[open]
repo = "https://github.com/myorg/my-app"
ci = "https://github.com/myorg/my-app/actions"
docs = "https://myapp.readme.io"
```

```
$ enx open repo
  ▸ Opening in browser...
```

## See Also

- [Configuration: `[open]`]({{ site.baseurl }}/configuration/project#open) — defining open targets
