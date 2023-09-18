use rusqlite::{
    types::{FromSql, FromSqlError, FromSqlResult},
    ToSql,
};
use serde::{Deserialize, Serialize};
use std::str::{from_utf8, FromStr};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Tournament {
    pub name: String,
    pub number_rounds: u16,
    pub current_round: Option<u16>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Player {
    pub id: i64,
    pub name: String,
    pub points: f64,
    pub rating: u16,
}   

#[derive(Debug, Deserialize, Serialize)]
pub struct Pairing {
    pub number_round: u16,
    pub kind: PairingKind,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum PairingKind {
    Game (GameInfo),
    Bye (ByeInfo),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameInfo {
    pub white_player: Player,
    pub black_player: Player,
    pub white_result: Option<GamePlayerResult>,
    pub black_result: Option<GamePlayerResult>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ByeInfo {
    pub player: Player,
    pub bye_point: ByePoint,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ByePoint {
    U,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum GamePlayerResult {
    W,
    D,
    L,
}

impl ToSql for ByePoint {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}

impl FromSql for ByePoint {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Text(t) => {
                match ByePoint::from_str(from_utf8(t).unwrap_or_default()) {
                    Ok(bp) => Ok(bp),
                    Err(_) => Err(FromSqlError::InvalidType),
                }
            }
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToString for ByePoint {
    fn to_string(&self) -> String {
        match self {
            Self::U => String::from("U"),
        }
    }
}

impl FromStr for ByePoint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::U),
            _ => Err(String::from("Invalid")),
        }
    }
}

impl ToSql for GamePlayerResult {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}

impl FromSql for GamePlayerResult {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Text(t) => {
                match GamePlayerResult::from_str(from_utf8(t).unwrap()) {
                    Ok(mpr) => Ok(mpr),
                    Err(_) => Err(FromSqlError::InvalidType),
                }
            }
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToString for GamePlayerResult {
    fn to_string(&self) -> String {
        match self {
            Self::W => String::from("W"),
            Self::D => String::from("D"),
            Self::L => String::from("L"),
        }
    }
}

impl FromStr for GamePlayerResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "W" => Ok(Self::W),
            "D" => Ok(Self::D),
            "L" => Ok(Self::L),
            _ => Err(String::from("Invalid")),
        }
    }
}
