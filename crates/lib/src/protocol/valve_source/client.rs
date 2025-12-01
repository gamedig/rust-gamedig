use std::{net::SocketAddr, time::Duration};

use bzip2::read::BzDecoder;
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
}

#[maybe_async::maybe_async]
impl ValveSourceClient {
    pub async fn new(addr: SocketAddr) -> Result<Self> {
        Ok(Self {
            net: UdpClient::new(addr, None, None).await?,

            legacy_split_packet: false,
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
                let number = datagram.read_u8()?;

                // skip size
                if !self.legacy_split_packet {
                    datagram.move_pos(2)?;
                };

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

                let mut fragments: Vec<Fragment> = Vec::with_capacity(total as usize);
                fragments.push(Fragment { number, payload });

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

                    // skip size
                    if !self.legacy_split_packet {
                        fragment.move_pos(2)?;
                    };

                    let fragment_pos = fragment.pos();
                    let mut fragment_payload = fragment.unpack();
                    fragment_payload.drain(0 .. fragment_pos);

                    fragments.push(Fragment {
                        number: fragment_number,
                        payload: fragment_payload,
                    });
                }

                fragments.sort_by_key(|f| f.number);

                let mut final_payload =
                    Vec::with_capacity(fragments.iter().map(|f| f.payload.len()).sum());

                for fragment in fragments {
                    final_payload.extend_from_slice(&fragment.payload);
                }

                if compression {
                    let mut decoder = BzDecoder::new(&*final_payload);
                    let mut decompressed_payload =
                        Vec::with_capacity(decompressed_size.unwrap() as usize);

                    if decoder.read_to_end(&mut decompressed_payload).is_err() {
                        todo!()
                    }

                    if crc32fast::hash(&decompressed_payload) != crc32.unwrap() {
                        todo!()
                    }

                    final_payload = decompressed_payload;
                }

                Ok(Buffer::new(final_payload))
            }

            // Invalid response
            _ => {
                // Unexpected header value
                todo!()
            }
        }
    }
}
