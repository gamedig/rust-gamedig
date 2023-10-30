use crate::errors::GDErrorKind::PacketBad;
use crate::protocols::types::{ExtendFromPacket, FromPacket, ToPacket};
use crate::GDResult;

use super::{MutatorsAndRules, PacketKind, Player, Players, ServerInfo, Unreal2StringDecoder};

use byteorder::LittleEndian;

type Buffer<'a> = crate::buffer::Buffer<'a, LittleEndian>;
type WriteBuffer = crate::wbuffer::WriteBuffer<LittleEndian>;

/// The first byte of unreal2 requests.
const REQUEST_FLAG: u8 = 0x79;

/// Unreal2 client request.
#[derive(Debug, Clone, PartialEq)]
pub struct PacketRequest {
    /// Should always be 0x79.
    pub request_flag: u8,
    pub padding: [u8; 3],
    pub packet_type: PacketKind,
}

impl From<PacketKind> for PacketRequest {
    fn from(packet_type: PacketKind) -> Self {
        Self {
            request_flag: REQUEST_FLAG,
            padding: [0, 0, 0],
            packet_type,
        }
    }
}

impl ToPacket for PacketRequest {
    fn as_packet(&self) -> GDResult<Vec<u8>> {
        let mut buffer = Vec::with_capacity(std::mem::size_of::<Self>());

        if self.request_flag != REQUEST_FLAG {
            return Err(PacketBad.context("Unreal2 request should start with 0x79"));
        }

        buffer.push(self.request_flag);
        buffer.extend_from_slice(&self.padding);
        buffer.push(self.packet_type as u8);
        Ok(buffer)
    }
}

impl FromPacket for PacketRequest {
    fn from_packet(packet: &[u8]) -> GDResult<Self> {
        let mut buffer = Buffer::new(packet);
        let request_flag = buffer.read()?;

        if request_flag != REQUEST_FLAG {
            return Err(PacketBad.context("Unreal2 request should start with 0x79"));
        }

        Ok(Self {
            request_flag,
            padding: [buffer.read()?, buffer.read()?, buffer.read()?],
            packet_type: PacketKind::try_from(buffer.read::<u8>()?)?,
        })
    }
}

/// Unreal 2 server response.
#[derive(Debug, Clone, PartialEq)]
pub struct PacketResponse<T> {
    pub padding: [u8; 4],
    pub packet_type: PacketKind,
    pub body: T,
}

impl<T: FromPacket> FromPacket for PacketResponse<T> {
    fn from_packet(packet: &[u8]) -> GDResult<Self> {
        let mut buffer = Buffer::new(packet);

        let padding = [
            buffer.read()?,
            buffer.read()?,
            buffer.read()?,
            buffer.read()?,
        ];

        let packet_type = PacketKind::try_from(buffer.read::<u8>()?)?;

        Ok(Self {
            padding,
            packet_type,
            body: T::from_packet(buffer.remaining_bytes())?,
        })
    }
}

impl<'a, T> ExtendFromPacket<'a> for PacketResponse<&'a mut T>
where for<'b> T: ExtendFromPacket<'b, Input = T>
{
    type Input = T;
    type Output = PacketResponse<&'a mut T>;
    fn extend_from_packet(packet: &[u8], input: &'a mut Self::Input) -> GDResult<Self::Output> {
        let mut buffer = Buffer::new(packet);

        let padding = [
            buffer.read()?,
            buffer.read()?,
            buffer.read()?,
            buffer.read()?,
        ];

        let packet_type = PacketKind::try_from(buffer.read::<u8>()?)?;

        T::extend_from_packet(buffer.remaining_bytes(), input)?;

        Ok(Self {
            padding,
            packet_type,
            body: input,
        })
    }
}

impl<T: ToPacket> ToPacket for PacketResponse<T> {
    fn as_packet(&self) -> GDResult<Vec<u8>> {
        let mut buffer = Vec::with_capacity(std::mem::size_of::<Self>() + std::mem::size_of::<T>());
        buffer.extend_from_slice(&self.padding);
        buffer.push(self.packet_type as u8);
        buffer.extend(self.body.as_packet()?);
        Ok(buffer)
    }
}

pub type PacketServerInfo = PacketResponse<ServerInfo>;

impl From<ServerInfo> for PacketServerInfo {
    fn from(value: ServerInfo) -> Self {
        Self {
            padding: [0, 0, 0, 0],
            packet_type: PacketKind::ServerInfo,
            body: value,
        }
    }
}

impl FromPacket for ServerInfo {
    fn from_packet(packet: &[u8]) -> GDResult<Self> {
        let mut buffer = Buffer::new(packet);
        Ok(ServerInfo {
            server_id: buffer.read()?,
            ip: buffer.read_string::<Unreal2StringDecoder>(None)?,
            game_port: buffer.read()?,
            query_port: buffer.read()?,
            name: buffer.read_string::<Unreal2StringDecoder>(None)?,
            map: buffer.read_string::<Unreal2StringDecoder>(None)?,
            game_type: buffer.read_string::<Unreal2StringDecoder>(None)?,
            num_players: buffer.read()?,
            max_players: buffer.read()?,
        })
    }
}

impl ToPacket for ServerInfo {
    fn as_packet(&self) -> GDResult<Vec<u8>> {
        let mut buffer = WriteBuffer::with_capacity(
            std::mem::size_of::<ServerInfo>(), // TODO: Add string lengths
        );

        buffer.write(self.server_id)?;
        buffer.write_string(&self.ip)?;
        buffer.write(self.game_port)?;
        buffer.write(self.query_port)?;
        buffer.write_string(&self.name)?;
        buffer.write_string(&self.map)?;
        buffer.write_string(&self.game_type)?;
        buffer.write(self.num_players)?;
        buffer.write(self.max_players)?;

        // Write string

        Ok(buffer.into_data())
    }
}

impl ExtendFromPacket<'_> for MutatorsAndRules {
    type Input = MutatorsAndRules;
    type Output = ();
    fn extend_from_packet(packet: &[u8], input: &mut Self::Input) -> GDResult<Self::Output> {
        let mut buffer = Buffer::new(packet);
        while buffer.remaining_length() > 0 {
            let key = buffer.read_string::<Unreal2StringDecoder>(None)?;
            let value = buffer.read_string::<Unreal2StringDecoder>(None).ok();

            if key.eq_ignore_ascii_case("mutator") {
                if let Some(value) = value {
                    input.mutators.insert(value);
                }
            } else {
                let rule_vec = input.rules.get_mut(&key);

                let rule_vec = if let Some(rule_vec) = rule_vec {
                    rule_vec
                } else {
                    input.rules.insert(key.clone(), Vec::default());
                    input
                        .rules
                        .get_mut(&key)
                        .expect("Value should be in HashMap after we inserted")
                };

                if let Some(value) = value {
                    rule_vec.push(value);
                }
            }
        }
        Ok(())
    }
}

impl ToPacket for MutatorsAndRules {
    fn as_packet(&self) -> GDResult<Vec<u8>> {
        let mut buffer = WriteBuffer::default();

        for mutator in self.mutators.iter() {
            buffer.write_string("mutator")?;
            buffer.write_string(mutator)?;
        }

        for (key, values) in self.rules.iter() {
            for value in values {
                buffer.write_string(key)?;
                buffer.write_string(value)?;
            }
        }

        Ok(buffer.into_data())
    }
}

pub type PacketMutatorsAndRules<'a> = PacketResponse<&'a mut MutatorsAndRules>;

impl From<MutatorsAndRules> for PacketResponse<MutatorsAndRules> {
    fn from(body: MutatorsAndRules) -> Self {
        Self {
            padding: [0, 0, 0, 0],
            packet_type: PacketKind::MutatorsAndRules,
            body,
        }
    }
}

impl ExtendFromPacket<'_> for Players {
    type Input = Players;
    type Output = ();

    /// Parse a raw buffer of players into the current struct.
    fn extend_from_packet(packet: &[u8], input: &mut Self::Input) -> GDResult<()> {
        let mut buffer = Buffer::new(packet);
        while buffer.remaining_length() > 0 {
            let player = Player {
                id: buffer.read()?,
                name: buffer.read_string::<Unreal2StringDecoder>(None)?,
                ping: buffer.read()?,
                score: buffer.read()?,
                stats_id: buffer.read()?,
            };

            // If ping is 0 the player is a bot
            if player.ping == 0 {
                input.bots.push(player);
            } else {
                input.players.push(player);
            }
        }

        Ok(())
    }
}

impl ToPacket for Players {
    fn as_packet(&self) -> GDResult<Vec<u8>> {
        let mut buffer = WriteBuffer::with_capacity(self.total_len() * std::mem::size_of::<Player>());

        for player in self.players.iter().chain(self.bots.iter()) {
            buffer.write(player.id)?;
            buffer.write_string(&player.name)?;
            buffer.write(player.ping)?;
            buffer.write(player.score)?;
            buffer.write(player.stats_id)?;
        }

        Ok(buffer.into_data())
    }
}

pub type PacketPlayers<'a> = PacketResponse<&'a mut Players>;

impl From<Players> for PacketResponse<Players> {
    fn from(value: Players) -> Self {
        Self {
            padding: [0, 0, 0, 0],
            packet_type: PacketKind::Players,
            body: value,
        }
    }
}
