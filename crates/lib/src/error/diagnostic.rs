use std::fmt;

/// A struct representing a failure reason.
///
/// This struct is used to describe a specific reason for a failure. It is used
/// as a printable component in an error stack, providing additional context for
/// each error frame within the report.
#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct FailureReason(String);

impl FailureReason {
    /// Creates a new `FailureReason`.
    ///
    /// # Arguments
    ///
    /// * `msg` - A message describing the failure reason.
    pub(crate) fn new<T: Into<String>>(msg: T) -> Self {
        FailureReason(
            Into::<String>::into(msg)
                .split_whitespace()
                .fold((String::new(), 0), |(mut acc, len), word| {
                    if len + word.len() > 80 {
                        acc.push('\n');

                        (acc + word, word.len())
                    } else {
                        if !acc.is_empty() {
                            acc.push(' ');
                        }

                        acc.push_str(word);

                        (acc, len + word.len() + 1)
                    }
                })
                .0,
        )
    }
}

impl fmt::Display for FailureReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\x1B[1m\x1B[34mFailure Reason:\x1B[0m\x1B[1m {}\x1B[0m\n\n",
            self.0
        )
    }
}

/// A struct representing a recommendation.
///
/// This struct is used to provide a recommendation or suggestion. It is used
/// as a printable component in an error stack, guiding the user on how to
/// address or mitigate the error.
#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct Recommendation(String);

impl Recommendation {
    /// Creates a new `Recommendation`.
    ///
    /// # Arguments
    ///
    /// * `msg` - A message containing the recommendation.
    pub(crate) fn new<T: Into<String>>(msg: T) -> Self {
        Recommendation(
            Into::<String>::into(msg)
                .split_whitespace()
                .fold((String::new(), 0), |(mut acc, len), word| {
                    if len + word.len() > 80 {
                        acc.push('\n');

                        (acc + word, word.len())
                    } else {
                        if !acc.is_empty() {
                            acc.push(' ');
                        }

                        acc.push_str(word);

                        (acc, len + word.len() + 1)
                    }
                })
                .0,
        )
    }
}

impl fmt::Display for Recommendation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\x1B[1m\x1B[32mRecommendation:\x1B[0m\x1B[1m {}\x1B[0m\n\n",
            self.0
        )
    }
}

/// A struct representing a prompt to open a GitHub issue.
///
/// This struct is used to notify the user of a possible bug and suggests
/// opening an issue on GitHub. It is used as a printable component in an
/// error stack when a bug is suspected.
#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct OpenGitHubIssue();

impl fmt::Display for OpenGitHubIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\x1B[1m\x1B[91mBug:\x1B[0m\x1B[1m Uh oh! Looks like you've encountered a possible bug in GameDig.\n\
            \n\x1B[0mPlease open an issue on GitHub with the error you've encountered and the steps to reproduce it.\n\
            \n\x1B[94mhttps://github.com/gamedig/rust-gamedig/issues\x1B[0m\n\
            \nThank you for helping us improve GameDig!\n"
        )
    }
}

/// A struct representing a hex dump of binary data.
///
/// This struct is used to display a hex dump of binary data for debugging
/// purposes. It is used as a printable component in an error stack to provide
/// detailed information about the binary data being processed, aiding in
/// troubleshooting.
#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct HexDump {
    name: String,
    inner: Vec<u8>,
    position: Option<usize>,
}

impl HexDump {
    /// Number of bytes per line
    const WIDTH: u8 = 16;

    /// Number of bytes per group within a line
    const GROUP_SIZE: u8 = 4;

    /// Replacement character for non-ASCII bytes
    const NON_ASCII: char = '.';

    /// Total width of the hex part
    const TOTAL_HEX_WIDTH: u8 = {
        // Each byte is represented by two hex digits
        let hex_digits = Self::WIDTH * 2;
        // Spaces between bytes
        let byte_spaces = Self::WIDTH - 1;
        // Additional spaces between groups
        let group_spaces = (Self::WIDTH / Self::GROUP_SIZE - 1) * 1;

        hex_digits + byte_spaces + group_spaces
    };

    /// Creates a new `HexDump`.
    ///
    /// # Arguments
    ///
    /// * `name` - A name or label for the data.
    /// * `inner` - The binary data to be displayed as a hex dump.
    /// * `position` - The position of the data when within a buffer.
    #[allow(dead_code)]
    pub(crate) fn new<T: Into<String>>(name: T, inner: Vec<u8>, position: Option<usize>) -> Self {
        Self {
            name: name.into(),
            inner,
            position,
        }
    }
}

impl fmt::Display for HexDump {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\x1B[93mHex Dump:\x1B[0m")?;
        writeln!(f, "Name    :   {}", self.name)?;

        let data = &self.inner;
        let len = data.len();
        writeln!(f, "Length  :   {len} (0x{len:x}) bytes")?;
        writeln!(
            f,
            "Position:   {}",
            if self.position.is_some() {
                self.position.unwrap().to_string()
            } else {
                "None".to_string()
            }
        )?;

        writeln!(f)?;

        if data.is_empty() {
            return Ok(());
        }

        for (line_num, line) in data.chunks(Self::WIDTH as usize).enumerate() {
            let address = line_num * Self::WIDTH as usize;
            write!(f, "{address:08x}:   ")?;

            let mut hex_len = 0;
            for (i, &byte) in line.iter().enumerate() {
                if i > 0 {
                    if i % Self::GROUP_SIZE as usize == 0 {
                        write!(f, "  ")?;

                        hex_len += 2;
                    } else {
                        write!(f, " ")?;

                        hex_len += 1;
                    }
                }

                write!(f, "{byte:02x}")?;

                hex_len += 2;
            }

            let padding_needed = Self::TOTAL_HEX_WIDTH - hex_len;
            if padding_needed > 0 {
                write!(f, "{:width$}", "", width = padding_needed as usize)?;
            }

            write!(f, "   ")?;
            for &byte in line {
                let ch = if byte.is_ascii_graphic() || byte == b' ' {
                    byte as char
                } else {
                    Self::NON_ASCII
                };

                write!(f, "{ch}")?;
            }

            writeln!(f)?;
        }

        writeln!(f)
    }
}
