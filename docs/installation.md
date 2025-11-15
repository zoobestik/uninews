# Installation

This page explains how to install or build UniNews.

UniNews is a Rust CLI. Recommended usage is to build a release binary once and then run the `uninews` binary directly in all commands throughout the docs.

## Prerequisites

- Rust toolchain: stable (see [`rust-toolchain.toml`](../rust-toolchain.toml))
- A supported OS: Linux, macOS, or Windows (x86_64/ARM64; tested mainly on macOS/Linux).

## Recommended — Build a release binary

```bash
cargo build --release
# The binary will be here:
./target/release/uninews --help
```

Copy it to a folder in your PATH (example shown for Unix-like systems):

```bash
sudo cp ./target/release/uninews /usr/local/bin/
```

After this step, all commands in the docs assume `uninews` is on your PATH.

## Alternative — Run with Cargo (no install)

You can also run UniNews without installing a system-wide binary (useful for quick tests):

```bash
# From the project root
cargo run -p news_cli -- --help
```

Use it the same way for any command by replacing `uninews` with `cargo run -p news_cli --`.

## Docker

UniNews provides official Docker images based on Alpine Linux for minimal footprint and security.

### Using pre-built images

Pull the latest image from Docker Hub:

```bash
docker pull zoobestik/uninews:latest
```

Run with a volume mount for persistent data:

```bash
docker run --rm \
  -v $(pwd)/data:/app/data \
  -e RUST_LOG=debug \
  zoobestik/uninews:latest
```

**Note:** The default command is `uninews collect`. To run other commands, override the CMD:

```bash
# Initialize database
docker run --rm -v $(pwd)/data:/app/data zoobestik/uninews:latest uninews init --force

# List sources
docker run --rm -v $(pwd)/data:/app/data zoobestik/uninews:latest uninews source ls

# Add a source
docker run --rm -v $(pwd)/data:/app/data zoobestik/uninews:latest \
  uninews source add atom https://example.com/feed.xml
```

### Using docker-compose

A `docker-compose.yml` file is provided in the repository root:

```yaml
services:
  uninews:
    image: zoobestik/uninews:latest
    restart: unless-stopped
    volumes:
      - ./data:/app/data
```

### Environment variables in Docker

The Docker image respects the following environment variables:

- `RUST_LOG` — Log level (default: `info`)
- `DATABASE_URL` — Database path (default: `sqlite:///app/data/app.sqlite`)
- `UNINEWS_DB_PATH` — Alternative way to specify database path

Example with custom configuration:

```bash
docker run --rm \
  -v $(pwd)/data:/app/data \
  -e RUST_LOG=debug \
  -e UNINEWS_DB_PATH=/app/data/custom.sqlite \
  zoobestik/uninews:latest
```

### Security considerations

- The container runs as a non-root user (`uninews:uninews`, UID/GID 1000)
- The data directory `/app/data` is owned by the `uninews` user
- Volume permissions: ensure your host directory is readable/writable by UID 1000

## Environment filters (logging)

UniNews uses environment-based logging filters. The default level is `info`.

```bash
# Enable debug logs
RUST_LOG=debug uninews collect
```

## Configuration file

A configuration file is not supported yet. All configuration is done via CLI commands and environment variables. See [Environment](./environment.md).

## See Also

- [Docker Deployment](./docker.md) — Detailed Docker documentation
- [Getting Started](./getting-started.md) — Quick start guide
- [Environment](./environment.md) — Environment variables reference
