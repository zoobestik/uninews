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

## Environment filters (logging)

UniNews uses environment-based logging filters. The default level is `info`.

```bash
# Enable debug logs
RUST_LOG=debug uninews collect
```

## Configuration file

A configuration file is not supported yet. All configuration is done via CLI commands and environment variables. See [Environment](./environment.md).
