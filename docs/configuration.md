# Configuration

## Configuration location and precedence

1. If `UNINEWS_CONFIG_PATH` is set (from the environment or loaded via a local `.env` file), its value is used.
2. Otherwise, the program uses `./config.toml` relative to the current working directory.

Note: If a `.env` file is present in the current working directory, it is loaded before reading `UNINEWS_CONFIG_PATH`. This allows you to define environment variables in `.env`. 

## Minimal complete example

```toml
[[atom]]
source_url = "https://example.com/feed.xml"

[[telegram]]
nickname = "zoobestik"
```

## TOML Properties

Note: Unknown fields are not allowed in the TOML config. Any misspelled or extra keys will cause a parse error.

### `[[atom]]`

Each `[[atom]]` table describes an Atom/RSS feed entry.

| Property         | Type   | Required | Description                                                                                                                                                                                                                                |
|------------------|--------|----------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `source_url`     | string | Yes      | Absolute URL of the feed. Must be a valid URL (parsed via the `url` crate).                                                                                                                                                                |

### `[[telegram]]`

Each `[[telegram]]` table describes a Telegram source.

| Property   | Type   | Required | Description                                                                                                                                                                          |
|------------|--------|----------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `nickname` | string | Yes      | Channel/user nickname without `@`. The application constructs the URL as `https://t.me/<nickname>`. Must be 5–32 characters long and contain only letters, numbers, and underscores. |
