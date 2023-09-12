use std::str::{from_utf8, FromStr};

use rusqlite::{
    types::{FromSql, FromSqlError, FromSqlResult},
    ToSql,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Tournament {
    pub name: String,
    // pub city: String,
    // pub fide_federation: String,
    // pub date_start: String,
    // pub date_end: String,
    // pub type_tournament: String,
    // pub format: String,
    // pub chief_arbiter: String,
    // pub deputy_chief_arbiter: String,
    // pub time_control: String,
    pub number_rounds: u16,
    pub current_round: Option<u16>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Player {
    pub id: i64,
    pub name: String,
    pub points: f64,
    // pub sex: String,
    // pub title: Option<Title>,
    pub rating: u16,
    // pub fide_federation: String,
    // pub fide_number: Option<String>,
    // pub birth_date: String,
}

// #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
// pub enum Title {
//     WCM,
//     WFM,
//     CM,
//     WIM,
//     FM,
//     WGM,
//     IM,
//     GM,
// }

pub struct Pairing {
    pub number_round: u16,
    pub kind: PairingKind,
}

#[derive(Deserialize, Serialize)]
pub enum PairingKind {
    Match {
        // number_round: u16,
        white_id: i64,
        black_id: i64,
        white_result: MatchPlayerResult,
        black_result: MatchPlayerResult,
    },
    Bye {
        // number_round: u16,
        player_id: i64,
        bye_point: ByePoint,
    },
}

#[derive(Deserialize, Serialize)]
pub enum ByePoint {
    // H,
    // F,
    U,
    // Z,
}

#[derive(Deserialize, Serialize)]
pub enum MatchPlayerResult {
    // FL,
    // FW,
    // UW,
    // UD,
    // UL,
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

// impl Display for MatchPlayerResult {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", match self {
//             // Self::FL => "-",
//             // Self::FW => "+",
//             // Self::UW => "W",
//             // Self::UD => "D",
//             // Self::UL => "L",
//             Self::W => "1",
//             Self::D => "=",
//             Self::L => "0",
//         })
//     }
// }

// impl ToString for Title {
//     fn to_string(&self) -> String {
//         match self {
//             Self::GM => String::from("GM"),
//             Self::IM => String::from("IM"),
//             Self::WGM => String::from("WGM"),
//             Self::FM => String::from("FM"),
//             Self::WIM => String::from("WIM"),
//             Self::CM => String::from("CM"),
//             Self::WFM => String::from("WFM"),
//             Self::WCM => String::from("WCM"),
//         }
//     }
// }

// impl FromStr for Title {
//     type Err = String;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "GM" => Ok(Self::GM),
//             "IM" => Ok(Self::IM),
//             "WGM" => Ok(Self::WGM),
//             "FM" => Ok(Self::FM),
//             "WIM" => Ok(Self::WIM),
//             "CM" => Ok(Self::CM),
//             "WFM" => Ok(Self::WFM),
//             "WCM" => Ok(Self::WCM),
//             _ => Err(String::from("Invalid String"))
//         }
//     }
// }

// impl FromSql for Title {
//     fn column_result(value: rusqlite::types::ValueRef<'_>) -> FromSqlResult<Self> {
//         match value {
//             rusqlite::types::ValueRef::Text(data) => match Title::from_str(from_utf8(data).unwrap()) {
//                 Ok(t) => Ok(t),
//                 Err(_) => Err(rusqlite::types::FromSqlError::InvalidType)
//             },
//             _ => Err(rusqlite::types::FromSqlError::InvalidType),
//         }
//     }
// }

// impl ToSql for Title {
//     fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
//         Ok(rusqlite::types::ToSqlOutput::Owned(self.to_string().into()))
//     }
// }
