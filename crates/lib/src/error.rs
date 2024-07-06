use std::{net::SocketAddr, time::Duration};

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

        // This allows converting from the custom error enum to an error stack report
        impl From<$enum_name> for error_stack::Report<crate::error::ErrorKind> {
            fn from(error: $enum_name) -> Self {
                error_stack::Report::from(crate::error::ErrorKind::$enum_name(error))
            }
        }
    };
}

#[derive(Debug, thiserror::Error)]
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
        /// The protocol used to establish the connection
        protocol: String,
        /// The client in context
        client: String,
        /// The address that the connection was attempted to
        addr: SocketAddr
    }(
        "[GameDig::Net::{protocol}::{client}::ConnectionError] Failed to establish a connection to address: \
         {addr:#?}"
    ),

    /// Network Read Error
    ///
    /// This error occurs when data cannot be read from a network stream.
    ReadError {
        /// The type of protocol used to read data
        protocol: String,
        /// The client in context
        client: String,
        /// The address that the data was attempted to be read from
        addr: SocketAddr
    }(
        "[GameDig::Net::{protocol}::{client}::ReadError] Failed to read data from address: {addr:#?}"
    ),

    /// Network Write Error
    ///
    /// This error occurs when data cannot be written to a network stream.
    WriteError {
        /// The type of protocol used to write data
        protocol: String,
        /// The client in context
        client: String,
        /// The address that the data was attempted to be written to
        addr: SocketAddr
    }(
        "[GameDig::Net::{protocol}::{client}::WriteError] Failed to write data to address: {addr:#?}"
    ),

    /// Network Set Timeout Error
    ///
    /// This error occurs when a timeout cannot be set on a network stream.
    /// Its usally due to the duration being equal to zero somehow.
    /// It is a edge case error due to timeout not being managed 
    /// within the library itself with `client_std` feature.
    #[cfg(feature = "client_std")]
    SetTimeoutError {
        /// The type of protocol used to set the timeout
        protocol: String,
        /// The duration that was attempted to be set
        duration: Duration,
        /// The kind of stream that the timeout was attempted to be set on
        /// (e.g. Read, Write)
        kind: String
    }(
        "[GameDig::Net::{protocol}::client_std::SetTimeoutError::{kind}] Failed to set timeout of \
         {duration:#?} on stream"
    ),

);

