# Configuration

## Configuration location and precedence

1. If `MYFEED_CONFIG_PATH` is set, its value is used.
2. Otherwise, the program uses `./config.toml` relative to the current working directory.

## Minimal complete example

```toml
[[atom]]
url = "https://example.com/feed.xml"

[[telegram]]
nickname = "zoobestik"
```

## TOML Properties

### `[[atom]]`

Each `[[atom]]` table describes an Atom/RSS feed entry.

| Property | Type   | Required | Description                                                                 |
|----------|--------|----------|-----------------------------------------------------------------------------|
| `url`    | string | Yes      | Absolute URL of the feed. Must be a valid URL (parsed via the `url` crate). |

### `[[telegram]]`

Each `[[telegram]]` table describes a Telegram source.

| Property   | Type   | Required | Description                                                                                         |
|------------|--------|----------|-----------------------------------------------------------------------------------------------------|
| `nickname` | string | Yes      | Channel/user nickname without `@`. The application constructs the URL as `https://t.me/<nickname>`. |
