# Database

UniNews stores data in a local SQLite database.

- Default path: `data/app.sqlite`
- Custom path: set `UNINEWS_DB_PATH` to a full or relative file path
- Connection string format (internal): `sqlite:<path>?mode=rwc`

## Initialize and re-initialize

Run migrations and create the database file:

```bash
uninews init
```

To overwrite an existing database without a prompt:

```bash
uninews init --force
```

What `init` does:

- Creates parent folders if needed
- Connects to the database file
- Applies SQL migrations from the `migrations/` folder

## Backups

To make a safe backup:

1. Stop any running UniNews process.
2. Copy the SQLite file to a secure location.

Example:

```bash
cp data/app.sqlite backups/app-$(date +%F).sqlite
```

## Changing the location

Use `UNINEWS_DB_PATH` to move the database file:

```bash
UNINEWS_DB_PATH=/tmp/uninews.sqlite uninews init --force
```

In Docker environments, the path should point to a location within a mounted volume:

```bash
docker run --rm \
  -v $(pwd)/data:/app/data \
  -e UNINEWS_DB_PATH=/app/data/custom.sqlite \
  zoobestik/uninews:latest uninews init --force
```

See also: [Docker Deployment](./docker.md).

See also: [Environment](./environment.md).
