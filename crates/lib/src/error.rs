pub(crate) use error_stack::{Report, ResultExt};
pub(crate) type Result<T> = error_stack::Result<T, ErrorKind>;

macro_rules! define_error {
    (
        // Top-level documentation or attribute metadata for the enum itself
        $(#[$enum_meta:meta])*
        $enum_name:ident,

        $(
            // Documentation and attributes for the specific variant
            $(#[$variant_meta:meta])*
            // Name of the variant
            $variant_name:ident
            // Optional fields associated with the variant, including their metadata
            $( { $($(#[$field_meta:meta])* $field_name:ident : $field_type:ty),* } )?
            // Error message string for the variant
            ($error_message:expr)
        ),+
        $(,)?
    ) => {
        // Define the enum with the specified name and apply top-level documentation or attributes
        $(#[$enum_meta])*
        #[derive(Debug, thiserror::Error)]
        pub enum $enum_name {
            $(
                // Apply documentation and attributes to the variant
                $(#[$variant_meta])*
                // The `#[error($error_message)]` attribute from `thiserror` sets the error message
                #[error($error_message)]
                // Define the variant, optionally including its fields with their metadata
                $variant_name
                $( { $($(#[$field_meta])* $field_name: $field_type),* } )?,
            )+
        }
    };
}

#[derive(Debug, thiserror::Error, derive_more::From)]
pub enum ErrorKind {
    #[error(transparent)]
    IoError(IoError),
    #[error(transparent)]
    NetworkError(NetworkError),
}

define_error!(
    IoError,
    UnimplementedError("[GameDig::Io::UnimplementedError] Unimplemented error"),
);

define_error!(
    /// Network Error
    ///
    /// This enum represents errors that can occur when working with network connections.
    NetworkError,

    /// Network Connection Error
    ///
    /// This error occurs when a connection of some sort to a remote server cannot be established.
    /// This can be due to a variety of reasons, the OS should propagate the true cause.
    ConnectionError {
        _protocol: _metadata::NetworkProtocol,
        _interface: _metadata::NetworkInterface
    }(
        "[GameDig]::[{_protocol}::<{_interface}>::ConnectionError]: Failed to establish a connection"
    ),

    /// Network Read Error
    ///
    /// This error occurs when data cannot be read from a network stream.
    ReadError {
        _protocol: _metadata::NetworkProtocol,
        _interface: _metadata::NetworkInterface
    }(
        "[GameDig]::[{_protocol}::<{_interface}>::ReadError]: Failed to read data"
    ),

    /// Network Write Error
    ///
    /// This error occurs when data cannot be written to a network stream.
    WriteError {
        _protocol: _metadata::NetworkProtocol,
        _interface: _metadata::NetworkInterface
    }(
        "[GameDig]::[{_protocol}::<{_interface}>::WriteError]: Failed to write data"
    ),

    /// Network Set Timeout Error
    ///
    /// This error occurs when a timeout cannot be set on a network stream.
    /// Its usally due to the duration being equal to zero somehow.
    /// It is a edge case error due to timeout not being managed
    /// within the library itself with `client_std` feature.
    #[cfg(feature = "client_std")]
    SetTimeoutError {
        _protocol: _metadata::NetworkProtocol,
        _interface: _metadata::NetworkInterface
    }(
        "[GameDig]::[{_protocol}::<{_interface}>::SetTimeoutError]: Failed to set timeout"
    ),
);

pub mod _metadata {
    use derive_more::Display;

    #[derive(Debug, Display)]
    pub enum NetworkProtocol {
        #[display(fmt = "TCP")]
        Tcp,
        #[display(fmt = "UDP")]
        Udp,
        #[display(fmt = "RCON")]
        Rcon,
        #[display(fmt = "HTTP")]
        Http,
        #[display(fmt = "HTTPS")]
        Https,
    }

    #[derive(Debug, Display)]
    pub enum NetworkInterface {
        #[display(fmt = "C")]
        Client,
        #[display(fmt = "SCI")]
        SealedClientInner,
        #[display(fmt = "SCS")]
        SealedClientStd,
        #[display(fmt = "SCT")]
        SealedClientTokio,
        #[display(fmt = "SCAS")]
        SealedClientAsyncStd,
    }
}
