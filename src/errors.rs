use std::{
    backtrace,
    error::Error,
    fmt::{self, Formatter},
};

/// Result of Type and GDError.
pub type GDResult<T> = Result<T, GDRichError>;

/// GameDig Error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GDError {
    /// The received packet was bigger than the buffer size.
    PacketOverflow,
    /// The received packet was shorter than the expected one.
    PacketUnderflow,
    /// The received packet is badly formatted.
    PacketBad,
    /// Couldn't send the packet.
    PacketSend,
    /// Couldn't send the receive.
    PacketReceive,
    /// Couldn't decompress data.
    Decompress,
    /// Couldn't create a socket connection.
    SocketConnect,
    /// Couldn't bind a socket.
    SocketBind,
    /// Invalid input.
    InvalidInput,
    /// The server queried is not the queried game server.
    BadGame(String),
    /// Couldn't automatically query.
    AutoQuery,
    /// A protocol-defined expected format was not met.
    ProtocolFormat,
    /// Couldn't cast a value to an enum.
    UnknownEnumCast,
    /// Couldn't parse a json string.
    JsonParse,
    /// Couldn't parse a value.
    TypeParse,
}

impl fmt::Display for GDError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { write!(f, "{:?}", self) }
}

impl Error for GDError {}

type ErrorSource = Box<dyn std::error::Error + 'static>;

/// Rich gamedig error with backtrace and source
/// ```
/// GDRichError::packet_bad_from_into("Reason packet was bad")
/// ```
pub struct GDRichError {
    pub kind: GDError,
    pub source: Option<ErrorSource>,
    pub backtrace: Option<std::backtrace::Backtrace>,
}

impl From<GDError> for GDRichError {
    fn from(value: GDError) -> Self {
        let backtrace = Some(backtrace::Backtrace::capture());
        Self {
            kind: value,
            source: None,
            backtrace,
        }
    }
}

impl PartialEq for GDRichError {
    fn eq(&self, other: &Self) -> bool { self.kind == other.kind }
}

impl Error for GDRichError {
    fn source(&self) -> Option<&(dyn Error + 'static)> { self.source.as_ref().map(Box::as_ref) }
}

impl fmt::Debug for GDRichError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?}", self.kind)?;
        if let Some(source) = &self.source {
            writeln!(f, "{:?}", source)?;
        }
        if let Some(backtrace) = &self.backtrace {
            writeln!(f, "{:#?}", backtrace)?;
        }
        Ok(())
    }
}

impl fmt::Display for GDRichError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { write!(f, "{:?}", self) }
}

impl GDRichError {
    /// Create a new rich error (with automatic backtrace)
    pub fn new(kind: GDError, source: Option<ErrorSource>) -> Self {
        let backtrace = Some(std::backtrace::Backtrace::capture());
        Self {
            kind,
            source,
            backtrace,
        }
    }
    // Helpers for creating specific kinds of Rich Errors
    pub fn packet_underflow(source: Option<ErrorSource>) -> Self { Self::new(GDError::PacketUnderflow, source) }
    pub fn packet_bad(source: Option<ErrorSource>) -> Self { Self::new(GDError::PacketBad, source) }
    pub fn protocol_format(source: Option<ErrorSource>) -> Self { Self::new(GDError::ProtocolFormat, source) }
    pub fn unknown_enum_cast(source: Option<ErrorSource>) -> Self { Self::new(GDError::UnknownEnumCast, source) }
    pub fn invalid_input(source: Option<ErrorSource>) -> Self { Self::new(GDError::InvalidInput, source) }
    pub fn decompress(source: Option<ErrorSource>) -> Self { Self::new(GDError::Decompress, source) }
    pub fn type_parse(source: Option<ErrorSource>) -> Self { Self::new(GDError::TypeParse, source) }

    // Helpers for converting source types, these were added as needed feel free to
    // add your own
    pub fn packet_underflow_from_into<E: Into<Box<dyn std::error::Error + 'static>>>(source: E) -> Self {
        Self::packet_underflow(Some(source.into()))
    }
    pub fn packet_bad_from_into<E: Into<Box<dyn std::error::Error + 'static>>>(source: E) -> Self {
        Self::packet_bad(Some(source.into()))
    }
    pub fn protocol_format_from_into<E: Into<Box<dyn std::error::Error + 'static>>>(source: E) -> Self {
        Self::protocol_format(Some(source.into()))
    }
    pub fn unknown_enum_cast_from_into<E: Into<Box<dyn std::error::Error + 'static>>>(source: E) -> Self {
        Self::unknown_enum_cast(Some(source.into()))
    }
    pub fn invalid_input_from_into<E: Into<Box<dyn std::error::Error + 'static>>>(source: E) -> Self {
        Self::invalid_input(Some(source.into()))
    }
    pub fn decompress_from_into<E: Into<Box<dyn std::error::Error + 'static>>>(source: E) -> Self {
        Self::decompress(Some(source.into()))
    }
    pub fn type_parse_from_into<E: Into<Box<dyn std::error::Error + 'static>>>(source: E) -> Self {
        Self::type_parse(Some(source.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Testing Ok variant of the GDResult type
    #[test]
    fn test_gdresult_ok() {
        let result: GDResult<u32> = Ok(42);
        assert_eq!(result.unwrap(), 42);
    }

    // Testing Err variant of the GDResult type
    #[test]
    fn test_gdresult_err() {
        let result: GDResult<u32> = Err(GDError::InvalidInput.into());
        assert!(result.is_err());
    }

    // Testing the Display trait for the GDError type
    #[test]
    fn test_display() {
        let error = GDError::PacketOverflow;
        assert_eq!(format!("{}", error), "PacketOverflow");
    }

    // Testing the Error trait for the GDError type
    #[test]
    fn test_error_trait() {
        let error = GDError::PacketBad;
        assert!(error.source().is_none());
    }

    // Testing cloning the GDError type
    #[test]
    fn test_cloning() {
        let error = GDError::BadGame(String::from("MyGame"));
        let cloned_error = error.clone();
        assert_eq!(error, cloned_error);
    }
}
