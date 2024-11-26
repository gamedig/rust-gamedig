# rust-GameDig CLI

The official [rust-GameDig](https://crates.io/crates/gamedig_cli) Command Line Interface.

[![CI](https://github.com/gamedig/rust-gamedig/actions/workflows/ci.yml/badge.svg)](https://github.com/gamedig/rust-gamedig/actions) [![License:MIT](https://img.shields.io/github/license/gamedig/rust-gamedig?color=blue)](https://github.com/gamedig/rust-gamedig/blob/main/LICENSE.md) [![node coverage](https://raw.githubusercontent.com/gamedig/rust-gamedig/main/.github/badges/node.svg)](https://github.com/gamedig/node-gamedig)

## Installation

You can install the CLI via `cargo`:

```sh
cargo install gamedig_cli
```

or

```sh
cargo install gamedig_cli --git https://github.com/gamedig/rust-gamedig.git
```

## Usage

Running `gamedig_cli` without any arguments will display the usage information. You can also use the `--help` (or `-h`) flag to see detailed usage instructions.

Here's also a quick rundown of a simple query with the `json-pretty` format:

Pick a game/service/protocol (check
the [GAMES](https://github.com/gamedig/rust-gamedig/blob/main/GAMES.md), [SERVICES](https://github.com/gamedig/rust-gamedig/blob/main/SERVICES.md)
and [PROTOCOLS](https://github.com/gamedig/rust-gamedig/blob/main/PROTOCOLS.md) files to see the currently supported
ones), provide the ip and the port (be aware that some game servers use a separate port for the info queries, the port
can also be optional if the server is running the default ports) then query on it.

[Team Fortress 2](https://store.steampowered.com/app/440/Team_Fortress_2/) query example:

```sh
gamedig_cli query -g teamfortress2 -i 127.0.0.1 -f json-pretty
```

What we are doing here:

- `-g` (or `--game`) specifies the game.
- `-i` (or `--ip`) target ip.
- `-f` (or `--format`) our preferred format.

Note: We haven't specified a port (via `-p` or `--port`), so the default one for the game will be used (`27015` in this
case).

Response (note that some games have a different structure):

```json
{
  "name": "A cool server.",
  "description": null,
  "game_mode": "Team Fortress",
  "game_version": "8690085",
  "map": "cp_foundry",
  "players_maximum": 24,
  "players_online": 0,
  "players_bots": 0,
  "has_password": false,
  "players": []
}
```

## Contributing

Please read [CONTRIBUTING](https://github.com/gamedig/rust-gamedig/blob/main/CONTRIBUTING.md).
