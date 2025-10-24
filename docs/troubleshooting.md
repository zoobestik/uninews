# Troubleshooting

This page lists common issues and how to fix them.

If your issue is not here, run with debug logs and collect more details:

```bash
RUST_LOG=debug uninews <command>
```

## Database file cannot be created

Symptoms:

- Errors about filesystem permissions
- Errors about missing directories

Fix:

- Ensure the parent directory exists and is writable.
- Re-run initialization; it creates parent directories when needed:
  ```bash
  uninews init --force
  ```
- If you use a custom path, verify it:
  ```bash
  echo $UNINEWS_DB_PATH
  ls -ld "$(dirname "$UNINEWS_DB_PATH")"
  ```

## Duplicate source errors

Symptoms:

- You try to add a feed or channel that already exists.

Fix:

- List sources and check duplicates:
  ```bash
  uninews source list
  ```
- Remove the unwanted entry:
  ```bash
  uninews source remove atom https://example.com/feed.xml
  # or
  uninews source remove telegram telegram
  ```

## Network or source not reachable

Symptoms:

- Timeouts or HTTP errors during `uninews collect`.

Fix:

- Check your network connection.
- Open the feed URL in a browser to confirm it is valid.
- Try again later; the source may be temporarily down.

## Where is the database?

- Default location: `data/app.sqlite`
- You can override it with `UNINEWS_DB_PATH`

See also: [Database](./database.md), [Environment](./environment.md).
