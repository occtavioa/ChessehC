use rusqlite::{ToSql, types::{FromSql, FromSqlResult, FromSqlError}};
use serde::{Deserialize, Serialize};
use std::str::{FromStr, from_utf8};

pub trait Point {
    fn get_value(&self) -> f64;
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub enum ByePoint {
    U,
    #[default]
    Z,
}

impl Point for ByePoint {
    fn get_value(&self) -> f64 {
        match self {
            Self::U => 1.0,
            Self::Z => 0.0,
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

#[derive(Debug, Deserialize, Serialize)]
pub enum GamePoint {
    W,
    D,
    L,
}

impl Point for GamePoint {
    fn get_value(&self) -> f64 {
        match self {
            Self::W => 1.0,
            Self::D => 1.0 / 2.0,
            Self::L => 0.0,
        }
    }
}

impl ToSql for GamePoint {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}

impl FromSql for GamePoint {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Text(t) => {
                match GamePoint::from_str(from_utf8(t).unwrap_or_default()) {
                    Ok(bp) => Ok(bp),
                    Err(_) => Err(FromSqlError::InvalidType),
                }
            }
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToString for GamePoint {
    fn to_string(&self) -> String {
        match self {
            Self::W => String::from("W"),
            Self::D => String::from("D"),
            Self::L => String::from("L"),
        }
    }
}

impl FromStr for GamePoint {
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
