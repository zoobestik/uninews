# Command-line reference (CLI)

This page describes the UniNews CLI commands, options, and examples.

- For a quick walk-through, see [Getting Started](./getting-started.md).
- For source types, see [Sources](./sources.md).

## Overview

Binary: `uninews`

Top-level commands and common aliases:

- `uninews collect` — Collect content from saved sources (alias: `cl`).
- `uninews init` — Initialize the database and required folders.
- `uninews source` — Manage sources (alias: `src`). 
  - `uninews source list` — Show all sources (alias: `ls`). 
  - `uninews source add atom` — Add an Atom/RSS feed (alias: `rss`). 
  - `uninews source add telegram` — Add a Telegram channel (alias: `tg`). 
  - `uninews source remove atom` — Remove an Atom/RSS feed (alias: `rss`). 
  - `uninews source remove telegram` — Remove a Telegram channel (alias: `tg`). 

Help is available everywhere:

```bash
uninews --help
uninews source --help
uninews source add --help
```

Logging and exit codes:

- Logs use environment filters. Default level is `info`. Adjust with `RUST_LOG=debug`.
- Commands return a non-zero exit code on errors.

## init

Initialize or re-initialize the local SQLite database.

```bash
uninews init
```

Options:

- `-f`, `--force` — overwrite an existing database without asking.

Behavior:

- Creates parent folders if needed (for example `data/`).
- Applies all SQL migrations from the `migrations/` folder.
- If a database already exists and `--force` is not set, the CLI asks for confirmation.

See also: [Database](./database.md).

## source

Manage information sources.

List all sources:

```bash
uninews source ls
```

Add an Atom/RSS feed:

```bash
uninews source add atom https://example.com/feed.xml
# or: uninews src add rss https://example.com/feed.xml
```

Add a Telegram channel (without `@`):

```bash
uninews source add telegram telegram
# or: uninews src add tg telegram
```

Remove an Atom/RSS feed:

```bash
uninews source rm atom https://example.com/feed.xml
```

Remove a Telegram channel:

```bash
uninews source rm telegram telegram
```

Notes:

- The CLI validates URLs and Telegram usernames.
- Duplicate sources are rejected.
- Short aliases exist: `src`, `ls`, `tg`, `rss`, and `rm` for `remove`.

See also: [Sources](./sources.md).

## collect

Collect content from all configured sources.

```bash
uninews collect
```

Options:

- `-w`, `--watch` — Planned: continuously watch sources for updates.

Current status of `--watch`:

- The flag is experimental and not fully implemented yet.
- Today, `collect` runs a one-time collection and returns.

Typical errors:

- No sources configured yet.
- Network errors when fetching feeds or channels.

Use debug logs to diagnose problems:

```bash
RUST_LOG=debug uninews collect
```
