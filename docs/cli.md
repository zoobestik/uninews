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

- `-w`, `--watch` — **Experimental:** Continuously monitor sources for updates. This flag enables long-running mode where UniNews periodically checks sources for new content instead of exiting after one collection cycle.

### Watch mode details

When `--watch` flag is enabled:

- UniNews runs in a continuous loop, checking sources periodically
- Default check interval: 60 seconds (configurable in code)
- Automatic exponential backoff on errors (up to 10 minutes)
- Press Ctrl+C to stop gracefully

**Warning:** Watch mode is experimental and behavior may change in future versions.

Example usage:

```bash
# Run continuous collection
RUST_LOG=info uninews collect --watch
```

Typical errors:

- No sources configured yet.
- Network errors when fetching feeds or channels.

Use debug logs to diagnose problems:

```bash
RUST_LOG=debug uninews collect
```

## Output and Reporting

Colors are automatically disabled when:
- Output is redirected to a file
- Terminal does not support colors (e.g., basic CI environments)
- `NO_COLOR` environment variable is set

### Verbosity control

Verbose mode is enabled when:
- `RUST_LOG` environment variable is set
- `DEBUG` environment variable is set

Example:

```bash
# Verbose output with debug logs
RUST_LOG=debug uninews collect

# Or use DEBUG flag
DEBUG=1 uninews collect
```

## See Also

- [Docker Deployment](./docker.md) — Running with Docker
- [Environment](./environment.md) — Environment variables reference
- [Troubleshooting](./troubleshooting.md) — Common issues and solutions
