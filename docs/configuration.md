# Configuration

## Overview

The application reads its configuration from a TOML file. You can specify a custom config file path using the `UNINEWS_CONFIG_PATH` environment variable, or the application will use `./config.toml` in the current working directory by default.

## Quick Start

Create a `config.toml` file:
```toml
[[atom]]
source_url = "https://example.com/feed.xml"

[[telegram]]
nickname = "zoobestik"
```
This minimal configuration defines one Atom/RSS feed and one Telegram channel.

## Configuration File Location

The application looks for configuration in this order:

1. **Custom path**: If `UNINEWS_CONFIG_PATH` environment variable is set, use that file
2. **Default path**: Otherwise, use `./config.toml` in the current working directory

**Note:** The application automatically loads variables from a `.env` file in the current directory if present.

## Configuration Reference

> **Important:** The TOML config does not allow unknown fields. Misspelled or extra keys will cause an error.

### Atom/RSS Feeds: `[[atom]]`

Define RSS or Atom feeds to monitor. You can add multiple `[[atom]]` sections.

| Property     | Type   | Required | Description                                                 |
|--------------|--------|----------|-------------------------------------------------------------|
| `source_url` | string | **Yes**  | Full URL of the feed (e.g., `https://example.com/feed.xml`) |

**Example:**
```toml
[[atom]]
source_url = "https://blog.example.com/rss"

[[atom]]
source_url = "https://news.example.org/atom.xml"
```

### Telegram Channels: `[[telegram]]`

Define Telegram channels to monitor. You can add multiple `[[telegram]]` sections.

| Property   | Type   | Required | Description                                                                                                   |
|------------|--------|----------|---------------------------------------------------------------------------------------------------------------|
| `nickname` | string | **Yes**  | Channel or user nickname without the `@` symbol. Must be 5–32 characters (letters, numbers, underscores only) |

The application constructs the Telegram URL as `https://t.me/<nickname>`.

**Example:**
```toml
[[telegram]]
nickname = "example_channel"

[[telegram]]
nickname = "another_news_source"
```

## Environment Variables

### `UNINEWS_CONFIG_PATH`

Optional. Specifies the path to your TOML configuration file.

**Using a `.env` file** (recommended for local development):

Create a `.env` file in your working directory:
```bash
UNINEWS_CONFIG_PATH=./custom_config.toml
```
**Using shell environment:**

```bash
# Set for the current session
export UNINEWS_CONFIG_PATH=/absolute/path/to/config.toml
cargo collect

# Set for a single command
UNINEWS_CONFIG_PATH=./config.toml cargo collect
```
## Complete Example

```toml
# Multiple Atom/RSS feeds
[[atom]]
source_url = "https://blog.rust-lang.org/feed.xml"

[[atom]]
source_url = "https://this-week-in-rust.org/rss.xml"

# Multiple Telegram channels
[[telegram]]
nickname = "rust_news"

[[telegram]]
nickname = "programming_tips"
```
