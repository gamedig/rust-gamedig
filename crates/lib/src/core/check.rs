//TODO: come up with new compile errors for the new features
/* 
// Runtime Client Feature Check
//
// This check ensures that exactly one runtime client feature is enabled at a
// time. The runtime client features are `client_std` and `client_tokio`.
// If both or neither are enabled, a compile-time error is generated.
#[cfg(not(any(
    all(feature = "client_std", not(feature = "client_tokio")),
    all(feature = "client_tokio", not(feature = "client_std")),
)))]
compile_error!(
    "GameDig Feature Compiler Error: Exactly 1 client feature must be enabled: \n`client_std` or \
     `client_tokio`. \n\nEnsure that exactly 1 client feature is selected and the other is \
     disabled. \nBy default, the `client_std` feature is enabled.\n\nExample usage in \
     Cargo.toml:\n\nTo use the default `client_std`:\n[dependencies]\ngamedig = { version = \"X\" \
     features = [\"some_non_client_feature\"] }\n\nTo use a different client, disable default \
     features and specify the desired client:\n\nFor tokio use:\n[dependencies]\ngamedig = { \
     version = \"X\", default-features = false, features = [\"client_tokio\", \
     \"non_client_feature\"] }\n"
);

// HTTP Client Check
//
// This check ensures that only one HTTP client type (`http_std` or
// `http_tokio`) is enabled at a time, and that it matches the enabled runtime
// client (`client_std` or `client_tokio`). It also verifies that an HTTP client
// type is only enabled when a `_HTTP` feature is enabled.
#[cfg(not(any(
    // Correct combinations with only 1 HTTP client type selected
    all(feature = "client_std", feature = "http_std", not(any(feature = "client_tokio", feature = "http_tokio")), feature = "_HTTP"),
    all(feature = "client_tokio", feature = "http_tokio", not(any(feature = "client_std", feature = "http_std")), feature = "_HTTP"),
    // Ignored if no _HTTP feature is enabled
    not(feature = "_HTTP")
)))]
compile_error!(
    "GameDig Feature Compiler Error: Exactly 1 HTTP client type must be enabled, and it must \
     match the runtime client when a `_HTTP` feature is enabled. \n\nEnsure that only one HTTP \
     client type is selected and that it corresponds to the correct runtime client when `_HTTP` \
     is enabled. \n\nExample usage in Cargo.toml:\n\nTo use `http_std` with \
     `client_std`:\n[dependencies]\ngamedig = { version = \"X\", features = [\"_HTTP\", \
     \"client_std\", \"http_std\"] }\n\nTo use `http_tokio` with \
     `client_tokio`:\n[dependencies]\ngamedig = { version = \"X\", default-features = false, \
     features = [\"_HTTP\", \"client_tokio\", \"http_tokio\"] }\n"
);
*/