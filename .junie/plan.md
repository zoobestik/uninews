# Test coverage plan for rust-myfeed (collect crate)

> Goal: Achieve high-confidence automated test coverage across the Rust codebase with fast feedback, deterministic
> results, and CI enforcement.

- [x] Establish testing baseline and tooling
  - [x] Verify `cargo test` runs green locally on a clean checkout (no env vars set)
  - [x] Add `cargo-llvm-cov` for coverage reporting and an initial local coverage run
  - [x] Decide on snapshot testing tool (e.g., `insta`) and property testing tool (`proptest`) and add as
    dev-dependencies
  - [x] Create `tests/fixtures/` directory for TOML and output snapshots

- [x] Unit tests: crates/collect/src/config.rs (Config)
  - [x] `Config::new()` returns empty `atoms` and `telegrams`
  - [x] `Default` delegates to `new()`
  - [x] Deserialization: minimal valid TOML with one `[[atom]]` and one `[[telegram]]`
  - [x] Deserialization: many entries (order preserved as in file)
  - [x] Deserialization failures: invalid TOML, wrong types (e.g., `url = 1`, `nickname = false`)
  - [x] `list()` combines atoms then telegrams; count and order assertions
  - [x] `list()` on empty config yields an empty iterator

- [x] Unit tests: crates/collect/src/sources/atom.rs (Atom)
  - [x] Deserialization: valid absolute URL accepted
  - [x] Deserialization failure: invalid/relative URL rejected by `url::Url`
  - [x] `SourceOrigin::original_url()` returns the exact `Url::to_string()` value

- [x] Unit tests: crates/collect/src/sources/telegram.rs (TelegramChannel)
  - [x] Nickname validation success cases: lengths 5, 10, 32; allowed chars (letters, digits, underscore)
  - [x] Nickname validation failures: length < 5; length > 32; invalid chars (spaces, hyphens, punctuation, non-ASCII)
  - [x] Deserialization builds URL exactly as `https://t.me/{nickname}`
  - [x] Error messages propagate via serde error on invalid nickname
  - [x] `SourceOrigin::original_url()` returns the constructed URL string

- [x] Unit tests: crates/collect/src/sources.rs (Source, SourceOrigin)
  - [x] Construct `Source::Atom` and `Source::TelegramChannel` and ensure pattern matching behaves as expected
  - [x] Trait object-style calls to `SourceOrigin` work for both variants (via direct values in tests)

- [x] Unit tests: crates/collect/src/cli.rs (logger, helpers)
  - [x] `init_logger()` sets a global subscriber once; subsequent calls are no-op or error is handled (guard with
    separate process if needed)
  - [x] Env filter default is `info` when `RUST_LOG` is unset
  - [x] Respect `RUST_LOG` when set (e.g., `RUST_LOG=error`)

- [x] Integration tests: CLI end-to-end behavior
  - [ ] With `MYFEED_CONFIG_PATH` pointing to a valid TOML (fixture), `feed-collect` outputs one line per source
    - [ ] Lines are formatted as `[atom] {url}` for Atom and `[tlgr] {url}` for Telegram
    - [ ] Order reflects `Config::list()` chaining (all atoms first, then telegrams)
  - [ ] Without `MYFEED_CONFIG_PATH`, default `./config.toml` is used (use a temp dir with per-test file)
  - [ ] Missing file produces a clear error and exits with code 1
  - [ ] Invalid TOML produces a clear error and exits with code 1
  - [ ] Logging does not intermix with stdout formatting (capture and assert stream separation if applicable)

- [ ] Async and runtime considerations
  - [ ] Ensure tests using Tokio use the appropriate test attribute/runtime flavor
  - [ ] Simulate file read errors (permissions, not found) using temp dirs and set `MYFEED_CONFIG_PATH`

- [ ] Test utilities and fixtures
  - [ ] Helper for creating temp config files with arbitrary sets of atoms/telegrams
  - [ ] Reusable builders for Atom and TelegramChannel test data
  - [ ] Fixture TOML files that mirror `config.toml` examples

- [ ] Snapshot tests for CLI output
  - [ ] Snapshot a small config’s stdout lines for stability
  - [ ] Include a case with Unicode in URLs/nicknames to verify correct rendering

- [ ] CI integration (GitHub Actions or other CI)
  - [ ] Workflow: checkout, toolchain setup, cache, build, test (unit + integration), coverage, and artifacts
  - [ ] Matrix for stable (and optionally nightly) toolchains

- [ ] Coverage and quality gates
  - [ ] Run `cargo llvm-cov` and store HTML report artifact locally
  - [ ] Set an initial coverage threshold (e.g., 80%) and raise over time
  - [ ] Add clippy and fmt checks to CI alongside tests

- [ ] Property-based tests (optional but recommended)
  - [ ] Generate valid Telegram nicknames to ensure validator accepts the full domain of allowed inputs
  - [ ] Generate invalid nicknames (too short/long, invalid chars) to ensure rejections with helpful messages
  - [ ] Fuzz-style URL strings for Atom to ensure only valid absolute URLs deserialize

- [ ] Documentation
  - [ ] CONTRIBUTING notes for running tests, snapshots, property tests, and coverage locally
  - [ ] README: one-liner on how to run the test suite
