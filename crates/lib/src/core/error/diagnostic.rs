use std::fmt;

pub(crate) use gamedig_macros::wrap;

/// A struct representing displayable information.
///
/// This struct is used to wrap any displayable information that can be attached as context to an error report.
/// It includes a name for the context and the inner value, which can be of any type that implements `fmt::Display`.
/// This allows for flexible and informative error reporting by including various types of contextual information in a consistent format.
#[derive(Debug)]
pub(crate) struct ContextComponent<T: fmt::Display> {
    name: &'static str,
    inner: T,
}

impl<T: fmt::Display> ContextComponent<T> {
    pub(crate) const fn new(name: &'static str, inner: T) -> Self { Self { name, inner } }
}

impl<T: fmt::Display> fmt::Display for ContextComponent<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const CYAN: &str = "\x1B[36m";
        const BOLD: &str = "\x1B[1m";
        const RESET: &str = "\x1B[0m";
        const LABEL_WIDTH: usize = 6;

        writeln!(f, "{BOLD}{CYAN}Context:{RESET}")?;
        writeln!(f, "{BOLD}{:<LABEL_WIDTH$}:{RESET} {}", "Name", self.name)?;
        writeln!(
            f,
            "{BOLD}{:<LABEL_WIDTH$}:{RESET} {}",
            "Type",
            std::any::type_name::<T>()
        )?;
        writeln!(f, "{BOLD}{:<LABEL_WIDTH$}:{RESET} {}", "Value", self.inner)?;

        Ok(())
    }
}

/// A struct representing a failure reason.
///
/// This struct is used to describe a specific reason for a failure. It is used
/// as a printable component in an error stack, providing additional context for
/// each error frame within the report.
#[derive(Debug)]
pub(crate) struct FailureReason(&'static str);

impl FailureReason {
    /// Creates a new `FailureReason`.
    ///
    /// # Arguments
    ///
    /// * `msg` - A message describing the failure reason.
    pub(crate) const fn new(msg: &'static str) -> Self { Self(msg) }
}

impl fmt::Display for FailureReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const BOLD: &str = "\x1B[1m";
        const YELLOW: &str = "\x1B[33m";
        const RESET: &str = "\x1B[0m";

        writeln!(
            f,
            "{BOLD}{YELLOW}Failure Reason:{RESET} {BOLD}{}{RESET}",
            self.0
        )?;
        writeln!(f)
    }
}

/// A struct representing a prompt to open a GitHub issue.
///
/// This struct is used to notify the user of a possible bug and suggests
/// opening an issue on GitHub. It is used as a printable component in an
/// error stack when a bug is suspected.
#[derive(Debug)]
pub(crate) struct OpenGitHubIssue();

impl fmt::Display for OpenGitHubIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const BOLD: &str = "\x1B[1m";
        const RED: &str = "\x1B[91m";
        const BLUE: &str = "\x1B[94m";
        const RESET: &str = "\x1B[0m";

        writeln!(
            f,
            "{BOLD}{RED}Bug:{RESET}{BOLD} Uh oh! Looks like you've encountered a possible bug in GameDig.{RESET}"
        )?;
        writeln!(f)?;
        writeln!(
            f,
            "Please open an issue on GitHub with the error you've encountered and the steps to reproduce it."
        )?;
        writeln!(f)?;
        writeln!(
            f,
            "{BLUE}https://github.com/gamedig/rust-gamedig/issues{RESET}"
        )?;
        writeln!(f)?;
        writeln!(f, "Thank you for helping us improve GameDig!")?;

        Ok(())
    }
}

/// A struct representing a hex dump of binary data.
///
/// This struct is used to display a hex dump of binary data for debugging
/// purposes. It is used as a printable component in an error stack to provide
/// detailed information about the binary data being processed, aiding in
/// troubleshooting.
#[cfg(feature = "_BUFFER")]
#[derive(Debug)]
pub(crate) struct HexDump<B: super::super::buffer::Bufferable> {
    name: &'static str,
    inner: B,
    position: Option<usize>,
}

#[cfg(feature = "_BUFFER")]
impl<B: super::super::buffer::Bufferable> HexDump<B> {
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
    pub(crate) fn new(name: &'static str, inner: B, position: Option<usize>) -> Self {
        Self {
            name,
            inner,
            position,
        }
    }
}

#[cfg(feature = "_BUFFER")]
impl<B: super::super::buffer::Bufferable> fmt::Display for HexDump<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const YELLOW: &str = "\x1B[93m";
        const RESET: &str = "\x1B[0m";
        const LABEL_WIDTH: usize = 8;

        let data: &[u8] = self.inner.as_ref();
        let len = data.len();

        writeln!(f, "{YELLOW}Hex Dump:{RESET}")?;
        writeln!(f, "{:<LABEL_WIDTH$}: {}", "Name", self.name)?;
        writeln!(f, "{:<LABEL_WIDTH$}: {} (0x{:x}) bytes", "Length", len, len)?;

        match self.position {
            Some(position) => writeln!(f, "{:<LABEL_WIDTH$}: {}", "Position", position)?,
            None => writeln!(f, "{:<LABEL_WIDTH$}: None", "Position")?,
        }

        writeln!(f)?;

        if data.is_empty() {
            return Ok(());
        }

        for (line_num, line) in data.chunks(Self::WIDTH as usize).enumerate() {
            let address = line_num * Self::WIDTH as usize;
            write!(f, "{address:08x}:   ")?;

            let mut hex_width: u8 = 0;

            for (i, &byte) in line.iter().enumerate() {
                if i > 0 {
                    if i % Self::GROUP_SIZE as usize == 0 {
                        write!(f, "  ")?;
                        hex_width += 2;
                    } else {
                        write!(f, " ")?;
                        hex_width += 1;
                    }
                }

                write!(f, "{byte:02x}")?;
                hex_width += 2;
            }

            let padding = Self::TOTAL_HEX_WIDTH - hex_width;
            if padding > 0 {
                write!(f, "{:width$}", "", width = padding as usize)?;
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

/// A struct representing crate information.
///
/// This struct is used to provide printable information about the crate.
#[derive(Debug)]
pub(crate) struct CrateInfo {
    /// The target triple for which the crate was built.
    target: &'static str,
    /// The version of the crate.
    version: &'static str,
    /// Indicates if the crate is built in debug mode.
    debug_build: bool,
    /// A list of enabled features.
    enabled_features: &'static [&'static str],
}

include!(concat!(env!("OUT_DIR"), "/features.rs"));

impl CrateInfo {
    /// Creates a new `CrateInfo`.
    ///
    /// This function initializes the `CrateInfo` struct with the current crate's
    /// version, build type, and enabled features.
    #[allow(dead_code)]
    pub(crate) const fn new() -> Self {
        Self {
            target: env!("BUILD_TARGET"),
            version: env!("CARGO_PKG_VERSION"),
            debug_build: cfg!(debug_assertions),
            enabled_features: ENABLED_FEATURES,
        }
    }
}

impl fmt::Display for CrateInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const BOLD: &str = "\x1B[1m";
        const BLUE: &str = "\x1B[34m";
        const RESET: &str = "\x1B[0m";
        const WIDTH: usize = 20;

        writeln!(f, "{BOLD}{BLUE}{:<WIDTH$}:{RESET}", "Crate Information")?;
        writeln!(f, "{BOLD}{:<WIDTH$}:{RESET} {}", "Target", self.target)?;
        writeln!(f, "{BOLD}{:<WIDTH$}:{RESET} {}", "Version", self.version)?;
        writeln!(
            f,
            "{BOLD}{:<WIDTH$}:{RESET} {}",
            "Debug Build", self.debug_build
        )?;
        writeln!(
            f,
            "{BOLD}{:<WIDTH$}:{RESET} {:#?}",
            "Features", self.enabled_features
        )?;

        Ok(())
    }
}

pub(crate) const CRATE_INFO: CrateInfo = CrateInfo::new();

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
        let fr = FailureReason::new(wrap!(
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore \
             magna aliqua."
        ));

        let output = fr.to_string();

        println!("FailureReason (wrapped) output:\n\n{}", output);

        assert!(
            output.contains('\n'),
            "Expected wrapped output but found no newline"
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

    #[cfg(feature = "_BUFFER")]
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

    #[cfg(feature = "_BUFFER")]
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

    #[cfg(feature = "_BUFFER")]
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

    #[cfg(feature = "_BUFFER")]
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

    #[cfg(feature = "_BUFFER")]
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
}
