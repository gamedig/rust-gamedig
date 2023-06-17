use std::collections::HashMap;

use crate::protocols::types::{CommonBorrowedPlayer, CommonBorrowedResponse, CommonPlayer};
use crate::protocols::GenericResponse;
use crate::protocols::{gamespy::VersionedResponse, types::CommonResponse};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Team {
    pub name: String,
    pub score: u16,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Player {
    pub name: String,
    pub score: u16,
    pub ping: u16,
    pub team_index: u16,
}

impl From<Player> for CommonPlayer {
    fn from(p: Player) -> Self {
        CommonPlayer {
            name: p.name,
            score: p.score.into(),
        }
    }
}

impl<'a> From<&'a Player> for CommonBorrowedPlayer<'a> {
    fn from(p: &'a Player) -> Self {
        CommonBorrowedPlayer {
            name: &p.name,
            score: p.score.into(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    pub name: String,
    pub map: String,
    pub has_password: bool,
    pub teams: Vec<Team>,
    pub players_maximum: usize,
    pub players_online: usize,
    pub players_minimum: Option<u8>,
    pub players: Vec<Player>,
    pub unused_entries: HashMap<String, String>,
}

impl From<Response> for GenericResponse {
    fn from(r: Response) -> Self { GenericResponse::GameSpy(VersionedResponse::Two(r)) }
}

impl From<Response> for CommonResponse {
    fn from(r: Response) -> Self {
        CommonResponse {
            name: Some(r.name),
            description: None,
            game: None,
            game_version: None,
            map: Some(r.map),
            players_maximum: r.players_maximum.try_into().unwrap(),
            players_online: r.players_online.try_into().unwrap(),
            players_bots: None,
            has_password: None,
            players: r.players.into_iter().map(Player::into).collect(),
        }
    }
}

impl<'a> From<&'a Response> for CommonBorrowedResponse<'a> {
    fn from(r: &'a Response) -> Self {
        CommonBorrowedResponse {
            name: Some(&r.name),
            description: None,
            game: None,
            game_version: None,
            map: Some(&r.map),
            players_maximum: r.players_maximum.try_into().unwrap(),
            players_online: r.players_online.try_into().unwrap(),
            players_bots: None,
            has_password: None,
            players: r.players.iter().map(|p| p.into()).collect(),
        }
    }
}
