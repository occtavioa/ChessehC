// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod models;
mod types;

use db::{create_schema, insert_tournament, select_players, select_tournament, insert_player};
use models::{Player, Tournament};
use rusqlite::Connection;
use std::path::PathBuf;
use types::{GetPlayersError, GetTournamentError, RusqliteToInvokeError, CreatePlayerError};

#[tauri::command]
async fn pick_tournament_file() -> Option<PathBuf> {
    tauri::api::dialog::blocking::FileDialogBuilder::new()
        .add_filter("chessehc tournament file", &["ctf"])
        .pick_file()
}

#[tauri::command]
async fn create_tournament(tournament: Tournament) -> Result<Option<PathBuf>, RusqliteToInvokeError> {
    match tauri::api::dialog::blocking::FileDialogBuilder::new()
        .set_file_name(&tournament.name)
        .add_filter("chessehc tournament file", &["ctf"])
        .save_file()
    {
        None => Ok(None),
        Some(path) => {
            let connection = Connection::open(&path)?;
            create_schema(&connection)?;
            insert_tournament(&tournament, &connection)?;
            Ok(Some(path))
        }
    }
}

#[tauri::command]
async fn get_tournament(path: PathBuf) -> Result<Tournament, GetTournamentError> {
    if !path.exists() {
        return Err(GetTournamentError::PathNotFound);
    }
    match Connection::open(path) {
        Err(_) => Err(GetTournamentError::DatabaseNotFound),
        Ok(connection) => match select_tournament(&connection) {
            Err(_) => Err(GetTournamentError::TournamentNotFound),
            Ok(tournament) => Ok(tournament),
        },
    }
}

#[tauri::command]
async fn get_players(path: PathBuf) -> Result<Vec<Player>, GetPlayersError> {
    if !path.exists() {
        return Err(GetPlayersError::PathNotFound);
    }
    match Connection::open(path) {
        Err(_) => Err(GetPlayersError::DatabaseNotFound),
        Ok(connection) => match select_players(&connection) {
            Err(_) => Err(GetPlayersError::PlayersAccessError),
            Ok(players) => Ok(players),
        },
    }
}

#[tauri::command]
async fn create_player(path: PathBuf, player: Player) -> Result<Player, CreatePlayerError> {
    if !path.exists() {
        return Err(CreatePlayerError::PathNotFound);
    }
    match Connection::open(path) {
        Err(_) => Err(CreatePlayerError::DatabaseNotFound),
        Ok(connection) => match insert_player(&connection, &player) {
            Err(e) => {
                println!("{}", e);
                Err(CreatePlayerError::InsertPlayerError)
            },
            Ok(_) => Ok(player),
        },
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_tournament,
            pick_tournament_file,
            get_tournament,
            get_players,
            create_player
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
