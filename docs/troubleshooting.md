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

## Docker Issues

### Permission Denied when accessing database

**Symptom:**
```
Error: Failed to initialize database
  → Permission denied (os error 13)
```

**Cause:** Docker container runs as UID 1000, but host directory has different ownership.

**Solution:**
```bash
# Fix directory ownership
sudo chown -R 1000:1000 data/

# Or create directory with correct permissions before first run
mkdir -p data && sudo chown 1000:1000 data
```

### Database is Locked

**Symptom:**
```
Error: database is locked
```

**Cause:** Multiple containers trying to access the same SQLite database.

**Solution:**
```bash
# Stop all running containers
docker ps | grep uninews | awk '{print $1}' | xargs docker stop

# Remove stopped containers
docker ps -a | grep uninews | awk '{print $1}' | xargs docker rm

# Start single container
docker-compose up -d
```

### Container Exits Immediately

**Symptom:** Container stops right after starting (when using `docker-compose up -d`)

**Cause:** Database not initialized, or no sources configured.

**Solution:**
```bash
# Check logs
docker-compose logs uninews

# Initialize database
docker-compose run --rm uninews uninews init --force

# Add at least one source
docker-compose run --rm uninews uninews source add atom https://example.com/feed.xml

# Restart service
docker-compose up -d
```

## Output and Display Issues

### Colors Not Working in Terminal

**Symptom:** Output shows ANSI escape codes like `\033[32m` instead of colors.

**Cause:** Terminal doesn't support ANSI colors.

**Solution:**
```bash
# Disable colors explicitly
NO_COLOR=1 uninews collect

# Or update your terminal emulator to one that supports colors
```

### Colors Showing in Log Files

**Symptom:** Log files contain ANSI escape codes making them hard to read.

**Solution:**
```bash
# Disable colors when redirecting
NO_COLOR=1 uninews collect > output.log 2>&1

# Or strip colors from existing log
sed 's/\x1b\[[0-9;]*m//g' output.log > clean.log
```
