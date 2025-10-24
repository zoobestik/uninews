[![License-MIT][mit-img]][mit-url]
[![CI][ci-img]][ci-url]

# UniNews

A small Rust CLI to collect content from multiple sources into one local feed. Supports Atom/RSS feeds and Telegram channels, stored in a SQLite database.

Status: early alpha; behavior will evolve. The `--watch` flag is experimental.

## Features
- Single binary: `uninews`
- SQLite storage with automatic migrations
- Manage sources: add/list/remove (Atom/RSS and Telegram) with validation
- One-shot collection; planned continuous mode via `--watch`
- Logging controlled by `RUST_LOG`

## Quickstart
Build the release binary:

```bash
cargo build --release
```

Run commands:

```bash
./target/release/uninews --help
./target/release/uninews init
./target/release/uninews source add atom https://example.com/feed.xml
./target/release/uninews source ls
RUST_LOG=debug ./target/release/uninews collect
```

Alternative (no install):

```bash
cargo run -p uninews_cli -- --help
```

To install the binary into your PATH, see [Installation](docs/installation.md).

## Documentation
- Start → [docs/home.md](docs/home.md)
- Getting Started → [docs/getting-started.md](docs/getting-started.md)
- Installation → [docs/installation.md](docs/installation.md)
- CLI reference → [docs/cli.md](docs/cli.md)
- Sources → [docs/sources.md](docs/sources.md)
- Database → [docs/database.md](docs/database.md)
- Environment → [docs/environment.md](docs/environment.md)

Note: a configuration file is not supported yet. Use CLI commands and environment variables.

## Contributing

Issues and PRs are not welcome. 🙃
For larger changes, please open an issue first.

- Build: `cargo build --release`
- Formatting: `cargo fmt --all`
- Lints: `cargo clippy --all-targets --all-features -- -D warnings`
- Smoke test:
  ```bash
  ./target/release/uninews init --force
  ./target/release/uninews source ls
  ```

## Acknowledgements
[![MIT license][mit-img]][mit-url] [![Develop By][author-img]][author-url]

[mit-img]: https://img.shields.io/badge/License-MIT-teal.svg
[mit-url]: https://opensource.org/licenses/MIT

[ci-img]: https://github.com/zoobestik/uninews/actions/workflows/ci.yml/badge.svg
[ci-url]: https://github.com/zoobestik/uninews/actions/workflows/ci.yml

[author-img]: https://img.shields.io/badge/develop%20by-zoobestik-blue.svg?style=flat
[author-url]: https://www.linkedin.com/in/kbchernenko/
