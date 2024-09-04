# justfile for the workspace
# This file is used to define "recipes" for ease of use
#
# Book: https://just.systems/man/en/
# Repo: https://github.com/casey/just

# List all recipes
default:
    just -l

# Lint the entire workspace with default features
lint:
    cargo clippy --workspace --all-targets -- -D warnings

# Format the entire workspace
format:
    cargo +nightly fmt --all

# Build the entire workspace (Debug)
build:
    cargo build --workspace

# Build the entire workspace for release
build_release:
    cargo build --workspace --release

# Build a specific crate (Debug)
[positional-arguments]
build_crate crate_name_from_toml:
    cargo build --package {{crate_name_from_toml}}

# Build a specific crate for release
[positional-arguments]
build_crate_release crate_name_from_toml:
    cargo build --package {{crate_name_from_toml}} --release

# Test the entire workspace
test:
    cargo test --workspace

# Test a specific crate
[positional-arguments]
test_crate crate_name_from_toml:
    cargo test --package {{crate_name_from_toml}}

# Lint library with the std default client feature
lint_lib_std:
    cargo clippy --package gamedig --lib --features default_client_std --no-default-features -- -D warnings

# Lint library with the tokio default client feature
lint_lib_tokio:
    cargo clippy --package gamedig --lib --features default_client_tokio --no-default-features -- -D warnings
