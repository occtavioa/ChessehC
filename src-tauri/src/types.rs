use serde::Deserialize;
use std::io;
use tauri::InvokeError;

use crate::trf;

#[derive(Deserialize)]
pub enum InvokeErrorBind {
    Sqlite(String),
    Io(String),
    Tauri(String),
    Other(String),
}

impl Into<InvokeError> for InvokeErrorBind {
    fn into(self) -> InvokeError {
        self.to_string().into()
    }
}

impl ToString for InvokeErrorBind {
    fn to_string(&self) -> String {
        match self {
            Self::Io(s) => s.into(),
            Self::Other(s) => s.into(),
            Self::Sqlite(s) => s.into(),
            Self::Tauri(s) => s.into(),
        }
    }
}

impl From<trf::Error> for InvokeErrorBind {
    fn from(value: trf::Error) -> Self {
        Self::Other("trf".to_string())
    }
}

impl From<String> for InvokeErrorBind {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<&str> for InvokeErrorBind {
    fn from(value: &str) -> Self {
        Self::Other(value.into())
    }
}

impl From<tauri::api::Error> for InvokeErrorBind {
    fn from(value: tauri::api::Error) -> Self {
        Self::Tauri(value.to_string())
    }
}

impl From<rusqlite::Error> for InvokeErrorBind {
    fn from(value: rusqlite::Error) -> Self {
        Self::Sqlite(value.to_string())
    }
}

impl From<io::Error> for InvokeErrorBind {
    fn from(value: io::Error) -> Self {
        Self::Io(value.to_string())
    }
}
