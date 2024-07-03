pub(crate) mod io;
pub(crate) mod net;

// Ensure that exactly one client feature is enabled
#[cfg(not(any(
    all(
        feature = "client_std",
        not(any(feature = "client_tokio", feature = "client_async_std"))
    ),
    all(
        feature = "client_tokio",
        not(any(feature = "client_std", feature = "client_async_std"))
    ),
    all(
        feature = "client_async_std",
        not(any(feature = "client_std", feature = "client_tokio"))
    ),
)))]
compile_error!(
    "GameDig Core Compiler Error: Exactly 1 client feature must be enabled: \n`client_std`, \
     `client_tokio`, or `client_async_std`. \n\nEnsure that exactly 1 client feature is selected \
     and all others are disabled. \nBy default, the `client_std` feature is enabled.\n\nExample \
     usage in Cargo.toml:\n\nTo use the default `client_std`:\n[dependencies]\ngamedig = { version \
     = \"X\" features = [\"some_non_client_feature\"] }\n\nTo use a different client, disable \
     default features and specify the desired client:\n\nFor tokio use:\n[dependencies]\ngamedig \
     = { version = \"X\", default-features = false, features = [\"client_tokio\", \
     \"non_client_feature\"] }\n\nFor async_std use:\n[dependencies]\ngamedig = { version = \
     \"X\", default-features = false, features = [\"client_async_std\", \"non_client_feature\"] \
     }\n"
);
