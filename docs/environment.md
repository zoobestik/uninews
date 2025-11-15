# Environment

This page lists environment variables that affect UniNews.

- Logging uses environment filters (from `tracing-subscriber`). The common variable is `RUST_LOG`.
- The database path is controlled by `UNINEWS_DB_PATH`.

## Variables

- `UNINEWS_DB_PATH` — Path to the SQLite database file.
  - Default: `data/app.sqlite`
  - Example:
    ```bash
    UNINEWS_DB_PATH=/tmp/uninews.sqlite uninews init --force
    ```

- `RUST_LOG` — Set the log level and filters for the CLI.
  - Default: `info`
  - Examples:
    ```bash
    RUST_LOG=debug uninews collect
    RUST_LOG=uninews=debug,sqlx=warn uninews collect
    ```

- `DEBUG` — Alternative way to enable verbose output and debug mode.
  - Default: unset
  - When set (to any value), enables verbose logging and output
  - Example:
    ```bash
    DEBUG=1 uninews collect
    ```
  - Note: `RUST_LOG` takes precedence if both are set

- `NO_COLOR` — Disable colorized output (follows [NO_COLOR standard](https://no-color.org/)).
  - Default: unset
  - When set (to any value), disables all color output
  - Example:
    ```bash
    NO_COLOR=1 uninews collect
    ```
  - Automatically disabled when output is redirected or terminal doesn't support colors

## Configuration file

A configuration file is not supported yet. Do not use `UNINEWS_CONFIG_PATH` for now.

All configuration is done via CLI commands and environment variables.
