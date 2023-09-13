use std::str::{from_utf8, FromStr};
use rusqlite::{
    types::{FromSql, FromSqlError, FromSqlResult},
    ToSql,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Tournament {
    pub name: String,
    pub number_rounds: u16,
    pub current_round: Option<u16>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Player {
    pub id: i64,
    pub name: String,
    pub points: f64,
    pub rating: u16,
}

pub struct Pairing {
    pub number_round: u16,
    pub kind: PairingKind,
}

#[derive(Deserialize, Serialize)]
pub enum PairingKind {
    Match {
        white_id: i64,
        black_id: i64,
        white_result: MatchPlayerResult,
        black_result: MatchPlayerResult,
    },
    Bye {
        player_id: i64,
        bye_point: ByePoint,
    },
}

#[derive(Deserialize, Serialize)]
pub enum ByePoint {
    U,
}

#[derive(Deserialize, Serialize)]
pub enum MatchPlayerResult {
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
            rusqlite::types::ValueRef::Text(t) => match ByePoint::from_str(from_utf8(t).unwrap()) {
                Ok(bp) => Ok(bp),
                Err(_) => Err(FromSqlError::InvalidType),
            },
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

impl ToSql for MatchPlayerResult {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}

impl FromSql for MatchPlayerResult {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Text(t) => {
                match MatchPlayerResult::from_str(from_utf8(t).unwrap()) {
                    Ok(mpr) => Ok(mpr),
                    Err(_) => Err(FromSqlError::InvalidType),
                }
            }
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToString for MatchPlayerResult {
    fn to_string(&self) -> String {
        match self {
            Self::W => String::from("W"),
            Self::D => String::from("D"),
            Self::L => String::from("L"),
        }
    }
}

impl FromStr for MatchPlayerResult {
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
