#!/usr/bin/env bash
set -euo pipefail

# Requires: cargo-llvm-cov (install with: cargo install cargo-llvm-cov)
# Usage: scripts/coverage.sh [--open]

OPEN=${1:-}

if ! command -v cargo-llvm-cov >/dev/null 2>&1; then
  echo "cargo-llvm-cov not found. Install with: cargo install cargo-llvm-cov" >&2
  exit 1
fi

# Run coverage for the whole workspace and generate an HTML report
cargo llvm-cov --workspace --lcov --output-path target/lcov.info --html --output-dir target/llvm-cov

if [[ "${OPEN}" == "--open" ]]; then
  # Attempt to open the HTML report index
  if command -v open >/dev/null 2>&1; then
    open target/llvm-cov/index.html
  elif command -v xdg-open >/dev/null 2>&1; then
    xdg-open target/llvm-cov/index.html
  else
    echo "Report generated at target/llvm-cov/index.html"
  fi
else
  echo "Report generated at target/llvm-cov/index.html"
fi
