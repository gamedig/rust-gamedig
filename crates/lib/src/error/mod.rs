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

        #[derive(Debug, thiserror::Error)]
        pub enum ErrorKind {
            $(
                #[error(transparent)]
                $enum_name($enum_name),
            )+
        }

        $(
            impl From<$enum_name> for ErrorKind {
                fn from(error: $enum_name) -> Self {
                    ErrorKind::$enum_name(error)
                }
            }
        )+
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
        BufferUnderflowError {
            /// The number of bytes that were attempted to be read.
            attempted: usize,

            /// The number of bytes that were available to be read.
            available: usize
        }(
            "[GameDig]::[IO::BufferUnderflowError]: Attempted to read {attempted} bytes, but only {available} bytes available."
        ),

        /// String Conversion Error
        ///
        /// This error occurs when a string cannot be converted from a byte slice.
        #[cfg(feature = "_BUFFER")]
        BufferStringConversionError {} (
            "[GameDig]::[IO::BufferStringConversionError]: Failed to convert string"
        )

    }

    /// Network Error
    ///
    /// This set of errors are related to network operations.
    NetworkError, {
        //TODO: Add HTTP errors

        /// TCP Connection Error
        ///
        /// This error occurs when a TCP connection cannot be established.
        #[cfg(feature = "_TCP")]
        TcpConnectionError {
            /// The address of the remote server that the connection was attempted to.
            peer_addr: std::net::SocketAddr
        }(
            "[GameDig]::[TCP::ConnectionError]: Failed to establish a connection"
        ),

        /// TCP Read Error
        ///
        /// This error occurs when data cannot be read from a TCP stream.
        #[cfg(feature = "_TCP")]
        TcpReadError {
            /// The address of the remote server that the read operation was attempted to.
            peer_addr: std::net::SocketAddr
        }(
            "[GameDig]::[TCP::ReadError]: Failed to read data"
        ),

        /// TCP Write Error
        ///
        /// This error occurs when data cannot be written to a TCP stream.
        #[cfg(feature = "_TCP")]
        TcpWriteError {
            /// The address of the remote server that the write operation was attempted to.
            peer_addr: std::net::SocketAddr
        }(
            "[GameDig]::[TCP::WriteError]: Failed to write data"
        ),

        /// TCP Timeout Elapsed Error
        ///
        /// This error occurs when a timeout elapses while waiting for an operation to complete.
        // both _TCP && client_std
        #[cfg(all(feature = "_TCP", feature = "client_tokio"))]
        TcpTimeoutElapsedError {
            /// The address of the remote server that the operation was attempted to.
            peer_addr: std::net::SocketAddr
        }(
            "[GameDig]::[TCP::TimeoutElapsedError]: Timeout elapsed while waiting for operation"
        ),

        /// TCP Set Timeout Error
        ///
        /// This error occurs when a timeout cannot be set on a TCP stream.
        /// It's usually due to the duration being equal to zero somehow.
        #[cfg(all(feature = "_TCP", feature = "client_std"))]
        TcpSetTimeoutError {
            /// The address of the remote server that the timeout was attempted to be set on.
            peer_addr: std::net::SocketAddr
        }(
            "[GameDig]::[TCP::SetTimeoutError]: Failed to set timeout"
        ),

        /// UDP Bind Error
        ///
        /// This error occurs when a UDP socket cannot be bound to an address.
        /// This is usually a OS level error where no ports are available.
        #[cfg(feature = "_UDP")]
        UdpBindError {} (
            "[GameDig]::[UDP::BindError]: Failed to bind to address"
        ),

        /// UDP Connection Error
        ///
        /// This error occurs when a UDP "connection" cannot be established.
        #[cfg(feature = "_UDP")]
        UdpConnectionError {
            /// The address of the remote server that the connection was attempted to.
            peer_addr: std::net::SocketAddr
        }(
            "[GameDig]::[UDP::ConnectionError]: Failed to establish a connection"
        ),

        /// UDP Read Error
        ///
        /// This error occurs when data cannot be read from a UDP socket.
        #[cfg(feature = "_UDP")]
        UdpReadError {
            /// The address of the remote server that the read operation was attempted to.
            peer_addr: std::net::SocketAddr
        }(
            "[GameDig]::[UDP::ReadError]: Failed to read data"
        ),

        /// UDP Write Error
        ///
        /// This error occurs when data cannot be written to a UDP socket.
        #[cfg(feature = "_UDP")]
        UdpWriteError {
            /// The address of the remote server that the write operation was attempted to.
            peer_addr: std::net::SocketAddr
        }(
            "[GameDig]::[UDP::WriteError]: Failed to write data"
        ),

        /// UDP Timeout Elapsed Error
        ///
        /// This error occurs when a timeout elapses while waiting for an operation to complete.
        #[cfg(all(feature = "_UDP", feature = "client_tokio"))]
        UdpTimeoutElapsedError {
            /// The address of the remote server that the operation was attempted to.
            peer_addr: std::net::SocketAddr
        }(
            "[GameDig]::[UDP::TimeoutElapsedError]: Timeout elapsed while waiting for operation"
        ),

        /// UDP Set Timeout Error
        ///
        /// This error occurs when a timeout cannot be set on a UDP socket.
        /// It's usually due to the duration being equal to zero somehow.
        #[cfg(all(feature = "_UDP", feature = "client_std"))]
        UdpSetTimeoutError {
            /// The address of the remote server that the timeout was attempted to be set on.
            peer_addr: std::net::SocketAddr
        }(
            "[GameDig]::[UDP::SetTimeoutError]: Failed to set timeout"
        ),
    }
}
