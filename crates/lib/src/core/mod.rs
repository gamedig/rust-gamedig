pub(crate) mod io;
pub(crate) mod net;

// Ensure that exactly one client feature is enabled
#[cfg(not(any(
    all(
        feature = "async-tokio-client",
        not(any(feature = "async-std-client", feature = "sync-std-client"))
    ),
    all(
        feature = "async-std-client",
        not(any(feature = "async-tokio-client", feature = "sync-std-client"))
    ),
    all(
        feature = "sync-std-client",
        not(any(feature = "async-tokio-client", feature = "async-std-client"))
    )
)))]
compile_error!(
    "GameDig Core Compiler Error: Exactly one client feature must be enabled: \
     \n`async-tokio-client`, `async-std-client`, or `sync-std-client`. \n\nEnsure that exactly \
     one client feature is selected and all others are disabled. \nBy default, the \
     `async-tokio-client` feature is enabled.\n\nExample usage in Cargo.toml:\n\nTo use the \
     default async-tokio-client:\n[dependencies]\ngamedig = \"X\"\n\nTo use a different client, \
     disable default features and specify the desired client:\n\nFor \
     async-std-client:\n[dependencies]\ngamedig = { version = \"X\", default-features = false, \
     features = [\"async-std-client\"] }\n\nFor sync-std-client:\n[dependencies]\ngamedig = { \
     version = \"X\", default-features = false, features = [\"sync-std-client\"] }\n"
);
