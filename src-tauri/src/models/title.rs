use std::{fmt::Display, str::from_utf8};

use rusqlite::{
    types::{FromSql, FromSqlError},
    ToSql,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
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

pub enum Error {
    ParseError,
}

impl Into<Value> for Title {
    fn into(self) -> Value {
        Value::String(self.to_string())
    }
}

impl ToSql for Title {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::Owned(
            rusqlite::types::Value::Text(self.to_string()),
        ))
    }
}

impl FromSql for Title {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Text(t) => {
                Ok(Title::try_from(t).map_err(|_| FromSqlError::InvalidType)?)
            }
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::WCM => "WCM",
                Self::WFM => "WFM",
                Self::CM => "CM",
                Self::WIM => "WIM",
                Self::FM => "FM",
                Self::WGM => "WGM",
                Self::IM => "IM",
                Self::GM => "GM",
            }
        )
    }
}

impl TryFrom<&str> for Title {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "WCM" => Ok(Self::WCM),
            "WFM" => Ok(Self::WFM),
            "CM" => Ok(Self::CM),
            "WIM" => Ok(Self::WIM),
            "FM" => Ok(Self::FM),
            "WGM" => Ok(Self::WGM),
            "IM" => Ok(Self::IM),
            "GM" => Ok(Self::GM),
            _ => Err(Error::ParseError),
        }
    }
}

impl TryFrom<&[u8]> for Title {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Title::try_from(from_utf8(value).map_err(|_| Error::ParseError)?)
    }
}
