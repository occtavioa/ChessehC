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
    pub id: i64,
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
    Z,
}

impl ByePoint {
    pub fn get_points(&self) -> f64 {
        match self {
            Self::U => 1.0,
            Self::Z => 0.0
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum GamePlayerResult {
    W,
    D,
    L,
}

impl GamePlayerResult {
    pub fn get_points(&self) -> f64 {
        match self {
            Self::W => 1.0,
            Self::D => 1.0/2.0,
            Self::L => 0.0
        }
    }
}

pub trait Point {
    fn get_value(&self) -> f64;
}

pub struct Round {
    pub id: i64,
    pub number: u16,
    pub date: String,
}

pub struct Game {
    pub id: i64,
    pub round_id: i64,
    pub white_id: i64,
    pub black_id: i64,
    pub state: GameState
}

pub enum GameState {
    Ongoing,
    Finished(GamePoint, GamePoint)
}

impl Game {
    pub fn update_result(&self, white_point: GamePoint, black_point: GamePoint) -> Result<usize, rusqlite::Error> {
        todo!()
    }
}

pub struct Bye {
    pub id: i64,
    pub round_id: i64,
    pub player_id: i64,
    pub bye_point: ByePoint
}

impl Bye {
    pub fn update_point(&self, bye_point: ByePoint) -> Result<usize, rusqlite::Error> {
        todo!()
    }
}

pub enum GamePoint {
    W,
    D,
    L
}

impl Point for GamePoint {
    fn get_value(&self) -> f64 {
        match self {
            Self::W => 1.0,
            Self::D => 1.0/2.0,
            Self::L => 0.0
        }
    }
}

impl Point for ByePoint {
    fn get_value(&self) -> f64 {
        match self {
            Self::U => 1.0,
            Self::Z => 0.0
        }
    }
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
            Self::Z => String::from("Z"),
        }
    }
}

impl FromStr for ByePoint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::U),
            "Z" => Ok(Self::Z),
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
