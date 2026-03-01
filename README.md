> [!WARNING]  
> WIP. This is a experimental branch of Rust GameDig, it is not ready for any use

<h1 align="center">Rust GameDig</h1>

<img align="right" src="https://github.com/user-attachments/assets/179d72f8-0c1f-4034-9852-b725254ece53" alt="image" />

A Rust library and CLI tool for querying the status and details of multiplayer game servers.

&nbsp;

&nbsp;

> TODO: add more detail to the short description and some badges

## Crates

### User Crates

These are the main crates that users will interact with. They provide the functionality for querying game servers.

| Crate                                   | Path              | Description                                           | Crates.io       | CI Status       | Coverage        |
| --------------------------------------- | ----------------- | ----------------------------------------------------- | --------------- | --------------- | --------------- |
| [`gamedig`](./crates/lib)               | `crates/lib`      | The main crate with all game querying logic.          | TODO: Add badge | TODO: Add badge | TODO: Add badge |
| [`gamedig_cli`](./crates/cli)           | `crates/cli`      | A command line interface for querying game servers.   | TODO: Add badge | TODO: Add badge | TODO: Add badge |
| [`gamedig_ffi`](./crates/ffi)           | `crates/ffi`      | A C FFI library for integrating with other languages. | ❌              | TODO: Add badge | TODO: Add badge |
| [`gamedig_ffi_neon`](./crates/ffi_neon) | `crates/ffi_neon` | A Neon FFI library for integrating with Node.js.      | ❌              | TODO: Add badge | TODO: Add badge |

### Internal Crates

These are internal crates used for testing and development purposes. They are not intended for public use.

| Crate                                             | Path                      | Description                                                 |
| ------------------------------------------------- | ------------------------- | ----------------------------------------------------------- |
| [`gamedig_id_gen`](./crates/tools/id_gen)         | `crates/tools/id_gen`     | A CLI tool to generate deterministic identifiers for games. |
| [`gamedig_net_replay`](./crates/tools/net_replay) | `crates/tools/net_replay` | A CLI tool for replaying network traffic from game servers. |

### Target Support

| Symbol | Type            | Meaning                                                                       |
| ------ | --------------- | ----------------------------------------------------------------------------- |
| ✅     | **Supported**   | Actively tested in CI and maintained.                                         |
| 🟡     | **May work**    | Not tested or maintained. May require additional setup. Use at your own risk. |
| ⛔     | **Unsupported** | Not expected to work due to missing features or platform limitations.         |

| Platform     | Tier | Notes                                                                                  |
| ------------ | ---- | -------------------------------------------------------------------------------------- |
| Windows      | ✅   | TODO: add notes on tested env                                                          |
| Linux        | ✅   | TODO: add notes on tested env                                                          |
| macOS        | ✅   | TODO: add notes on tested env                                                          |
| Android, iOS | 🟡   | Requires additional setup like SDKs/NDKs and proper linker setup.                      |
| Embedded     | 🟡   | Targets with `std` may work, but may require additional setup. `no_std` will not work. |
| WebAssembly  | ⛔   | Will not work due to socket constraints.                                               |

## Contributing

Contributions are welcome! Please see the [CONTRIBUTING.md](./CONTRIBUTING.md) for details on how to contribute to this project.

> TODO: redo the contributing guidelines

## Security

If you discover a security vulnerability within this project, please follow the [security policy](./SECURITY.md) to report it responsibly.

> TODO: add security policy

## License

This project is licensed under the [MIT License](./LICENSE.md). See the [LICENSE](./LICENSE.md) file for details.

> TODO: finish the README.md
