# Getting Started

This guide helps you install UniNews, initialize the database, add sources, and run your first collection.

- If you only need to install instructions, see [Installation](./installation.md).
- For command details, see the [CLI reference](./cli.md).

## Requirements

- Rust toolchain: stable (see [`rust-toolchain.toml`](../rust-toolchain.toml))
- SQLite is bundled via the `sqlx` crate; no manual setup is usually needed.

## Install and build

Install the release binary once as described in [Installation](./installation.md). All examples below assume `uninews` is available in your PATH.

## Initialize the app

Initialization creates the SQLite database and required folders.

```bash
uninews init
```

- If the database already exists, the CLI asks before overwriting it.
- Use `--force` to overwrite without confirmation:

```bash
uninews init --force
```

## Add sources

Add an Atom/RSS feed:

```bash
uninews source add atom https://example.com/feed.xml
# ort: uninews src add rss https://example.com/feed.xml
```

Add a Telegram channel (use the channel name without `@`):

```bash
uninews source add telegram telegram
```

List sources:

```bash
uninews source ls
```

Remove sources:

```bash
uninews source rm atom https://example.com/feed.xml
uninews source rm telegram telegram
```

See more: [Sources](./sources.md).

## Collect content

Collect content at once from all configured sources:

```bash
uninews collect
```

Note about `--watch`:

- The `--watch` flag is planned for continuous collection.
- Today it is experimental and may not change behavior.

## Paths and environment

- Default database path: `data/app.sqlite`.
- You can change the database file path using `UNINEWS_DB_PATH`.
- Logging level is controlled by the environment filter (e.g., `RUST_LOG=debug`).

Examples:

```bash
# Use a custom database location
UNINEWS_DB_PATH=/tmp/uninews.sqlite uninews init --force

# Enable debug logs
RUST_LOG=debug uninews collect
```

Migrations are applied automatically during `uninews init` from the `migrations/` folder.

Backup tip: to make a safe copy, stop UniNews, and copy the database file.

## Troubleshooting

If something goes wrong, see [Troubleshooting](./troubleshooting.md).
