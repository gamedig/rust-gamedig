use std::{net::SocketAddr, time::Duration};

use bzip2::read::BzDecoder;
use serde::de;
use std::io::Read;

use crate::{
    core::{Buffer, UdpClient},
    error::Result,
};

use super::model::Fragment;

pub struct ValveSourceClient {
    net: UdpClient,

    /// Set as `false` by default.
    ///
    /// This is required for some older games which use a legacy split packet format.
    ///
    /// AppIDs which are known to require this to be set to `true` include:
    ///
    /// `[215, 240, 17550, 17700]` when protocol = `7`.
    pub legacy_split_packet: bool,

    /// Maximum payload size for receiving packets.
    ///
    /// Defaults to `1400`.
    pub max_payload_size: usize,
}

#[maybe_async::maybe_async]
impl ValveSourceClient {
    pub async fn new(addr: SocketAddr) -> Result<Self> {
        Ok(Self {
            net: UdpClient::new(addr, None, None).await?,

            legacy_split_packet: false,
            max_payload_size: 1400,
        })
    }

    pub async fn new_with_timeout(
        addr: SocketAddr,
        read_timeout: Option<Duration>,
        write_timeout: Option<Duration>,
    ) -> Result<Self> {
        Ok(Self {
            net: UdpClient::new(addr, read_timeout, write_timeout).await?,

            legacy_split_packet: false,
            max_payload_size: 1400,
        })
    }

    async fn net_send(&mut self, payload: &[u8]) -> Result<Buffer<Vec<u8>>> {
        self.net.send(payload).await?;

        let mut datagram = Vec::with_capacity(1400);
        self.net.recv(&mut datagram).await?;

        let mut datagram = Buffer::new(datagram);

        match datagram.read_i32_le()? {
            // Single
            -1 => Ok(datagram),

            // Fragmented
            -2 => {
                let id = datagram.read_u32_le()?;
                let compression = (id & 0x8000_0000) != 0;

                let total = datagram.read_u8()?;
                if total > 32 {
                    // too many fragments
                    todo!();
                }

                let number = datagram.read_u8()?;
                if number != 0 {
                    // first fragment must be 0
                    todo!();
                }

                let size = if self.legacy_split_packet {
                    1248
                } else {
                    datagram.read_u16_le()?
                };
                if size as usize > self.max_payload_size {
                    // fragment size exceeds allowed limit
                    todo!();
                }

                let decompressed_size = if compression {
                    Some(datagram.read_u32_le()?)
                } else {
                    None
                };

                let crc32 = if compression {
                    Some(datagram.read_u32_le()?)
                } else {
                    None
                };

                let pos = datagram.pos();
                let mut payload = datagram.unpack();
                payload.drain(0 .. pos);

                let mut fragments = Vec::with_capacity((total - 1) as usize);

                for _ in 1 .. total {
                    let mut fragment = Vec::with_capacity(1400);
                    self.net.recv(&mut fragment).await?;

                    let mut fragment = Buffer::new(fragment);

                    // skip header
                    fragment.move_pos(4)?;

                    let fragment_id = fragment.read_u32_le()?;
                    if fragment_id != id {
                        // Fragment ID mismatch
                        todo!()
                    }

                    let fragment_number = fragment.read_u8()?;

                    let fragment_size = if self.legacy_split_packet {
                        1248
                    } else {
                        fragment.read_u16_le()?
                    };
                    if fragment_size as usize > self.max_payload_size {
                        // fragment size exceeds allowed limit
                        todo!();
                    }

                    let fragment_pos = fragment.pos();
                    let mut fragment_payload = fragment.unpack();
                    fragment_payload.drain(0 .. fragment_pos);

                    fragments.push(Fragment {
                        number: fragment_number,
                        payload: fragment_payload,
                    });
                }

                fragments.sort_by_key(|f| f.number);

                payload.reserve(fragments.iter().map(|f| f.payload.len()).sum());

                for fragment in fragments {
                    payload.extend(fragment.payload);
                }

                if compression {
                    // safe unwraps as we are guaranteed to have these if compression is true
                    let decompressed_size = decompressed_size.unwrap();
                    let crc32 = crc32.unwrap();

                    let mut decompressed_payload = Vec::with_capacity(decompressed_size as usize);

                    if BzDecoder::new(&*payload)
                        .read_to_end(&mut decompressed_payload)
                        // map this error to a report later
                        .is_err()
                    {
                        todo!()
                    }

                    if decompressed_payload.len() != decompressed_size as usize {
                        // decompressed size mismatch
                        todo!()
                    }

                    if crc32fast::hash(&decompressed_payload) != crc32 {
                        // crc32 mismatch
                        todo!()
                    }

                    payload = decompressed_payload;
                }

                Ok(Buffer::new(payload))
            }

            // Invalid response
            _ => {
                // Unexpected header value
                todo!()
            }
        }
    }
}
