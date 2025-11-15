# Docker Deployment

This page provides comprehensive information about deploying UniNews using Docker.

## Overview

UniNews provides official Docker images optimized for production use:

- **Base image:** Alpine Linux (minimal footprint)
- **Image size:** ~50MB (compressed)
- **Build type:** Multi-stage build with static linking
- **Security:** Non-root user execution
- **Registry:** Docker Hub (`zoobestik/uninews`)

## Quick Start

```bash
# Pull the image
docker pull zoobestik/uninews:latest

# Initialize database
docker run --rm -v $(pwd)/data:/app/data zoobestik/uninews:latest uninews init --force

# Add sources
docker run --rm -v $(pwd)/data:/app/data zoobestik/uninews:latest \
  uninews source add atom https://example.com/feed.xml

# Collect content
docker run --rm -v $(pwd)/data:/app/data zoobestik/uninews:latest
```

## Image Details

### Available Tags

- `latest` — Latest stable build from main branch
- `vX.Y.Z` — Specific version tags (when released)
- `edge` — Development builds from main branch (unstable)

### Exposed Volumes

- `/app/data` — Database and persistent data directory

### Environment Variables

See [Environment Variables](#environment-variables) section below.

### Default Command

The default CMD is `uninews collect`, which performs a one-time collection from all configured sources.

## Using docker-compose

### Basic Configuration

Create a `docker-compose.yml` file:

```yaml
services:
  uninews:
    image: zoobestik/uninews:latest
    container_name: uninews
    restart: unless-stopped
    volumes:
      - ./data:/app/data
```

### With Custom Database Path

```yaml
services:
  uninews:
    image: zoobestik/uninews:latest
    restart: unless-stopped
    volumes:
      - ./data:/app/data
      - ./custom-db:/custom
    environment:
      - RUST_LOG=info
      - UNINEWS_DB_PATH=/custom/uninews.sqlite
```

## Building from Source

### Standard Build

```bash
docker build -t uninews:local .
```

### Multi-Platform Build

Use the provided script for building images for multiple architectures:

```bash
./scripts/docker-image.sh
```

This script builds for:
- `linux/amd64` (x86_64)
- `linux/arm64` (ARM64)

### Build Arguments

The Dockerfile does not currently accept build arguments, but you can customize the build by modifying the Dockerfile.

## Volume Management

### Data Persistence

The container stores all data in `/app/data`. **Always mount a volume** to this path to persist data between container restarts:

```bash
docker run -v $(pwd)/data:/app/data zoobestik/uninews:latest
```

### Permissions

The container runs as UID/GID 1000. Ensure your host directory has appropriate permissions:

```bash
# Create directory with correct ownership
mkdir -p data
sudo chown 1000:1000 data

# Or use your current user (if UID matches)
mkdir -p data
```

If you encounter permission errors:

```bash
# Fix ownership
sudo chown -R 1000:1000 data/

# Or run with user override (not recommended for production)
docker run --user $(id -u):$(id -g) -v $(pwd)/data:/app/data zoobestik/uninews:latest
```

### Backup

To backup your data:

```bash
# Stop container
docker-compose down

# Copy data directory
cp -r data backups/data-$(date +%F)

# Or backup just the database
cp data/app.sqlite backups/app-$(date +%F).sqlite
```

## Networking

UniNews does not expose any network ports. It makes outbound HTTP/HTTPS requests to fetch content from sources.

If running in a restricted network environment, ensure the container can access:
- Feed URLs (Atom/RSS)
- `https://t.me/s/*` (Telegram channels)

## Security Best Practices

1. **Non-root execution:** The image runs as a non-root user by default
2. **Read-only root filesystem:** Consider adding `--read-only` flag with tmpfs mounts:
   ```bash
   docker run --read-only --tmpfs /tmp -v $(pwd)/data:/app/data zoobestik/uninews:latest
   ```
3. **Resource limits:** Set memory and CPU limits in production:
   ```yaml
   services:
     uninews:
       image: zoobestik/uninews:latest
       deploy:
         resources:
           limits:
             memory: 256M
             cpus: '0.5'
   ```
4. **Network isolation:** Use Docker networks to isolate containers
5. **Secrets management:** Do not pass sensitive data via environment variables; use Docker secrets or mounted files

## Troubleshooting

### Permission Denied Errors

**Problem:** Cannot write to `/app/data`

**Solution:**
```bash
# Check host directory permissions
ls -la data/

# Fix ownership
sudo chown -R 1000:1000 data/
```

### Database Locked

**Problem:** `database is locked` error

**Solution:**
- Ensure only one container accesses the database at a time
- Stop all running containers before starting a new one
- Check for zombie processes: `docker ps -a`

### Cannot Pull Image

**Problem:** `Error response from daemon: pull access denied`

**Solution:**
- Verify image name: `zoobestik/uninews:latest`
- Check Docker Hub status
- Try explicit pull: `docker pull zoobestik/uninews:latest`

### Container Exits Immediately

**Problem:** Container stops right after starting

**Solution:**
```bash
# Check logs
docker logs <container-id>

# Run interactively to debug
docker run -it --rm -v $(pwd)/data:/app/data zoobestik/uninews:latest sh

# Check if database is initialized
docker run --rm -v $(pwd)/data:/app/data zoobestik/uninews:latest uninews init --force
```

## Advanced Usage

### Cron-like Scheduling

Use a cron service or systemd timer to run periodic collections:

```yaml
services:
  uninews-cron:
    image: zoobestik/uninews:latest
    volumes:
      - ./data:/app/data
    entrypoint: /bin/sh
    command: >
      -c "while true; do
        uninews collect;
        sleep 3600;
      done"
```

Or use an external cron job:

```cron
0 * * * * docker run --rm -v /path/to/data:/app/data zoobestik/uninews:latest uninews collect
```

## See Also

- [Installation](./installation.md) — General installation instructions
- [Getting Started](./getting-started.md) — First steps with UniNews
- [Environment](./environment.md) — Environment variables reference
- [Troubleshooting](./troubleshooting.md) — Common issues and solutions
