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
    /// Packet Errors
    PacketError, {
        /// Packet Deserialize Error
        ///
        /// This error occurs when a packet cannot be deserialized.
        PacketDeserializeError {} (
            "[GameDig]::[Packet::PacketDeserializeError]: Failed to deserialize packet"
        )
    }

    /// IO Error
    ///
    /// This set of errors are related to IO operations.
    IoError, {
        /// Buffer Position Arithmetic Error
        ///
        /// This error occurs when there is an overflow or underflow in position arithmetic checks.
        #[cfg(feature = "_BUFFER")]
        BufferPositionArithmeticError {
        }(
            "[GameDig]::[IO::BufferPositionArithmeticError]: Overflow or underflow in position arithmetic checks"
        ),

        /// Buffer Moved Out of Bounds Error
        ///
        /// This error occurs when there is an attempt to move the cursor in the buffer
        /// beyond or before the available data.
        #[cfg(feature = "_BUFFER")]
        BufferMovedOutOfBoundsError {
            /// The the number of bytes that were attempted to be moved.
            attempted: isize,

            /// The current position in the buffer.
            position: usize,

            /// The total size of the buffer.
            available: usize
        }(
            "[GameDig]::[IO::BufferMovedOutOfBoundsError]: Attempted to move position by {attempted} bytes from position {position}, but only [0..{available}] is valid."
        ),

        /// Out of Bounds Error
        ///
        /// This error occurs when there is an attempt to read beyond the available data in the buffer.
        #[cfg(feature = "_BUFFER")]
        BufferOutOfBoundsError {
            /// The number of bytes that were attempted to be read.
            attempted: usize,

            /// The number of bytes that were available to be read.
            available: usize
        }(
            "[GameDig]::[IO::BufferOutOfBoundsError]: Attempted to read {attempted} bytes, but only {available} bytes available."
        ),

        /// Invalid Range Error
        ///
        /// This error occurs when an invalid range is provided to a buffer read operation.
        #[cfg(feature = "_BUFFER")]
        BufferRangeInvalidError {
            /// The start of the range that was attempted to be read.
            start: usize,

            /// The end of the range that was attempted to be read.
            end: usize
        }(
            "[GameDig]::[IO::BufferInvalidRangeError]: Invalid range: [{start}..{end}]"
        ),

        #[cfg(feature = "_BUFFER")]
        BufferRangeOverflowError {}(
            "[GameDig]::[IO::BufferRangeOverflowError]: Attempted to read a range that overflows usize"
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
        #[cfg(all(feature = "_TCP", feature = "socket_tokio"))]
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
        #[cfg(all(feature = "_TCP", feature = "socket_std"))]
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

        /// UDP Send Error
        ///
        /// This error occurs when data cannot be sent over a UDP socket.
        #[cfg(feature = "_UDP")]
        UdpSendError {
            /// The address of the remote server that the read operation was attempted to.
            peer_addr: std::net::SocketAddr
        }(
            "[GameDig]::[UDP::SendError]: Failed to send data"
        ),

        /// UDP Recv Error
        ///
        /// This error occurs when data cannot be read from a UDP socket.
        #[cfg(feature = "_UDP")]
        UdpRecvError {
            /// The address of the remote server that the write operation was attempted to.
            peer_addr: std::net::SocketAddr
        }(
            "[GameDig]::[UDP::RecvError]: Failed to read data"
        ),

        /// UDP Timeout Elapsed Error
        ///
        /// This error occurs when a timeout elapses while waiting for an operation to complete.
        #[cfg(all(feature = "_UDP", feature = "socket_tokio"))]
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
        #[cfg(all(feature = "_UDP", feature = "socket_std"))]
        UdpSetTimeoutError {
            /// The address of the remote server that the timeout was attempted to be set on.
            peer_addr: std::net::SocketAddr
        }(
            "[GameDig]::[UDP::SetTimeoutError]: Failed to set timeout"
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    const MOCK_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080);

    fn assert_non_empty_display<T: std::fmt::Display>(error: T) {
        let msg = error.to_string();

        assert!(
            !msg.is_empty(),
            "Expected non-empty error message, got empty string"
        );
    }

    #[test]
    fn test_errors() {
        let packet_err = PacketError::PacketDeserializeError {};
        assert_non_empty_display(&packet_err);

        let kind: ErrorKind = packet_err.into();
        assert_non_empty_display(&kind);

        #[cfg(feature = "_BUFFER")]
        {
            let io_err1 = IoError::BufferOutOfBoundsError {
                attempted: 10,
                available: 5,
            };
            assert_non_empty_display(&io_err1);

            let io_err2 = IoError::BufferRangeInvalidError { start: 15, end: 5 };
            assert_non_empty_display(&io_err2);

            let io_err3 = IoError::BufferRangeOverflowError {};
            assert_non_empty_display(&io_err3);

            let io_err4 = IoError::BufferStringConversionError {};
            assert_non_empty_display(&io_err4);
        }

        #[cfg(all(feature = "_TCP", feature = "socket_tokio"))]
        {
            let tcp_tokio_err = NetworkError::TcpTimeoutElapsedError {
                peer_addr: MOCK_ADDR,
            };

            assert_non_empty_display(&tcp_tokio_err);
        }

        #[cfg(all(feature = "_TCP", feature = "socket_std"))]
        {
            let tcp_std_err = NetworkError::TcpSetTimeoutError {
                peer_addr: MOCK_ADDR,
            };

            assert_non_empty_display(&tcp_std_err);
        }

        #[cfg(feature = "_UDP")]
        {
            let udp_bind_err = NetworkError::UdpBindError {};
            assert_non_empty_display(&udp_bind_err);

            let udp_conn_err = NetworkError::UdpConnectionError {
                peer_addr: MOCK_ADDR,
            };
            assert_non_empty_display(&udp_conn_err);

            let udp_send_err = NetworkError::UdpSendError {
                peer_addr: MOCK_ADDR,
            };
            assert_non_empty_display(&udp_send_err);

            let udp_recv_err = NetworkError::UdpRecvError {
                peer_addr: MOCK_ADDR,
            };
            assert_non_empty_display(&udp_recv_err);

            #[cfg(feature = "socket_tokio")]
            {
                let udp_tokio_err = NetworkError::UdpTimeoutElapsedError {
                    peer_addr: MOCK_ADDR,
                };
                assert_non_empty_display(&udp_tokio_err);
            }

            #[cfg(feature = "socket_std")]
            {
                let udp_std_err = NetworkError::UdpSetTimeoutError {
                    peer_addr: MOCK_ADDR,
                };
                assert_non_empty_display(&udp_std_err);
            }
        }
    }
}
