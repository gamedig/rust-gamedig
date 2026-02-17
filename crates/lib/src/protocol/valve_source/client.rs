use {
    super::{
        ExtraData,
        ExtraDataFlag,
        ExtraDataFlags,
        Fragment,
        Info,
        Player,
        Server,
        ServerEnvironment,
        ServerType,
        SourceTV,
        TheShip,
        TheShipMode,
        TheShipPlayer,
        ValveSourceClientError,
    },
    crate::core::{
        Buffer,
        ToSocketAddr,
        UdpClient,
        error::{
            Report,
            ResultExt,
            diagnostic::{ContextComponent, FailureReason},
        },
    },
    bzip2::read::BzDecoder,
    std::{collections::HashMap, io::Read, time::Duration},
};

pub struct ValveSourceClient<
    const MAX_PACKET_SIZE_PLUS_ONE: usize = 1401,
    const MAX_TOTAL_FRAGMENTS: u8 = 35,
> {
    net: UdpClient,

    /// Whether to use "The Ship" server query format.
    ///
    /// Defaults to `false`.
    pub the_ship: bool,

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
impl<const MAX_PACKET_SIZE_PLUS_ONE: usize, const MAX_TOTAL_FRAGMENTS: u8>
    ValveSourceClient<MAX_PACKET_SIZE_PLUS_ONE, MAX_TOTAL_FRAGMENTS>
{
    pub async fn new<A: ToSocketAddr>(addr: A) -> Result<Self, Report<ValveSourceClientError>> {
        Ok(Self {
            net: UdpClient::new(addr, None, None)
                .await
                .change_context(ValveSourceClientError::UdpClientInit)?,

            the_ship: false,
            legacy_split_packet: false,
        })
    }

    pub async fn new_with_timeout<A: ToSocketAddr>(
        addr: A,
        read_timeout: Option<Duration>,
        write_timeout: Option<Duration>,
    ) -> Result<Self, Report<ValveSourceClientError>> {
        Ok(Self {
            net: UdpClient::new(addr, read_timeout, write_timeout)
                .await
                .change_context(ValveSourceClientError::UdpClientInit)?,

            the_ship: false,
            legacy_split_packet: false,
        })
    }

    async fn request(
        &mut self,
        payload: &[u8],
    ) -> Result<Buffer<Vec<u8>>, Report<ValveSourceClientError>> {
        self.net
            .send(payload)
            .await
            .change_context(ValveSourceClientError::UdpRequest)?;

        let mut datagram = [0u8; MAX_PACKET_SIZE_PLUS_ONE];
        let datagram_len = self
            .net
            .recv(&mut datagram)
            .await
            .change_context(ValveSourceClientError::UdpRequest)?;

        if datagram_len == MAX_PACKET_SIZE_PLUS_ONE {
            return Err(Report::new(ValveSourceClientError::SanityCheck {
                name: "datagram length",
            })
            .attach(FailureReason::new(
                "Received a datagram at the maximum allowed size. This strongly suggests \
                 truncation, so the datagram cannot be parsed",
            ))
            .attach(ContextComponent::new(
                "Maximum datagram size",
                MAX_PACKET_SIZE_PLUS_ONE - 1,
            ))
            .attach(ContextComponent::new(
                "Received datagram size (which may be truncated)",
                datagram_len,
            )));
        }

        let mut datagram = Buffer::new(datagram[.. datagram_len].to_vec());

        let datagram_header =
            datagram
                .read_i32_le()
                .change_context(ValveSourceClientError::Parse {
                    section: "datagram",
                    field: "header",
                })?;

        match datagram_header {
            // Single
            -1 => Ok(datagram),

            // Fragmented
            -2 => {
                let id = datagram
                    .read_u32_le()
                    .change_context(ValveSourceClientError::Parse {
                        section: "datagram",
                        field: "id",
                    })?;

                let compression = (id & 0x8000_0000) != 0;

                let total = datagram
                    .read_u8()
                    .change_context(ValveSourceClientError::Parse {
                        section: "datagram",
                        field: "total",
                    })?;

                if total > MAX_TOTAL_FRAGMENTS {
                    return Err(Report::new(ValveSourceClientError::SanityCheck {
                        name: "total fragments",
                    })
                    .attach(FailureReason::new(
                        "Total fragments exceeded the maximum allowed",
                    ))
                    .attach(ContextComponent::new(
                        "Maximum total fragments",
                        MAX_TOTAL_FRAGMENTS,
                    ))
                    .attach(ContextComponent::new("Received total fragments", total)));
                }

                let number = datagram
                    .read_u8()
                    .change_context(ValveSourceClientError::Parse {
                        section: "datagram",
                        field: "number",
                    })?;

                if number != 0 {
                    return Err(Report::new(ValveSourceClientError::SanityCheck {
                        name: "first fragment number",
                    })
                    .attach(FailureReason::new(
                        "First fragment number was expected to be 0",
                    ))
                    .attach(ContextComponent::new("Received", number)));
                }

                if !self.legacy_split_packet {
                    datagram
                        .move_pos(2)
                        .change_context(ValveSourceClientError::Parse {
                            section: "datagram",
                            field: "size",
                        })?
                };

                let decompressed_size =
                    if compression {
                        Some(datagram.read_u32_le().change_context(
                            ValveSourceClientError::Parse {
                                section: "datagram",
                                field: "decompressed_size",
                            },
                        )?)
                    } else {
                        None
                    };

                let crc32 =
                    if compression {
                        Some(datagram.read_u32_le().change_context(
                            ValveSourceClientError::Parse {
                                section: "datagram",
                                field: "crc32",
                            },
                        )?)
                    } else {
                        None
                    };

                let pos = datagram.pos();
                let mut payload = datagram.unpack();
                payload.drain(0 .. pos);

                let mut fragments = Vec::with_capacity((total - 1) as usize);

                for _ in 1 .. total {
                    let mut fragment = [0u8; MAX_PACKET_SIZE_PLUS_ONE];
                    let fragment_len = self
                        .net
                        .recv(&mut fragment)
                        .await
                        .change_context(ValveSourceClientError::UdpRequest)?;

                    if fragment_len == MAX_PACKET_SIZE_PLUS_ONE {
                        return Err(Report::new(ValveSourceClientError::SanityCheck {
                            name: "fragment length",
                        })
                        .attach(FailureReason::new(
                            "Received a fragment at the maximum allowed size. This strongly \
                             suggests truncation, so the fragment cannot be parsed",
                        ))
                        .attach(ContextComponent::new(
                            "Maximum fragment size",
                            MAX_PACKET_SIZE_PLUS_ONE - 1,
                        ))
                        .attach(ContextComponent::new(
                            "Received fragment size (which may be truncated)",
                            fragment_len,
                        )));
                    }

                    let mut fragment = Buffer::new(fragment[.. fragment_len].to_vec());

                    fragment
                        .move_pos(4)
                        .change_context(ValveSourceClientError::Parse {
                            section: "fragment",
                            field: "header",
                        })?;

                    let fragment_id =
                        fragment
                            .read_u32_le()
                            .change_context(ValveSourceClientError::Parse {
                                section: "fragment",
                                field: "id",
                            })?;

                    if fragment_id != id {
                        return Err(Report::new(ValveSourceClientError::SanityCheck {
                            name: "fragment ID",
                        })
                        .attach(FailureReason::new(
                            "Received a fragment with a mismatching ID compared to the initial \
                             fragment",
                        ))
                        .attach(ContextComponent::new("Expected fragment ID", id))
                        .attach(ContextComponent::new("Received fragment ID", fragment_id)));
                    }

                    let fragment_number =
                        fragment
                            .read_u8()
                            .change_context(ValveSourceClientError::Parse {
                                section: "fragment",
                                field: "number",
                            })?;

                    if !self.legacy_split_packet {
                        fragment
                            .move_pos(2)
                            .change_context(ValveSourceClientError::Parse {
                                section: "fragment",
                                field: "size",
                            })?
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

                let remaining_fragments_size = fragments.iter().try_fold(0usize, |acc, f| {
                    acc.checked_add(f.payload.len()).ok_or_else(|| {
                        Report::new(ValveSourceClientError::SanityCheck {
                            name: "reassembled payload size overflow",
                        })
                        .attach(FailureReason::new(
                            "The total size of the reassembled payload exceeded usize::MAX",
                        ))
                        .attach(ContextComponent::new("Current accumulated size", acc))
                        .attach(ContextComponent::new(
                            "Next fragment payload size",
                            f.payload.len(),
                        ))
                        .attach(ContextComponent::new("Fragment index", f.number))
                        .attach(ContextComponent::new("Total fragment count", total))
                    })
                })?;

                payload
                    .try_reserve_exact(remaining_fragments_size)
                    .change_context(ValveSourceClientError::SanityCheck {
                        name: "fragment reassembly capacity reservation",
                    })
                    .attach(FailureReason::new(
                        "Unable to reserve sufficient capacity to reassemble the remaining \
                         fragments",
                    ))
                    .attach(ContextComponent::new(
                        "Current payload capacity",
                        payload.capacity(),
                    ))
                    .attach(ContextComponent::new(
                        "Requested additional capacity",
                        remaining_fragments_size,
                    ))?;

                for fragment in fragments {
                    payload.extend(fragment.payload);
                }

                if compression {
                    // safe unwraps as we are guaranteed to have these if compression is true
                    let decompressed_size = decompressed_size.unwrap();
                    let crc32 = crc32.unwrap();

                    let mut decompressed_payload = Vec::with_capacity(decompressed_size as usize);

                    BzDecoder::new(&*payload)
                        .read_to_end(&mut decompressed_payload)
                        .change_context(ValveSourceClientError::Bzip2Decompress)?;

                    let decompresed_playload_len = decompressed_payload.len();
                    if decompresed_playload_len != decompressed_size as usize {
                        return Err(Report::new(ValveSourceClientError::SanityCheck {
                            name: "decompressed size",
                        })
                        .attach(FailureReason::new(
                            "Decompressed payload size did not match the expected decompressed \
                             size",
                        ))
                        .attach(ContextComponent::new(
                            "Expected decompressed size",
                            decompressed_size,
                        ))
                        .attach(ContextComponent::new(
                            "Actual decompressed size",
                            decompresed_playload_len,
                        )));
                    }

                    let payload_crc32 = crc32fast::hash(&decompressed_payload);
                    if payload_crc32 != crc32 {
                        return Err(Report::new(ValveSourceClientError::SanityCheck {
                            name: "crc32 checksum",
                        })
                        .attach(FailureReason::new(
                            "CRC32 checksum of the decompressed payload did not match the \
                             expected CRC32 checksum",
                        ))
                        .attach(ContextComponent::new("Expected CRC32", crc32))
                        .attach(ContextComponent::new("Actual CRC32", payload_crc32)));
                    }

                    payload = decompressed_payload;
                }

                Ok(Buffer::new(payload))
            }

            _ => {
                return Err(Report::new(ValveSourceClientError::SanityCheck {
                    name: "datagram header",
                })
                .attach(FailureReason::new(
                    "Received an unexpected datagram header value that does not indicate either a \
                     single packet or the start of a fragmented packet sequence",
                ))
                .attach(ContextComponent::new(
                    "Received datagram header",
                    datagram_header,
                )));
            }
        }
    }

    async fn challenge<const L: usize>(
        &mut self,
        buf: &mut Buffer<Vec<u8>>,
        payload: &[u8],
        expected_header: u8,
    ) -> Result<Buffer<Vec<u8>>, Report<ValveSourceClientError>> {
        let challenge = buf
            .read_i32_le()
            .change_context(ValveSourceClientError::Parse {
                section: "challenge",
                field: "value",
            })?;

        let mut challenge_payload = [0u8; L];
        let payload_len = payload.len();

        challenge_payload[.. payload_len].copy_from_slice(payload);
        challenge_payload[payload_len .. payload_len + 4].copy_from_slice(&challenge.to_le_bytes());

        let mut response = self.request(&challenge_payload).await?;

        let response_header = response
            .read_u8()
            .change_context(ValveSourceClientError::Parse {
                section: "challenge response",
                field: "header",
            })?;

        if response_header != expected_header {
            return Err(Report::new(ValveSourceClientError::SanityCheck {
                name: "post challenge response header",
            })
            .attach(FailureReason::new(
                "Received an unexpected response header after a challenge",
            ))
            .attach(ContextComponent::new(
                "Expected response header",
                expected_header,
            ))
            .attach(ContextComponent::new(
                "Actual response header",
                response_header,
            )));
        }

        Ok(response)
    }

    pub async fn query_rules(
        &mut self,
    ) -> Result<HashMap<String, String>, Report<ValveSourceClientError>> {
        const RULES_PAYLOAD: [u8; 5] = [0xFF, 0xFF, 0xFF, 0xFF, 0x56];

        let mut response = self.request(&RULES_PAYLOAD).await?;

        let response_header = response
            .read_u8()
            .change_context(ValveSourceClientError::Parse {
                section: "rules",
                field: "header",
            })?;

        response = match response_header {
            b'E' => response,
            b'A' => {
                const CHALLENGE_LEN: usize = RULES_PAYLOAD.len() + 4;

                self.challenge::<CHALLENGE_LEN>(&mut response, &RULES_PAYLOAD, b'E')
                    .await?
            }

            _ => {
                return Err(Report::new(ValveSourceClientError::SanityCheck {
                    name: "rules response header",
                })
                .attach(FailureReason::new(
                    "Received an unexpected response header for rules query",
                ))
                .attach(ContextComponent::new(
                    "Expected response headers",
                    "'E' (0x45) or 'A' (0x41)",
                ))
                .attach(ContextComponent::new(
                    "Actual response header",
                    response_header,
                )));
            }
        };

        let total = response
            .read_u16_le()
            .change_context(ValveSourceClientError::Parse {
                section: "rules",
                field: "total",
            })?;

        let mut rules = HashMap::with_capacity(total as usize);

        for num in 0 .. total {
            let key = response
                .read_string_utf8(None, true)
                .change_context(ValveSourceClientError::Parse {
                    section: "rules",
                    field: "key",
                })
                .attach(ContextComponent::new("Index", num))?;

            let value = response
                .read_string_utf8(None, true)
                .change_context(ValveSourceClientError::Parse {
                    section: "rules",
                    field: "value",
                })
                .attach(ContextComponent::new("Index", num))?;

            rules.insert(key, value);
        }

        Ok(rules)
    }

    pub async fn query_players(&mut self) -> Result<Vec<Player>, Report<ValveSourceClientError>> {
        const PLAYERS_PAYLOAD: [u8; 5] = [0xFF, 0xFF, 0xFF, 0xFF, 0x55];

        let mut response = self.request(&PLAYERS_PAYLOAD).await?;

        let response_header = response
            .read_u8()
            .change_context(ValveSourceClientError::Parse {
                section: "players",
                field: "header",
            })?;

        response = match response_header {
            b'D' => response,
            b'A' => {
                const CHALLENGE_LEN: usize = PLAYERS_PAYLOAD.len() + 4;

                self.challenge::<CHALLENGE_LEN>(&mut response, &PLAYERS_PAYLOAD, b'D')
                    .await?
            }

            _ => {
                return Err(Report::new(ValveSourceClientError::SanityCheck {
                    name: "players response header",
                })
                .attach(FailureReason::new(
                    "Received an unexpected response header for players query",
                ))
                .attach(ContextComponent::new(
                    "Expected response headers",
                    "'D' (0x44) or 'A' (0x41)",
                ))
                .attach(ContextComponent::new(
                    "Actual response header",
                    response_header,
                )));
            }
        };

        let total = response
            .read_u8()
            .change_context(ValveSourceClientError::Parse {
                section: "players",
                field: "total",
            })?;
        let mut players = Vec::with_capacity(total as usize);

        for _ in 0 .. total {
            let index = response
                .read_u8()
                .change_context(ValveSourceClientError::Parse {
                    section: "players",
                    field: "index",
                })?;

            let name = response.read_string_utf8(None, true).change_context(
                ValveSourceClientError::Parse {
                    section: "players",
                    field: "name",
                },
            )?;
            let score = response
                .read_i32_le()
                .change_context(ValveSourceClientError::Parse {
                    section: "players",
                    field: "score",
                })?;
            let duration =
                response
                    .read_f32_le()
                    .change_context(ValveSourceClientError::Parse {
                        section: "players",
                        field: "duration",
                    })?;

            let the_ship = if self.the_ship {
                let deaths =
                    response
                        .read_i32_le()
                        .change_context(ValveSourceClientError::Parse {
                            section: "players",
                            field: "deaths",
                        })?;
                let money =
                    response
                        .read_i32_le()
                        .change_context(ValveSourceClientError::Parse {
                            section: "players",
                            field: "money",
                        })?;

                Some(TheShipPlayer { deaths, money })
            } else {
                None
            };

            players.push(Player {
                index,
                name,
                score,
                duration,
                the_ship,
            });
        }

        Ok(players)
    }

    pub async fn query_info(&mut self) -> Result<Info, Report<ValveSourceClientError>> {
        const INFO_PAYLOAD: [u8; 25] = [
            0xFF, 0xFF, 0xFF, 0xFF, 0x54, 0x53, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x20, 0x45, 0x6E,
            0x67, 0x69, 0x6E, 0x65, 0x20, 0x51, 0x75, 0x65, 0x72, 0x79, 0x00,
        ];

        let mut response = self.request(&INFO_PAYLOAD).await?;

        let response_header = response
            .read_u8()
            .change_context(ValveSourceClientError::Parse {
                section: "info",
                field: "header",
            })?;

        response = match response_header {
            b'I' => response,
            b'A' => {
                const CHALLENGE_LEN: usize = INFO_PAYLOAD.len() + 4;

                self.challenge::<CHALLENGE_LEN>(&mut response, &INFO_PAYLOAD, b'I')
                    .await?
            }

            _ => {
                return Err(Report::new(ValveSourceClientError::SanityCheck {
                    name: "info response header",
                })
                .attach(FailureReason::new(
                    "Received an unexpected response header for info query",
                ))
                .attach(ContextComponent::new(
                    "Expected response headers",
                    "'I' (0x49) or 'A' (0x41)",
                ))
                .attach(ContextComponent::new(
                    "Actual response header",
                    response_header,
                )));
            }
        };

        let protocol = response
            .read_u8()
            .change_context(ValveSourceClientError::Parse {
                section: "info",
                field: "protocol",
            })?;

        let name = response.read_string_utf8(None, true).change_context(
            ValveSourceClientError::Parse {
                section: "info",
                field: "name",
            },
        )?;

        let map = response.read_string_utf8(None, true).change_context(
            ValveSourceClientError::Parse {
                section: "info",
                field: "map",
            },
        )?;

        let folder = response.read_string_utf8(None, true).change_context(
            ValveSourceClientError::Parse {
                section: "info",
                field: "folder",
            },
        )?;

        let game = response.read_string_utf8(None, true).change_context(
            ValveSourceClientError::Parse {
                section: "info",
                field: "game",
            },
        )?;

        let app_id = response
            .read_u16_le()
            .change_context(ValveSourceClientError::Parse {
                section: "info",
                field: "app_id",
            })?;

        let players = response
            .read_u8()
            .change_context(ValveSourceClientError::Parse {
                section: "info",
                field: "players",
            })?;

        let max_players = response
            .read_u8()
            .change_context(ValveSourceClientError::Parse {
                section: "info",
                field: "max_players",
            })?;

        let bots = response
            .read_u8()
            .change_context(ValveSourceClientError::Parse {
                section: "info",
                field: "bots",
            })?;

        let raw_server_type = response
            .read_u8()
            .change_context(ValveSourceClientError::Parse {
                section: "info",
                field: "server_type",
            })?;

        let server_type = ServerType::from_u8(raw_server_type).ok_or_else(|| {
            Report::new(ValveSourceClientError::SanityCheck {
                name: "server type",
            })
            .attach(FailureReason::new(
                "Received an unrecognized server type value that does not match any known server \
                 types",
            ))
            .attach(ContextComponent::new(
                "Received server type value",
                raw_server_type,
            ))
        })?;

        let raw_environment = response
            .read_u8()
            .change_context(ValveSourceClientError::Parse {
                section: "info",
                field: "environment",
            })?;

        let environment = ServerEnvironment::from_u8(raw_environment).ok_or_else(|| {
            Report::new(ValveSourceClientError::SanityCheck {
                name: "server environment",
            })
            .attach(FailureReason::new(
                "Received an unrecognized server environment value that does not match any known \
                 environments",
            ))
            .attach(ContextComponent::new(
                "Received server environment value",
                raw_environment,
            ))
        })?;

        let password_protected =
            response
                .read_u8()
                .change_context(ValveSourceClientError::Parse {
                    section: "info",
                    field: "password_protected",
                })?
                != 0;

        let vac_enabled = response
            .read_u8()
            .change_context(ValveSourceClientError::Parse {
                section: "info",
                field: "vac_enabled",
            })?
            != 0;

        let mut the_ship = None;

        if self.the_ship {
            let raw_mode = response
                .read_u8()
                .change_context(ValveSourceClientError::Parse {
                    section: "info",
                    field: "the_ship_mode",
                })?;

            let mode = TheShipMode::from_u8(raw_mode).ok_or_else(|| {
                Report::new(ValveSourceClientError::SanityCheck {
                    name: "The Ship game mode",
                })
                .attach(FailureReason::new(
                    "Received an unrecognized The Ship game mode value that does not match any \
                     known game modes",
                ))
                .attach(ContextComponent::new(
                    "Received The Ship game mode value",
                    raw_mode,
                ))
            })?;

            let witnesses = response
                .read_u8()
                .change_context(ValveSourceClientError::Parse {
                    section: "info",
                    field: "the_ship_witnesses",
                })?;

            let duration = response
                .read_u8()
                .change_context(ValveSourceClientError::Parse {
                    section: "info",
                    field: "the_ship_duration",
                })?;

            the_ship = Some(TheShip {
                mode,
                witnesses,
                duration,
            });
        }

        let version = response.read_string_utf8(None, true).change_context(
            ValveSourceClientError::Parse {
                section: "info",
                field: "version",
            },
        )?;

        let edf = ExtraDataFlag(response.read_u8().change_context(
            ValveSourceClientError::Parse {
                section: "info",
                field: "edf",
            },
        )?);

        let mut extra_data = ExtraData {
            port: None,
            server_steam_id: None,
            source_tv: None,
            keywords: None,
            app_id_64: None,
        };

        if edf.contains(ExtraDataFlags::Port) {
            let port = response
                .read_u16_le()
                .change_context(ValveSourceClientError::Parse {
                    section: "info",
                    field: "port",
                })?;

            extra_data.port = Some(port);
        }

        if edf.contains(ExtraDataFlags::SteamID) {
            let server_steam_id =
                response
                    .read_u64_le()
                    .change_context(ValveSourceClientError::Parse {
                        section: "info",
                        field: "server_steam_id",
                    })?;

            extra_data.server_steam_id = Some(server_steam_id);
        }

        if edf.contains(ExtraDataFlags::SourceTV) {
            let source_tv_port =
                response
                    .read_u16_le()
                    .change_context(ValveSourceClientError::Parse {
                        section: "info",
                        field: "source_tv_port",
                    })?;

            let source_tv_name = response.read_string_utf8(None, true).change_context(
                ValveSourceClientError::Parse {
                    section: "info",
                    field: "source_tv_name",
                },
            )?;

            extra_data.source_tv = Some(SourceTV {
                port: source_tv_port,
                name: source_tv_name,
            });
        }

        if edf.contains(ExtraDataFlags::Keywords) {
            let keywords = response.read_string_utf8(None, true).change_context(
                ValveSourceClientError::Parse {
                    section: "info",
                    field: "keywords",
                },
            )?;

            extra_data.keywords = Some(keywords);
        }

        if edf.contains(ExtraDataFlags::GameID) {
            let app_id_64 =
                response
                    .read_u64_le()
                    .change_context(ValveSourceClientError::Parse {
                        section: "info",
                        field: "app_id_64",
                    })?;

            extra_data.app_id_64 = Some(app_id_64);
        }

        Ok(Info {
            protocol,
            name,
            map,
            folder,
            game,
            app_id,
            players,
            max_players,
            bots,
            server_type,
            environment,
            password_protected,
            vac_enabled,
            the_ship,
            version,
            edf,
            extra_data,
        })
    }

    pub async fn query(&mut self) -> Result<Server, Report<ValveSourceClientError>> {
        Ok(Server {
            info: self.query_info().await?,
            players: self.query_players().await.ok(),
            rules: self.query_rules().await.ok(),
        })
    }
}
