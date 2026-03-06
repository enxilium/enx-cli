---
layout: default
title: enx env
parent: Commands
nav_order: 7
---

# enx env

{: .no_toc }

Switch between environment configurations.
{: .fs-6 .fw-300 }

## Table of Contents

{: .no_toc .text-delta }

1. TOC
   {:toc}

---

## Usage

```
enx env [query]
```

## Arguments

| Argument | Required | Description                                                                                                     |
| :------- | :------- | :-------------------------------------------------------------------------------------------------------------- |
| `query`  | No       | Name (or partial name) of the environment to switch to. If omitted, lists all environments or shows a selector. |

## Description

Environments are defined in the `[env]` section of `enx.toml`. Each environment maps a name to a dotenv file containing key-value pairs.

When you switch environments, enx records the active environment so that subsequent commands (`enx run`, `enx start`, etc.) load the correct environment variables.

### Without arguments

If no query is provided, enx displays an interactive fuzzy-select menu of all available environments, or lists them if only a few are defined.

### With a query

Fuzzy-matches the query against environment names. If a single match is found, switches to it immediately. If ambiguous, shows a selector.

## Examples

```
$ enx env staging
  ▸ Matched "staging"
  ✓ Switched to staging environment
```

```
$ enx env
  Available environments:
    ├─ development    .env.development
    ├─ staging        .env.staging
    └─ production     .env.production
```

## See Also

- [Configuration: `[env]`]({{ site.baseurl }}/configuration/project#env) — defining environments
- [`enx status`]({{ site.baseurl }}/commands/status) — see which environment is active
