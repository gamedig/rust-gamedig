Who knows what the future holds...

# X.Y.Z - DD/MM/YYYY

Nothing... yet.

# 0.3.0 - 23/04/2025

### Changes:

- CLI now uses `gamedig` v0.7.0 (To update, run `cargo install gamedig_cli`).

### Breaking Changes:

- MSRV has been updated to `1.81.0` to match the latest `gamedig` version.

# 0.2.1 - 05/12/2024

Dependencies:
- `gamedig`: `v0.6.0 -> v0.6.1`

# 0.2.0 - 26/11/2024

### Breaking Changes:

- Restructured the release flow to be more consistent (GitHub releases will no longer be available, use cargo instead).
- Changed crate name from `gamedig-cli` to `gamedig_cli` to align with recommended naming conventions.
- The CLI now requires a minimum Rust version of `1.74.1`.

# 0.1.1 - 15/07/2024

### Changes:

- Dependency updates (by @cainthebest)
  - `gamedig`: `v0.5.0 -> v0.5.1`
  - `clap`: `v4.1.11 -> v4.5.4`
  - `quick-xml`: `v0.31.0 -> v0.36.0`
  - `webbrowser`: `v0.8.12 -> v1.0.0`

# 0.1.0 - 15/03/2024

### Changes:

- Added the CLI (by @cainthebest).
- Added DNS lookup support (by @Douile).
- Added JSON output option (by @Douile).
- Added BSON output in hex or base64 (by @cainthebest).
- Added XML output option (by @cainthebest).
- Added ExtraRequestSettings as CLI arguments (by @Douile).
- Added TimeoutSettings as CLI argument (by @Douile).
- Added Comprehensive end-user documentation for the CLI interface (by @Douile & @cainthebest).
- Tweaked compile-time flags to allow for a more preformant binary (by @cainthebest).
- Added client for socket capture, dev tools are not included by default (by @Douile).
- Added license information to the CLI (by @cainthebest).
- Added source code information to the CLI (by @cainthebest).
