/// Defines different logging target names for use with the `log` crate.
///
/// The `EventTarget` struct provides static constants that represent different
/// logging targets. These can be used as target identifiers when logging events
/// within a Rust application that supports the `log` crate API.
#[cfg(feature = "attribute_log")]
pub struct EventTarget;

#[cfg(feature = "attribute_log")]
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

/// This module contains macros for logging events at different levels and targets.
#[macro_use]
pub(crate) mod macros {

    /// Emit a `trace` level event to the `production` logging target.
    ///
    /// Only compiled if the `attribute_log` feature is enabled.
    ///
    /// # Example
    /// ```rust
    /// prod_trace!("This is a trace message");
    /// prod_trace!("This is {} example number {}", "trace", 2);
    /// ```
    macro_rules! prod_trace {
    ($($arg:tt)+) => {
        #[cfg(feature = "attribute_log")]
        log::trace!(
            target: crate::log::EventTarget::GAMEDIG_PROD,
            $($arg)+
        );
    };
}

    /// Emit a `debug` level event to the `production` logging target.
    ///
    /// Only compiled if the `attribute_log` feature is enabled.
    ///
    /// # Example
    /// ```rust
    /// prod_debug!("This is a debug message");
    /// prod_debug!("This is {} example number {}", "debug", 2);
    /// ```
    macro_rules! prod_debug {
    ($($arg:tt)+) => {
        #[cfg(feature = "attribute_log")]
        log::debug!(
            target: crate::log::EventTarget::GAMEDIG_PROD,
            $($arg)+
        );
    };
}

    /// Emit an `info` level event to the `production` logging target.
    ///
    /// Only compiled if the `attribute_log` feature is enabled.
    ///
    /// # Example
    /// ```rust
    /// prod_info!("This is an info message");
    /// prod_info!("This is {} example number {}", "info", 2);
    /// ```
    macro_rules! prod_info {
    ($($arg:tt)+) => {
        #[cfg(feature = "attribute_log")]
        log::info!(
            target: crate::log::EventTarget::GAMEDIG_PROD,
            $($arg)+
        );
    };
}

    /// Emit a `warn` level event to the `production` logging target.
    ///
    /// Only compiled if the `attribute_log` feature is enabled.
    macro_rules! prod_warn {
    ($($arg:tt)+) => {
        #[cfg(feature = "attribute_log")]
        log::warn!(
            target: crate::log::EventTarget::GAMEDIG_PROD,
            $($arg)+
        );
    };
}

    /// Emit an `error` level event to the `production` logging target.
    ///
    /// Only compiled if the `attribute_log` feature is enabled.
    ///
    /// # Example
    /// ```rust
    /// prod_error!("This is an error message");
    /// prod_error!("This is {} example number {}", "error", 2);
    /// ```
    macro_rules! prod_error {
    ($($arg:tt)+) => {
        #[cfg(feature = "attribute_log")]
        log::error!(
            target: crate::log::EventTarget::GAMEDIG_PROD,
            $($arg)+
        );
    };
}

    /// Emit a `trace` level event to the `development` logging target.
    ///
    /// Only compiled if the `_DEV_LOG` feature is enabled.
    ///
    /// # Example
    /// ```rust
    /// dev_trace!("This is a trace message");
    /// dev_trace!("This is {} example number {}", "trace", 2);
    /// ```
    macro_rules! dev_trace {
    ($($arg:tt)+) => {
        #[cfg(feature = "_DEV_LOG")]
        log::trace!(
            target: crate::log::EventTarget::GAMEDIG_DEV,
            $($arg)+
        );
    };
}

    /// Emit a `debug` level event to the `development` logging target.
    ///
    /// Only compiled if the `_DEV_LOG` feature is enabled.
    ///
    /// # Example
    /// ```rust
    /// dev_debug!("This is a debug message");
    /// dev_debug!("This is {} example number {}", "debug", 2);
    /// ```
    macro_rules! dev_debug {
    ($($arg:tt)+) => {
        #[cfg(feature = "_DEV_LOG")]
        log::debug!(
            target: crate::log::EventTarget::GAMEDIG_DEV,
            $($arg)+
        );
    };
}

    /// Emit an `info` level event to the `development` logging target.
    ///
    /// Only compiled if the `_DEV_LOG` feature is enabled.
    ///
    /// # Example
    /// ```rust
    /// dev_info!("This is an info message");
    /// dev_info!("This is {} example number {}", "info", 2);
    /// ```
    macro_rules! dev_info {
    ($($arg:tt)+) => {
        #[cfg(feature = "_DEV_LOG")]
        log::info!(
            target: crate::log::EventTarget::GAMEDIG_DEV,
            $($arg)+
        );
    };
}

    /// Emit a `warn` level event to the `development` logging target.
    ///
    /// Only compiled if the `_DEV_LOG` feature is enabled.
    ///
    /// # Example
    /// ```rust
    /// dev_warn!("This is a warning message");
    /// dev_warn!("This is {} example number {}", "warning", 2);
    /// ```
    macro_rules! dev_warn {
    ($($arg:tt)+) => {
        #[cfg(feature = "_DEV_LOG")]
        log::warn!(
            target: crate::log::EventTarget::GAMEDIG_DEV,
            $($arg)+
        );
    };
}

    /// Emit an `error` level event to the `development` logging target.
    ///
    /// Only compiled if the `_DEV_LOG` feature is enabled.
    ///
    /// # Example
    /// ```rust
    /// dev_error!("This is an error message");
    /// dev_error!("This is {} example number {}", "error", 2);
    macro_rules! dev_error {
    ($($arg:tt)+) => {
        #[cfg(feature = "_DEV_LOG")]
        log::error!(
            target: crate::log::EventTarget::GAMEDIG_DEV,
            $($arg)+
        );
    };
}
}
