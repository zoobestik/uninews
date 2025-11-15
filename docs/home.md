# UniNews Documentation

Welcome to UniNews, a small command-line tool to collect content from different sources into one place. It focuses on Atom/RSS feeds and Telegram channels.

This documentation helps you install, initialize, and use UniNews. It also explains data paths, environment variables, and common problems.

- Start here: Getting Started → [docs/getting-started.md](./getting-started.md)
- Install and build → [docs/installation.md](./installation.md)
- Command reference → [docs/cli.md](./cli.md)
- Sources (Atom, Telegram) → [docs/sources.md](./sources.md)
- Database and migrations → [docs/database.md](./database.md)
- Docker deployment → [docs/docker.md](./docker.md)
- Environment variables → [docs/environment.md](./environment.md)
- Troubleshooting → [docs/troubleshooting.md](./troubleshooting.md)

## What is UniNews?

UniNews is a self-hosted CLI application. It lets you store a list of content sources, fetch their updates, and aggregate results in a local SQLite database.

- Binary name: `uninews`
- Default database: `data/app.sqlite`

## Quick links

- Initialize the app: `uninews init`
- Add an Atom feed: `uninews source add atom https://example.com/feed.xml`
- Add a Telegram channel: `uninews source add telegram my_channel`
- List sources: `uninews source ls`
- Collect content: `uninews collect`
- Run with Docker: `docker run -v $(pwd)/data:/app/data zoobestik/uninews:latest`

## FAQ

- Where is the database stored?
  - By default in `data/app.sqlite`. You can change it with the `UNINEWS_DB_PATH` environment variable. See [Database](./database.md).

- Can I run UniNews as a background service?
  - Not yet. You can run it via your OS service manager (systemd, launchd, etc.), but there is no official guide at the moment.

- What is `--watch` in `uninews collect`?
  - `--watch` is planned for continuous collection. Today it is experimental and may not change behavior. Use one-time collection for stable runs. See [Collect](./cli.md#collect).

- Can I run UniNews in Docker?
  - Yes. Official Docker images are available on Docker Hub. See [Installation → Docker](./installation.md#docker) or [Docker Deployment](./docker.md) for details.

- How do I disable colored output?
  - Set the `NO_COLOR=1` environment variable, or redirect output to a file. Colors are automatically disabled when the terminal doesn't support them. See [CLI → Output and Reporting](./cli.md#output-and-reporting).
