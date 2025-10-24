# Sources

UniNews supports two source types today: Atom/RSS feeds and Telegram channels.

This page explains each source type: what input is required, typical validation rules, and tips. For commands to add/list/remove sources, see the [CLI reference → source](./cli.md#source).

## Atom (RSS)

Atom/RSS feeds are standard web feeds.

- Input: a valid HTTP/HTTPS URL to a feed document.
- The CLI validates the URL; duplicates are rejected.
- Tips:
  - Many websites expose a feed at `/feed` or `/feed.xml`.
  - If a site has multiple feeds, choose the one you need (for example, posts vs. comments).

## Telegram channel

A Telegram channel is referenced by its public name.

- Input: the channel name without `@`, for example `telegram`.
- The CLI validates the username format; duplicates are rejected.
- Limitations:
  - Private channels are not supported.
  - Rate limits and availability depend on Telegram.
