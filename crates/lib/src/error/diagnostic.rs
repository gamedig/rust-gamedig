use std::{
    env::consts::{ARCH, FAMILY, OS},
    fmt,
};

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
            "\x1B[1m\x1B[33mFailure Reason:\x1B[0m\x1B[1m {}\x1B[0m\n\n",
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
        let group_spaces = Self::WIDTH / Self::GROUP_SIZE - 1;

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

#[allow(dead_code)]
pub(crate) struct CrateInfo {
    version: &'static str,
    debug_build: bool,
}

impl CrateInfo {
    #[allow(dead_code)]
    pub(crate) const fn new() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION"),
            debug_build: cfg!(debug_assertions),
        }
    }
}
impl fmt::Display for CrateInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\x1B[1m\x1B[34mCrate Infomation:\x1B[0m")?;
        writeln!(f, "\x1B[1mVersion         : \x1B[0m{}", self.version)?;
        writeln!(f, "\x1B[1mDebug Build     : \x1B[0m{}", self.debug_build)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct SystemInfo {
    os: &'static str,
    family: &'static str,
    architecture: &'static str,
}

impl SystemInfo {
    #[allow(dead_code)]
    pub(crate) const fn new() -> Self {
        const UNKNOWN: &str = "Unknown";

        Self {
            os: if OS.is_empty() { UNKNOWN } else { OS },
            family: if FAMILY.is_empty() { UNKNOWN } else { FAMILY },
            architecture: if ARCH.is_empty() { UNKNOWN } else { ARCH },
        }
    }
}

impl fmt::Display for SystemInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\x1B[1m\x1B[35mSystem Infomation:\x1B[0m")?;
        writeln!(f, "\x1B[1mOS               : \x1B[0m{}", self.os)?;
        writeln!(f, "\x1B[1mFamily           : \x1B[0m{}", self.family)?;
        writeln!(f, "\x1B[1mArchitecture     : \x1B[0m{}", self.architecture)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_failure_reason_new_and_display() {
        let fr = FailureReason::new("This is a short failure reason.");

        let output = fr.to_string();

        println!("FailureReason output:\n\n{}", output);

        assert!(
            output.contains("Failure Reason:"),
            "Missing header in FailureReason output"
        );

        assert!(
            output.contains("This is a short failure reason."),
            "Missing message in FailureReason output"
        );
    }

    #[test]
    fn test_failure_reason_wrapping() {
        let fr = FailureReason::new(
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor \
             incididunt ut labore et dolore magna aliqua.",
        );

        let output = fr.to_string();

        println!("FailureReason (wrapped) output:\n\n{}", output);

        assert!(
            output.contains('\n'),
            "Expected wrapped output but found no newline"
        );
    }

    #[test]
    fn test_recommendation_new_and_display() {
        let rec =
            Recommendation::new("Try restarting the application to resolve transient issues.");

        let output = rec.to_string();

        println!("Recommendation output:\n\n{}", output);

        assert!(
            output.contains("Recommendation:"),
            "Missing header in Recommendation output"
        );

        assert!(
            output.contains("Try restarting the application"),
            "Missing message in Recommendation output"
        );
    }

    #[test]
    fn test_recommendation_wrapping() {
        let rec = Recommendation::new(
            "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu \
             fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident.",
        );

        let output = rec.to_string();

        println!("Recommendation (wrapped) output:\n\n{}", output);

        assert!(
            output.contains('\n'),
            "Expected wrapping in long Recommendation message"
        );
    }

    #[test]
    fn test_open_github_issue_display() {
        let open_issue = OpenGitHubIssue();

        let output = open_issue.to_string();

        println!("OpenGitHubIssue output:\n\n{}", output);

        assert!(
            output.contains("Bug:"),
            "Missing 'Bug:' marker in OpenGitHubIssue output"
        );

        assert!(
            output.contains("https://github.com/gamedig/rust-gamedig/issues"),
            "Missing GitHub URL in OpenGitHubIssue output"
        );
    }

    #[test]
    fn test_hex_dump_empty() {
        let hd = HexDump::new("Empty", Vec::new(), None);

        let output = hd.to_string();

        println!("HexDump (empty) output:\n\n{}", output);

        assert!(output.contains("Hex Dump:"), "Missing 'Hex Dump:' header");

        assert!(
            output.contains("Name    :   Empty"),
            "Missing or incorrect name"
        );

        assert!(
            !output.contains("00000000:"),
            "Empty hex dump should not show address lines"
        );
    }

    #[test]
    fn test_hex_dump_full_line() {
        let data: Vec<u8> = (0 .. 16).collect();

        let hd = HexDump::new("FullLine", data.clone(), Some(0));

        let output = hd.to_string();

        println!("HexDump (full line) output:\n\n{}", output);

        assert!(
            output.contains("00000000:"),
            "Missing address line for full-line hex dump"
        );

        for byte in data {
            assert!(
                output.contains(&format!("{:02x}", byte)),
                "Missing hex for byte {:02x}",
                byte
            );
        }
    }

    #[test]
    fn test_hex_dump_partial_line() {
        // ASCII "ABCDE"
        let data: Vec<u8> = vec![0x41, 0x42, 0x43, 0x44, 0x45];

        let hd = HexDump::new("PartialLine", data.clone(), Some(4));
        let output = hd.to_string();

        println!("HexDump (partial line) output:\n\n{}", output);

        assert!(
            output.contains("00000000:"),
            "Missing address line for partial hex dump"
        );

        assert!(
            output.contains("ABCDE"),
            "ASCII representation is missing or incorrect"
        );
    }

    #[test]
    fn test_hex_dump_non_ascii() {
        // non-graphic bytes replaced with '.'.
        let data: Vec<u8> = vec![0x00, 0x1F, 0x7F, 0x80, 0xFF];

        let hd = HexDump::new("NonAscii", data.clone(), None);

        let output = hd.to_string();

        println!("HexDump (non-ascii) output:\n\n{}", output);

        assert!(
            output.contains("."),
            "Expected non-ASCII bytes to be represented as '.'"
        );
    }

    #[test]
    fn test_hex_dump_multiple_lines() {
        let data: Vec<u8> = (0 .. 40).collect();

        let hd = HexDump::new("MultiLine", data.clone(), Some(5));

        let output = hd.to_string();

        println!("HexDump (multiple lines) output:\n\n{}", output);

        // 40 bytes / 16 ≈ 2 full lines + 1 partial
        let addr_count = output.matches("000000").count();

        assert!(
            addr_count >= 3,
            "Expected at least 3 address lines, found {}",
            addr_count
        );
    }

    #[test]
    fn test_crate_info_new_and_display() {
        let crate_info = CrateInfo::new();

        let output = crate_info.to_string();

        println!("CrateInfo output:\n\n{}", output);

        assert!(
            output.contains("Crate Infomation:"),
            "Missing header in CrateInfo output"
        );

        assert!(
            output.contains("Version"),
            "Missing version in CrateInfo output"
        );
        
        assert!(
            output.contains("Debug Build"),
            "Missing debug build info in CrateInfo output"
        );
    }

    #[test]
    fn test_system_info_new_and_display() {
        let sys_info = SystemInfo::new();

        let output = sys_info.to_string();

        println!("SystemInfo output:\n\n{}", output);

        assert!(
            output.contains("System Infomation:"),
            "Missing header in SystemInfo output"
        );

        assert!(output.contains("OS"), "Missing OS in SystemInfo output");

        assert!(
            output.contains("Family"),
            "Missing family in SystemInfo output"
        );

        assert!(
            output.contains("Architecture"),
            "Missing architecture in SystemInfo output"
        );
    }
}
