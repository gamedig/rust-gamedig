pub(crate) mod io;
pub(crate) mod net;

pub(crate) mod prelude;

// Ensure that exactly one client feature is enabled
#[cfg(not(any(
    all(feature = "client_std", not(feature = "client_tokio")),
    all(feature = "client_tokio", not(feature = "client_std")),
)))]
compile_error!(
    "GameDig Core Compiler Error: Exactly 1 client feature must be enabled: \n`client_std` or \
     `client_tokio`. \n\nEnsure that exactly 1 client feature is selected and the other is \
     disabled. \nBy default, the `client_std` feature is enabled.\n\nExample usage in \
     Cargo.toml:\n\nTo use the default `client_std`:\n[dependencies]\ngamedig = { version = \"X\" \
     features = [\"some_non_client_feature\"] }\n\nTo use a different client, disable default \
     features and specify the desired client:\n\nFor tokio use:\n[dependencies]\ngamedig = { \
     version = \"X\", default-features = false, features = [\"client_tokio\", \
     \"non_client_feature\"] }\n"
);
