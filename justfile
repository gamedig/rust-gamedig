# justfile for the workspace
# This file is used to define "recipes" for ease of use
#
# Book: https://just.systems/man/en/
# Repo: https://github.com/casey/just

# List all recipes
default:
    just -l

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

