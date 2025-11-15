# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

UniNews is a Rust CLI tool for collecting content from multiple sources (Atom/RSS feeds and Telegram channels) into a single local feed stored in SQLite. The binary is named `uninews` and is built from the `news_cli` crate.

## Workspace Structure

This is a Cargo workspace with four crates under `crates/`:

- **cli** (`news_cli`) - Main binary crate providing the `uninews` CLI interface with colorized output
- **core** (`news_core`) - Core domain models, traits, and service interfaces
- **sqlite_core** (`news_sqlite_core`) - SQLite-specific implementations of core traits
- **watch** (`news_watch`) - Experimental watch mode for continuous content monitoring

### Architectural Pattern

The codebase follows a service-oriented architecture with trait-based abstraction:

1. **Core Layer** (`news_core`): Defines domain models and service traits
   - `SourceService` trait: CRUD operations for content sources (add, get_by_id, get_all, drop_by)
   - `NewsService` trait: Updates news items in bulk
   - `HttpService` trait: Watches changes via HTTP with `HttpUpdateHandle`
   - Models implement `ExternalEntity` trait providing `source_key()` for UUID-based identity

2. **Implementation Layer** (`news_sqlite_core`): SQLite-backed implementations
   - Implements all service traits using `sqlx` for async database operations
   - Automatic migrations from `migrations/` directory

3. **CLI Layer** (`news_cli`): User-facing command interface
   - Uses `clap` for argument parsing with custom styled output
   - Respects `NO_COLOR` environment variable and terminal capabilities

4. **Watch Layer** (`news_watch`): Feed monitoring (experimental)
   - Handles Atom/RSS feed parsing via `feed-rs`
   - HTML sanitization with `ammonia` and conversion with `htmd`

### Key Design Principles

- **UUID Strategy**: Sources use both `source_key` (external UUID via UUID v5/v7) and internal database IDs
- **Error Handling**: Service traits define specific error types (e.g., `AddError::AlreadyExists`, `GetError::NotFound`)
- **DeleteCriteria**: Type alias for `SourceDraft` used in removal operations
- **Async-First**: All service operations are async using `tokio` runtime

## Build and Development Commands

### Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Build specific package
cargo build -p news_cli
```

### Running Without Installation

```bash
# Run CLI without installing
cargo run -p news_cli -- --help
cargo run -p news_cli -- init
cargo run -p news_cli -- source ls
```

### Testing

```bash
# Run all tests
cargo test --workspace --all-features --all-targets

# Run tests for specific crate
cargo test -p news_core

# Run single test by name
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

### Code Quality

```bash
# Format code
cargo fmt --all

# Check formatting (CI-style)
cargo fmt --all -- --check

# Run clippy lints
cargo clippy --all-targets --all-features -- -D warnings

# Check TOML formatting (requires taplo-cli)
taplo lint
```

### Database Operations

```bash
# Initialize database (creates SQLite file and runs migrations)
./target/release/uninews init

# Force re-initialization
./target/release/uninews init --force

# Use custom database path
UNINEWS_DB_PATH=/tmp/uninews.sqlite ./target/release/uninews init
```

Default database location: `data/app.sqlite`

Migrations are in `migrations/` and applied automatically during `init`.

### Logging and Debugging

```bash
# Enable debug logging
RUST_LOG=debug ./target/release/uninews collect

# Debug mode with color output
DEBUG=1 ./target/release/uninews collect

# Disable colors for scripting
NO_COLOR=1 ./target/release/uninews collect
```

Logging uses `tracing` and `tracing-subscriber` with environment filter support.

## Docker

Multi-stage Dockerfile using Alpine for minimal image size:

```bash
# Build image
docker build -t uninews .

# Run with volume mount
docker run -v $(pwd)/data:/app/data uninews

# Use docker-compose
docker-compose up -d
```

## SQLx Offline Mode

The project uses SQLx's offline mode with `.sqlx/` directory for compile-time checked queries without database connection. If you modify queries:

```bash
# Prepare offline query data (requires DATABASE_URL)
cargo sqlx prepare
```

## Cargo Configuration

Custom release profile in `.cargo/config.toml`:
- Thin LTO enabled
- 16 codegen units
- Panic abort strategy
- Debug symbols stripped
- Line tables only for profiling

Additional `profiling` profile inherits from `release` with full debug symbols.

## Recent Refactoring Notes

Based on recent commits:
- Renamed `source_id` to `source_key` across traits and models (commit ad2eab2)
- Replaced `drop_by_draft` with `drop_by` using `DeleteCriteria` alias (commit 023f308)
- Migrated from repository pattern to service pattern (`SourceRepository` → `SourceService`, `NewsRepository` → `NewsService`) (commit 0906c60)
- Unified UUID logic under `ExternalEntity` trait (commit 0906c60)
- Enhanced error handling with `AddError::AlreadyExists` variant (commit be28234)

When working with source management, use `SourceService` trait methods, not deprecated repository patterns.
