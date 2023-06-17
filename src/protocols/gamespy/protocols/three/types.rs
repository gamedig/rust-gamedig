use crate::protocols::types::{CommonBorrowedPlayer, CommonBorrowedResponse, CommonPlayer};
use crate::protocols::GenericResponse;
use crate::protocols::{gamespy::VersionedResponse, types::CommonResponse};
use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A player’s details.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Player {
    pub name: String,
    pub score: i32,
    pub ping: u16,
    pub team: u8,
    pub deaths: u32,
    pub skill: u32,
}

impl From<Player> for CommonPlayer {
    fn from(p: Player) -> Self {
        CommonPlayer {
            name: p.name,
            score: p.score.try_into().unwrap(), // FIXME: Should pass error
        }
    }
}

impl<'a> From<&'a Player> for CommonBorrowedPlayer<'a> {
    fn from(p: &'a Player) -> Self {
        CommonBorrowedPlayer {
            name: &p.name,
            score: p.score.try_into().unwrap(), // FIXME: Should pass error
        }
    }
}

/// A team's details
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Team {
    pub name: String,
    pub score: i32,
}

/// A query response.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    pub name: String,
    pub map: String,
    pub has_password: bool,
    pub game_type: String,
    pub game_version: String,
    pub players_maximum: usize,
    pub players_online: usize,
    pub players_minimum: Option<u8>,
    pub players: Vec<Player>,
    pub teams: Vec<Team>,
    pub tournament: bool,
    pub unused_entries: HashMap<String, String>,
}

impl From<Response> for GenericResponse {
    fn from(r: Response) -> Self { GenericResponse::GameSpy(VersionedResponse::Three(r)) }
}

impl TryFrom<Response> for CommonResponse {
    type Error = <u64 as TryFrom<usize>>::Error;
    fn try_from(r: Response) -> Result<Self, Self::Error> {
        Ok(CommonResponse {
            name: Some(r.name),
            description: None,
            game: Some(r.game_type),
            game_version: Some(r.game_version),
            map: Some(r.map),
            players_maximum: r.players_maximum.try_into()?,
            players_online: r.players_online.try_into()?,
            players_bots: None,
            has_password: Some(r.has_password),
            players: r.players.into_iter().map(Player::into).collect(),
        })
    }
}

impl<'a> TryFrom<&'a Response> for CommonBorrowedResponse<'a> {
    type Error = <u64 as TryFrom<usize>>::Error;
    fn try_from(r: &'a Response) -> Result<Self, Self::Error> {
        Ok(CommonBorrowedResponse {
            name: Some(&r.name),
            description: None,
            game: Some(&r.game_type),
            game_version: Some(&r.game_version),
            map: Some(&r.map),
            players_maximum: r.players_maximum.try_into()?,
            players_online: r.players_online.try_into()?,
            players_bots: None,
            has_password: Some(r.has_password),
            players: r.players.iter().map(|p| p.into()).collect(),
        })
    }
}
