> [!WARNING]  
> WIP. This is a experimental branch of the GameDig library, it is not ready for any use

<h1 align="center">Rust GameDig</h1>


<img align="right" src="https://github.com/user-attachments/assets/179d72f8-0c1f-4034-9852-b725254ece53" alt="image" />


A Rust library and CLI tool for querying the status and details of multiplayer game servers. 

&nbsp;

&nbsp;


> TODO: add more detail to the short description and some badges

## Crates

### User Crates

These are the main crates that users will interact with. They provide the functionality for querying game servers.

| Crate         | Path         | Description                                                      | Crates.io       | CI Status       | Coverage        |
| ------------- | ------------ | ---------------------------------------------------------------- | --------------- | --------------- | --------------- |
| `gamedig`     | `crates/lib` | The main crate with all game querying logic.                     | TODO: Add badge | TODO: Add badge | TODO: Add badge |
| `gamedig_cli` | `crates/cli` | A command line interface for querying game servers.              | TODO: Add badge | TODO: Add badge | TODO: Add badge |
| `gamedig_ffi` | `crates/ffi` | FFI bindings & UDL schemas for integrating with other languages. | ❌              | TODO: Add badge | TODO: Add badge |

> TODO: add links on crate names to their respective readmes

### Internal Crates

These are internal crates used for testing and development purposes. They are not intended for public use.

| Crate | Path              | Description     |
| ----- | ----------------- | --------------- |
| TODO  | `crates/tools/..` | For future use. |

### Target Support

| Symbol | Type            | Meaning                                                                       |
| ------ | --------------- | ----------------------------------------------------------------------------- |
| ✅     | **Supported**   | Actively tested in CI and maintained.                                         |
| 🟡     | **May work**    | Not tested or maintained. May require additional setup. Use at your own risk. |
| ⛔     | **Unsupported** | Not expected to work due to missing features or platform limitations.         |

| Platform        | Tier | Notes                                                                                  |
| --------------- | ---- | -------------------------------------------------------------------------------------- |
| Windows         | ✅   | TODO: add notes on tested env                                                          |
| Linux           | ✅   | TODO: add notes on tested env                                                          |
| macOS           | ✅   | TODO: add notes on tested env                                                          |
| Android, iOS    | 🟡   | Requires additional setup like SDKs/NDKs and proper linker setup.                      |
| Embedded        | 🟡   | Targets with `std` may work, but may require additional setup. `no_std` will not work. |
| WebAssembly     | ⛔   | Will not work due to socket constraints.                                               |
| UEFI, SGX, etc. | ⛔   | Not designed for those environments.                                                   |

## Contributing

Contributions are welcome! Please see the [CONTRIBUTING.md](./CONTRIBUTING.md) for details on how to contribute to this project.

> TODO: redo the contributing guidelines

## Security

If you discover a security vulnerability within this project, please follow the [security policy](./SECURITY.md) to report it responsibly.

> TODO: add security policy

## License

This project is licensed under the [MIT License](./LICENSE). See the [LICENSE](./LICENSE) file for details.

> TODO: finish the README.md
