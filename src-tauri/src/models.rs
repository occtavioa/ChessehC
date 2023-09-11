use std::{str::{FromStr, from_utf8}, fmt::Display};

use rusqlite::{ToSql, types::{FromSql, FromSqlResult}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Tournament {
    pub name: String,
    pub city: String,
    pub fide_federation: String,
    pub date_start: String,
    pub date_end: String,
    pub type_tournament: String,
    pub format: String,
    pub chief_arbiter: String,
    pub deputy_chief_arbiter: String,
    pub time_control: String,
    pub number_rounds: u16,
    pub current_round: Option<u16>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Player {
    pub name: String,
    pub points: f64,
    pub sex: String,
    pub title: Option<Title>,
    pub rating: u16,
    pub fide_federation: String,
    pub fide_number: Option<String>,
    pub birth_date: String,
}
    
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Title {
    WCM,
    WFM,
    CM,
    WIM,
    FM,
    WGM,
    IM,
    GM,
}

#[derive(Deserialize, Serialize)]
pub enum Pairing {
    Match {
        number_round: u16,
        white_starting_rank: u16,
        black_starting_rank: u16,
        white_result: PlayerResult,
        black_result: PlayerResult,
    },
    Bye {
        number_round: u16,
        starting_rank_player: u16,
        bye_point: ByePoint,
    },
}

#[derive(Deserialize, Serialize)]
pub enum ByePoint {
    H,
    F,
    U,
    Z,
}

#[derive(Deserialize, Serialize)]
pub enum PlayerResult {
    FL,
    FW,
    UW,
    UD,
    UL,
    W,
    D,
    L,
}

impl Display for ByePoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::H => "H",
            Self::F => "F",
            Self::U => "U",
            Self::Z => "Z",
        })
    }
}

impl Display for PlayerResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::FL => "-",
            Self::FW => "+",
            Self::UW => "W",
            Self::UD => "D",
            Self::UL => "L",
            Self::W => "1",
            Self::D => "=",
            Self::L => "0",
        })
    }
}

impl ToString for Title {
    fn to_string(&self) -> String {
        match self {
            Self::GM => String::from("GM"),
            Self::IM => String::from("IM"),
            Self::WGM => String::from("WGM"),
            Self::FM => String::from("FM"),
            Self::WIM => String::from("WIM"),
            Self::CM => String::from("CM"),
            Self::WFM => String::from("WFM"),
            Self::WCM => String::from("WCM"),
        }
    }
}

impl FromStr for Title {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GM" => Ok(Self::GM),
            "IM" => Ok(Self::IM),
            "WGM" => Ok(Self::WGM),
            "FM" => Ok(Self::FM),
            "WIM" => Ok(Self::WIM),
            "CM" => Ok(Self::CM),
            "WFM" => Ok(Self::WFM),
            "WCM" => Ok(Self::WCM),
            _ => Err(String::from("Invalid String"))
        }
    }
}

impl FromSql for Title {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Text(data) => match Title::from_str(from_utf8(data).unwrap()) {
                Ok(t) => Ok(t),
                Err(_) => Err(rusqlite::types::FromSqlError::InvalidType)
            },
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}

impl ToSql for Title {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::Owned(self.to_string().into()))
    }
}
