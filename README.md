[![License-MIT](https://img.shields.io/badge/License-MIT-teal.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/zoobestik/uninews/actions/workflows/ci.yml/badge.svg)](https://github.com/zoobestik/uninews/actions/workflows/ci.yml)

# UnifyNews

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

### Run

```bash
cargo run -p uninews_collect
```

Set log level (optional):
```bash
RUST_LOG=debug cargo run -p uninews_collect
```

## Configuration

Short snippet:
```toml
[[atom]]
url = "..."

[[telegram]]
nickname = "..."
```

Full example and field reference: [docs/configuration.md](docs/configuration.md)

## Contributing

Issues and PRs are not welcome. 🙃
Please open an issue to discuss bigger changes.

## License

MIT — see [LICENSE](LICENSE).
