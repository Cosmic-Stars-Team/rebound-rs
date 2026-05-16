# rebound-rs project tasks

set dotenv-load

# Show available recipes.
@default:
    just --list

# Remove Cargo build artifacts.
@clean:
    cargo clean

# Check whether the crate compiles.
@check:
    cargo check --all-targets

# Build the crate.
@build:
    cargo build --all-targets

# Build the crate in release mode.
@release:
    cargo build --release --all-targets

# Run all tests.
@test:
    cargo test --all-targets

# Run rustfmt.
@fmt:
    cargo fmt --all

# Check formatting without modifying files.
@fmt-check:
    cargo fmt --all -- --check

# Run clippy with warnings treated as errors.
@clippy:
    cargo clippy --all-targets -- -D warnings

# Run the full local CI suite.
@ci: fmt-check clippy test

# Generate documentation without building dependencies' docs.
@doc:
    cargo doc --no-deps

# Generate documentation and open it in the browser.
@doc-open:
    cargo doc --no-deps --open

# Run an example, e.g. `just example simplest`.
@example name:
    cargo run --example {{ name }}

# Run all examples currently in examples/.
@examples:
    cargo run --example api
    cargo run --example heartbeat
    cargo run --example high_order_symplectic
    cargo run --example orbital_elements
    cargo run --example rotations
    cargo run --example simplest

# Show dependency tree.
@tree:
    cargo tree

# Verify the crate package contents.
@package:
    cargo package --allow-dirty
