#[derive(Debug, thiserror::Error)]
pub enum ValveSourceClientError {
    #[error(
        "[GameDig]::[ValveSource::UDP_CLIENT_INIT]: An error occurred while initializing UDP \
         client"
    )]
    UdpClientInit,

    #[error(
        "[GameDig]::[ValveSource::UDP_REQUEST]: An error occurred while performing a UDP request"
    )]
    UdpRequest,

    #[error("[GameDig]::[ValveSource::PARSE]: Failed to parse {section}::{field}")]
    Parse {
        section: &'static str,
        field: &'static str,
    },

    #[error("[GameDig]::[ValveSource::BZIP2_DECOMPRESS]: Failed to decompress bzip2 payload")]
    Bzip2Decompress,

    #[error("[GameDig]::[ValveSource::SANITY_CHECK]: Sanity check failed for {name}")]
    SanityCheck { name: &'static str },
}
