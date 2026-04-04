#[cfg(not(any(feature = "rt_std", feature = "rt_tokio")))]
compile_error!(
    r#"
╔══════════════════════════════════════════════════════════════════════════════════════════════╗
║                            No `gamedig` Runtime Feature Enabled                              ║
╠══════════════════════════════════════════════════════════════════════════════════════════════╣
║ `gamedig` requires a runtime but none was selected.                                          ║
║                                                                                              ║
║ Enable exactly one runtime feature:                                                          ║
║                                                                                              ║
║   • rt_std                                                                                   ║
║   • rt_tokio                                                                                 ║
║                                                                                              ║
║ Example:                                                                                     ║
║   gamedig = { version = "*", default-features = false, features = ["rt_std"] }               ║
║   gamedig = { version = "*", default-features = false, features = ["rt_tokio"] }             ║
╚══════════════════════════════════════════════════════════════════════════════════════════════╝
"#
);

#[cfg(any(
    all(feature = "rt_std", feature = "rt_tokio"),
    all(feature = "default_client_std", feature = "default_client_tokio"),
    all(feature = "default_client_std", feature = "rt_tokio"),
    all(feature = "default_client_tokio", feature = "rt_std"),
))]
compile_error!(
    r#"
╔══════════════════════════════════════════════════════════════════════════════════════════════╗
║                 Invalid `gamedig` Runtime / Default Feature Configuration                    ║
╠══════════════════════════════════════════════════════════════════════════════════════════════╣
║ `gamedig` was configured with both the Std and Tokio runtimes.                               ║
║                                                                                              ║
║ Only one runtime may be selected.                                                            ║
║                                                                                              ║
║ This happens when:                                                                           ║
║   • `rt_std` and `rt_tokio` are enabled together                                             ║
║   • `default_client_std` and `default_client_tokio` are enabled together                     ║
║   • `rt_tokio` is combined with `default_client_std`                                         ║
║   • `rt_std` is combined with `default_client_tokio`                                         ║
║                                                                                              ║
║ Choose exactly ONE runtime configuration:                                                    ║
║                                                                                              ║
║ Runtime features (used with custom feature sets):                                            ║
║   gamedig = { version = "*", default-features = false, features = ["rt_std", ...] }          ║
║   gamedig = { version = "*", default-features = false, features = ["rt_tokio", ...] }        ║
║                                                                                              ║
║ Default bundles (fully featured clients):                                                    ║
║   gamedig = "*"                                                                              ║
║   gamedig = { version = "*", default-features = false, features = ["default_client_tokio"] } ║
║                                                                                              ║
║ Note: `default_client_std` is enabled by default.                                            ║
║ If you want to use Tokio, disable default features.                                          ║
╚══════════════════════════════════════════════════════════════════════════════════════════════╝
"#
);
