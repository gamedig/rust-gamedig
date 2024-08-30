use pretty_hex::PrettyHex;

/// A struct representing a failure reason.
///
/// This struct is used to describe a specific reason for a failure. It is used
/// as a printable component in an error stack, providing additional context for
/// each error frame within the report.
#[allow(dead_code)]
#[derive(Debug, derive_more::Display)]
#[display(fmt = "\x1B[1m\x1B[34mFailure Reason:\x1B[0m\x1B[1m {}\x1B[0m\n\n", _0)]
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

/// A struct representing a recommendation.
///
/// This struct is used to provide a recommendation or suggestion. It is used
/// as a printable component in an error stack, guiding the user on how to
/// address or mitigate the error.
#[allow(dead_code)]
#[derive(Debug, derive_more::Display)]
#[display(fmt = "\x1B[1m\x1B[32mRecommendation:\x1B[0m\x1B[1m {}\x1B[0m\n\n", _0)]
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

/// A struct representing a prompt to open a GitHub issue.
///
/// This struct is used to notify the user of a possible bug and suggests
/// opening an issue on GitHub. It is used as a printable component in an
/// error stack when a bug is suspected.
#[allow(dead_code)]
#[derive(Debug, derive_more::Display)]
#[display(
    fmt = "\x1B[1m\x1B[91mBug:\x1B[0m\x1B[1m Uh oh! Looks like you've encountered a possible bug in GameDig.\n\
                   \n\x1B[0mPlease open an issue on GitHub with the error you've encountered and the steps to reproduce it.\n\
                   \n\x1B[94mhttps://github.com/gamedig/rust-gamedig/issues\x1B[0m\n\
                   \nThank you for helping us improve GameDig!\n"
)]
pub(crate) struct OpenGitHubIssue();

/// A struct representing a hex dump of binary data.
///
/// This struct is used to display a hex dump of binary data for debugging
/// purposes. It is used as a printable component in an error stack to provide
/// detailed information about the binary data being processed, aiding in
/// troubleshooting.
#[allow(dead_code)]
#[derive(Debug, derive_more::Display)]
#[display(
    fmt = "\x1B[93mHex Dump:\x1B[0m\nName:   {}\n{:?}\n\n",
    name,
    "inner.hex_dump()"
)]
pub(crate) struct HexDump {
    name: String,
    inner: Vec<u8>,
}

impl HexDump {
    /// Creates a new `HexDump`.
    ///
    /// # Arguments
    ///
    /// * `name` - A name or label for the data.
    /// * `inner` - The binary data to be displayed as a hex dump.
    #[allow(dead_code)]
    pub(crate) fn new<T: Into<String>>(name: T, inner: Vec<u8>) -> Self {
        HexDump {
            name: name.into(),
            inner,
        }
    }
}

pub(crate) mod metadata {
    #[derive(Debug, derive_more::Display)]
    pub enum NetworkProtocol {
        #[display(fmt = "TCP")]
        Tcp,
        #[display(fmt = "UDP")]
        Udp,
    }
}