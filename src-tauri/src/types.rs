use serde::{Deserialize, Serialize};
use tauri::InvokeError;

#[derive(Deserialize, Serialize)]
pub enum GetTournamentError {
    PathNotFound,
    DatabaseNotFound,
    TournamentNotFound,
}

#[derive(Deserialize, Serialize)]
pub enum GetPlayersError {
    PathNotFound,
    DatabaseNotFound,
    PlayersAccessError,
}

#[derive(Deserialize, Serialize)]
pub enum CreatePlayerError {
    PathNotFound,
    DatabaseNotFound,
    InsertPlayerError,
}

pub struct RusqliteToInvokeError(rusqlite::Error);

impl Into<InvokeError> for RusqliteToInvokeError {
    fn into(self) -> InvokeError {
        self.0.to_string().into()
    }
}

impl From<rusqlite::Error> for RusqliteToInvokeError {
    fn from(value: rusqlite::Error) -> Self {
        RusqliteToInvokeError(value)
    }
}
