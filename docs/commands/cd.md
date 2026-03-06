---
layout: default
title: enx cd
parent: Commands
nav_order: 1
---

# enx cd

{: .no_toc }

Navigate to a registered project directory by name.
{: .fs-6 .fw-300 }

## Table of Contents

{: .no_toc .text-delta }

1. TOC
   {:toc}

---

## Usage

```
enx cd <name>
```

## Arguments

| Argument | Required | Description                                          |
| :------- | :------- | :--------------------------------------------------- |
| `name`   | Yes      | Name (or partial name) of the project to navigate to |

## Description

`enx cd` fuzzy-matches `<name>` against all registered project names. If a single match is found, enx prints the project's path to stdout and the shell wrapper function changes to that directory.

If multiple projects match, an interactive fuzzy-select menu is displayed so you can pick the right one.

{: .important }

> `enx cd` requires **shell integration** to actually change your working directory. A child process cannot change the parent shell's directory — the shell wrapper function that `enx setup` installs handles this. See [Shell Integration]({{ site.baseurl }}/shell-integration).

## Examples

```
$ enx cd api
  ▸ Matched "api-service" (~/code/work/api-service)

$ enx cd my
  ▸ Matched "my-app" (~/code/my-app)
```

If the match is ambiguous, you'll be presented with a selector:

```
$ enx cd a
? Select a project:
> api-service
  my-app
```

## See Also

- [`enx projects`]({{ site.baseurl }}/commands/projects) — list all registered projects
- [`enx init`]({{ site.baseurl }}/commands/init) — register a new project
- [Shell Integration]({{ site.baseurl }}/shell-integration) — required for `cd` to work
