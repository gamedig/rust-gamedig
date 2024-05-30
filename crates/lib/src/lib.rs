mod core;
mod definition;
mod error;

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
    "Exactly one client feature must be enabled: `async-tokio-client`, `async-std-client`, or \
     `sync-std-client`. Ensure that exactly one client feature is selected with GameDig and that \
     all others are disabled. By default, the `async-tokio-client` feature is enabled."
);
