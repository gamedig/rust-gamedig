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
    "GameDig Core Compiler Error: Exactly 1 client feature must be enabled: \n`client_std` or \
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
    "GameDig Core Compiler Error: Exactly 1 HTTP client type must be enabled, and it must match \
     the runtime client when a `_HTTP` feature is enabled. \n\nEnsure that only one HTTP client \
     type is selected and that it corresponds to the correct runtime client when `_HTTP` is \
     enabled. \n\nExample usage in Cargo.toml:\n\nTo use `http_std` with \
     `client_std`:\n[dependencies]\ngamedig = { version = \"X\", features = [\"_HTTP\", \
     \"client_std\", \"http_std\"] }\n\nTo use `http_tokio` with \
     `client_tokio`:\n[dependencies]\ngamedig = { version = \"X\", default-features = false, \
     features = [\"_HTTP\", \"client_tokio\", \"http_tokio\"] }\n"
);

// HTTP Client Dependency Check
//
// This check ensures that HTTP client features `http_std` or `http_tokio` can
// only be enabled if a feature with `_HTTP` is also enabled. This prevents
// enabling an HTTP client without the corresponding `_HTTP` feature.
#[cfg(any(
    all(not(feature = "_HTTP"), feature = "http_std"),
    all(not(feature = "_HTTP"), feature = "http_tokio")
))]
compile_error!(
    "GameDig Core Compiler Error: HTTP client features `http_std` or `http_tokio` can only be \
     enabled if a feature with `_HTTP` is enabled. \n\nEnsure that an `_HTTP` feature is enabled \
     when enabling an HTTP client. \n\nExample usage in Cargo.toml:\n\nTo use `http_std` with an \
     `_HTTP` feature:\n[dependencies]\ngamedig = { version = \"X\", features = [\"_HTTP\", \
     \"http_std\"] }\n\nTo use `http_tokio` with an `_HTTP` feature:\n[dependencies]\ngamedig = { \
     version = \"X\", default-features = false, features = [\"_HTTP\", \"http_tokio\"] }\n"
);

// TLS Configuration Check
//
// This check ensures that only one TLS type can be enabled at a time when a
// `_TLS` feature is enabled, and that the TLS type matches the HTTP client
// (`http_std` or `http_tokio`) corresponding to the runtime client
// (`client_std` or `client_tokio`).
#[cfg(not(any(
    // Correct combinations when _TLS is enabled
    all(feature = "_TLS", feature = "client_std", feature = "http_std", feature = "tls_std_rustls", not(any(feature = "tls_std_native", feature = "tls_std_rustls_native_certs", feature = "client_tokio", feature = "http_tokio"))),
    all(feature = "_TLS", feature = "client_std", feature = "http_std", feature = "tls_std_native", not(any(feature = "tls_std_rustls", feature = "tls_std_rustls_native_certs", feature = "client_tokio", feature = "http_tokio"))),
    all(feature = "_TLS", feature = "client_std", feature = "http_std", feature = "tls_std_rustls_native_certs", not(any(feature = "tls_std_rustls", feature = "tls_std_native", feature = "client_tokio", feature = "http_tokio"))),
    all(feature = "_TLS", feature = "client_tokio", feature = "http_tokio", feature = "tls_tokio_default", not(any(feature = "tls_tokio_rustls", feature = "tls_tokio_native", feature = "tls_tokio_native_vendored", feature = "client_std", feature = "http_std"))),
    all(feature = "_TLS", feature = "client_tokio", feature = "http_tokio", feature = "tls_tokio_rustls", not(any(feature = "tls_tokio_default", feature = "tls_tokio_native", feature = "tls_tokio_native_vendored", feature = "client_std", feature = "http_std"))),
    all(feature = "_TLS", feature = "client_tokio", feature = "http_tokio", feature = "tls_tokio_native", not(any(feature = "tls_tokio_default", feature = "tls_tokio_rustls", feature = "tls_tokio_native_vendored", feature = "client_std", feature = "http_std"))),
    all(feature = "_TLS", feature = "client_tokio", feature = "http_tokio", feature = "tls_tokio_native_vendored", not(any(feature = "tls_tokio_default", feature = "tls_tokio_rustls", feature = "tls_tokio_native", feature = "client_std", feature = "http_std"))),
    // Ignored if no _TLS feature is enabled
    not(feature = "_TLS")
)))]
compile_error!(
    "GameDig Core Compiler Error: Exactly 1 TLS type must be enabled when `_TLS` is enabled, and \
     it must match the HTTP client. \n\nEnsure that only one TLS type is selected at a time when \
     using TLS features, and that the TLS type corresponds to the correct HTTP client. \
     \n\nExample usage in Cargo.toml:\n\nTo use `client_std` with `http_std` and \
     `tls_std_rustls`:\n[dependencies]\ngamedig = { version = \"X\", features = [\"_TLS\", \
     \"client_std\", \"http_std\", \"tls_std_rustls\"] }\n\nTo use `client_tokio` with \
     `http_tokio` and `tls_tokio_default`:\n[dependencies]\ngamedig = { version = \"X\", \
     default-features = false, features = [\"_TLS\", \"client_tokio\", \"http_tokio\", \
     \"tls_tokio_default\"] }\n"
);
