pub(crate) use error_stack::Report;
pub(crate) type Result<T> = error_stack::Result<T, ErrorKind>;

pub(crate) mod diagnostic;

macro_rules! define_error {
    (
        $(#[$enum_meta:meta])*
        $enum_name:ident,
        $(
            $(#[$variant_meta:meta])*
            $variant_name:ident
            {
                $(
                    $(#[$field_meta:meta])*
                    $field_name:ident : $field_type:ty
                ),*
                $(,)?
            }
            ($error_message:expr)
        ),+
        $(,)?
    ) => {
        $(#[$enum_meta])*
        #[derive(Debug, thiserror::Error)]
        pub enum $enum_name {
            $(
                $(#[$variant_meta])*
                #[error($error_message)]
                $variant_name {
                    $(
                        $(#[$field_meta])*
                        $field_name: $field_type,
                    )*
                },
            )+
        }
    };
}

macro_rules! define_error_kind {
    (
        $(
            $(#[$enum_meta:meta])*
            $enum_name:ident, {
                $(
                    $(#[$variant_meta:meta])*
                    $variant_name:ident
                    {
                        $(
                            $(#[$field_meta:meta])*
                            $field_name:ident : $field_type:ty
                        ),*
                        $(,)?
                    }
                    ($error_message:expr)
                ),+
                $(,)?
            }
        )+
    ) => {
        $(
            define_error! {
                $(#[$enum_meta])*
                $enum_name,
                $(
                    $(#[$variant_meta])*
                    $variant_name
                    {
                        $(
                            $(#[$field_meta])*
                            $field_name : $field_type
                        ),*
                    }
                    ($error_message)
                ),+
            }
        )+

        #[derive(Debug, thiserror::Error, derive_more::From)]
        pub enum ErrorKind {
            $(
                #[error(transparent)]
                $enum_name($enum_name),
            )+
        }
    };
}

define_error_kind! {
    /// IO Error
    ///
    /// This set of errors are related to IO operations.
    IoError, {
        /// Underflow Error
        ///
        /// This error occurs when there is an attempt to read beyond the available data in the buffer.
        #[cfg(feature = "_BUFFER")]
        UnderflowError {
            /// The number of bytes that were attempted to be read.
            attempted: usize,

            /// The number of bytes that were available to be read.
            available: usize
        }(
            "[GameDig]::[IO::UnderflowError]: Attempted to read {attempted} bytes, but only {available} bytes available."
        ),

        /// String Conversion Error
        ///
        /// This error occurs when a string cannot be converted from a byte slice.
        #[cfg(feature = "_BUFFER")]
        StringConversionError {} (
            "[GameDig]::[IO::StringConversionError]: Failed to convert string"
        )
        
    }

    /// Network Error
    ///
    /// This set of errors are related to network operations.
    NetworkError, {
        /// Network Connection Error
        ///
        /// This error occurs when a connection of some sort to a remote server cannot be established.
        /// This can be due to a variety of reasons, the OS should propagate the true cause.
        ConnectionError {
            /// The network protocol that was used to establish the connection.
            _protocol: diagnostic::metadata::NetworkProtocol,

            /// The address of the remote server that the connection was attempted to.
            addr: std::net::SocketAddr
        }(
            "[GameDig]::[{_protocol}::ConnectionError]: Failed to establish a connection"
        ),

        /// Network Read Error
        ///
        /// This error occurs when data cannot be read from a network stream.
        ReadError {
            /// The network protocol that was used.
            _protocol: diagnostic::metadata::NetworkProtocol,

            /// The address of the remote server that the read operation was attempted to.
            addr: std::net::SocketAddr
        }(
            "[GameDig]::[{_protocol}::ReadError]: Failed to read data"
        ),

        /// Network Write Error
        ///
        /// This error occurs when data cannot be written to a network stream.
        WriteError {
            /// The network protocol that was used.
            _protocol: diagnostic::metadata::NetworkProtocol,

            /// The address of the remote server that the write operation was attempted to.
            addr: std::net::SocketAddr
        }(
            "[GameDig]::[{_protocol}::WriteError]: Failed to write data"
        ),

        /// Network Timeout Elapsed Error
        ///
        /// This error occurs when a timeout elapses while waiting for an operation to complete.
        #[cfg(not(feature = "client_std"))]
        TimeoutElapsedError {
            /// The network protocol that was used.
            _protocol: diagnostic::metadata::NetworkProtocol,

            /// The address of the remote server that the operation was attempted to.
            addr: std::net::SocketAddr
        }(
            "[GameDig]::[{_protocol}::TimeoutElapsedError]: Timeout elapsed while waiting for operation"
        ),

        /// Network Set Timeout Error
        ///
        /// This error occurs when a timeout cannot be set on a network stream.
        /// It's usually due to the duration being equal to zero somehow.
        /// It is an edge case error due to timeout not being managed
        /// within the library itself with the `client_std` feature.
        #[cfg(feature = "client_std")]
        SetTimeoutError {
            /// The network protocol that was used.
            _protocol: diagnostic::metadata::NetworkProtocol,

            /// The address of the remote server that the timeout was attempted to be set on.
            addr: std::net::SocketAddr
        }(
            "[GameDig]::[{_protocol}::SetTimeoutError]: Failed to set timeout"
        )
    }
}
