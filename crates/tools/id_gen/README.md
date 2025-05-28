> [!WARNING]  
> WIP. This is a experimental branch of Rust GameDig, it is not ready for any use

<h1 align="center">Rust GameDig ID Gen</h1>

<img align="right" src="https://github.com/user-attachments/assets/179d72f8-0c1f-4034-9852-b725254ece53" alt="image" />

A CLI tool to generate deterministic identifiers for games.

&nbsp;

&nbsp;

> TODO: add more detail to the short description and some badges

## Specification

### Overview

This defines a standardized, snake_case naming convention for generating unique identifiers (IDs) for games. The goal is to produce human readable, collision resistant IDs that reflect storefront titles.

### Definitions

| Term                | Description                                                                                             |
| ------------------- | ------------------------------------------------------------------------------------------------------- |
| **Game Title**      | Official name exactly as published.                                                                     |
| **Edition**         | Distribution variant.                                                                                   |
| **Mod**             | A third party modification.                                                                             |
| **Segment**         | A normalized component contributing to the ID (Title, Mod, Edition, Year).                              |
| **Snake Case**      | Lowercase letters and digits, underscores only; non alphanumeric runs → `_`; trim leading/trailing `_`. |
| **Base Key**        | Concatenation of Title and, if present, Edition and/or Mod.                                             |
| **Collision Group** | All entries sharing the same Base Key.                                                                  |

## Normalization

To normalize any raw string segment:

1. Trim whitespace.
2. Replace each contiguous sequence of non alphanumeric characters (`[^A-Za-z0-9]`) with a single underscore (`_`).
3. Remove leading/trailing underscores.
4. Convert to lowercase.

Example:

```plain
normalize(" Grand-Theft!Auto: V ") → "grand_theft_auto_v"
```

## Collision Handling

When two or more distinct releases share the same **Base Key** (AKA **Collision Group**), append the four digit release year to **every** ID in that **Collision Group**.

If this does not make the ID unique, the edition **Segment** should be used to differentiate them.

## Examples

| Title                   | Mod   | Edition | Year | Base Key                   | Base Key Collision | Final ID                       |
| ----------------------- | ----- | ------- | ---- | -------------------------- | ------------------ | ------------------------------ |
| ARMA 2                  | -     | -       | -    | `arma_2`                   | No                 | `arma_2`                       |
| Grand Theft Auto V      | FiveM | -       | -    | `grand_theft_auto_v_fivem` | No                 | `grand_theft_auto_v_fivem`     |
| Minecraft               | -     | Java    | -    | `minecraft_java`           | No                 | `minecraft_java`               |
| Star Wars Battlefront 2 | -     | -       | 2005 | `star_wars_battlefront_2`  | Yes                | `star_wars_battlefront_2_2005` |
| Star Wars Battlefront 2 | -     | -       | 2017 | `star_wars_battlefront_2`  | Yes                | `star_wars_battlefront_2_2017` |

## Usage

```bash
# Run from the root of the workspace.
#
# Remember to wrap arguments in quotes if they contain spaces (e.g. -t "ARMA 2").
#
# The CLI has no map of current IDs, so it is up to you to ensure it follows the spec.
cargo run -p gamedig_id_gen -- -t <TITLE> [OPTIONS]

# Options:
#  -t, --title <TITLE>      Game title exactly as on storefront
#  -m, --mod <MOD_NAME>     Mod name (optional; appended if present)
#  -e, --edition <EDITION>  Edition (optional; appended if present)
#  -y, --year <YEAR>        Release year (optional; appended if present)
#  -h, --help               Print help
#  -V, --version            Print version
```
