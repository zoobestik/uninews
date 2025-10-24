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

## Configuration file

A configuration file is not supported yet. Do not use `UNINEWS_CONFIG_PATH` for now.

All configuration is done via CLI commands and environment variables.
