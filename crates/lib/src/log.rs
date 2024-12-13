/// Defines different logging target names for use with the `log` crate.
///
/// The `EventTarget` struct provides static constants that represent different
/// logging targets. These can be used as target identifiers when logging events
/// within a Rust application that supports the `log` crate API.
pub struct EventTarget;

impl EventTarget {
    /// Logging target for production.
    ///
    /// This target is intended for production environments where minimal logging
    /// is preferred. Use this target to log critical or essential information that
    /// should be recorded in production systems.
    pub const GAMEDIG_PROD: &'static str = "GAMEDIG::PROD";

    /// Logging target for development.
    ///
    /// This target is intended for development environments where verbose logging
    /// is needed to aid debugging and provide more insight into the application flow.
    #[cfg(feature = "_DEV_LOG")]
    pub const GAMEDIG_DEV: &'static str = "GAMEDIG::DEV";
}
