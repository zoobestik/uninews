[![License-MIT][mit-img]][mit-url]
[![CI][ci-img]][ci-url]

# UniNews

A small Rust workspace for experimenting with collecting content sources into a single feed pipeline.

🚨Early alpha; behavior will evolve.

## Features
 - Simple, fast, easy to use
 - Configurable news sources:
   - Atom
   - Telegram

## Development

### Prerequisites

- Rust toolchain: stable (as per [`rust-toolchain.toml`](./rust-toolchain.toml))
- Install [Taplo](https://github.com/tamasfe/taplo): `cargo install taplo-cli`

### Run

You can use the provided cargo aliases (recommended):

```bash
cargo collect
```

Set the log level (optional):
```bash
RUST_LOG=debug cargo collect
```

Other binaries:
- Manage (WIP):
  ```bash
  cargo manage
  ```

## Configuration

By default, the app loads a local `.env` (if present), then reads the configuration from `UNINEWS_CONFIG_PATH` if set; otherwise it falls back to `./config.toml` relative to the current working directory. See [configuration.md](docs/configuration.md) for details.

Short snippet:
```toml
[[atom]]
source_url = "https://example.com/feed.xml"

[[telegram]]
nickname = "..."
```

Full example and field reference: [docs/configuration.md](docs/configuration.md)

## Contributing

Issues and PRs are not welcome. 🙃
Please open an issue to discuss bigger changes.

## Acknowledgements
[![MIT license][mit-img]][mit-url] [![Develop By][author-img]][author-url]

[mit-img]: https://img.shields.io/badge/License-MIT-teal.svg
[mit-url]: https://opensource.org/licenses/MIT

[ci-img]: https://github.com/zoobestik/uninews/actions/workflows/ci.yml/badge.svg
[ci-url]: https://github.com/zoobestik/uninews/actions/workflows/ci.yml

[author-img]: https://img.shields.io/badge/develop%20by-zoobestik-blue.svg?style=flat
[author-url]: https://ru.linkedin.com/in/kbchernenko
