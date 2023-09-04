// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod models;
mod types;

use db::{
    create_schema, insert_player, insert_tournament, open_not_create, select_current_round,
    select_players, select_tournament,
};
use models::{Player, Tournament};
use rusqlite::Connection;
use std::path::PathBuf;
use types::RusqliteToInvokeError;

#[tauri::command]
fn pick_tournament_file() -> Option<PathBuf> {
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
async fn get_tournament(path: PathBuf) -> Result<Tournament, RusqliteToInvokeError> {
    let connection = open_not_create(&path)?;
    Ok(select_tournament(&connection)?)
}

#[tauri::command]
async fn get_players(path: PathBuf) -> Result<Vec<Player>, RusqliteToInvokeError> {
    let connection = open_not_create(&path)?;
    Ok(select_players(&connection)?)
}

#[tauri::command]
async fn create_player(path: PathBuf, player: Player) -> Result<Player, RusqliteToInvokeError> {
    let connection = open_not_create(&path)?;
    insert_player(&connection, &player)?;
    Ok(player)
}

#[tauri::command]
async fn get_current_round(path: PathBuf) -> Result<Option<u16>, RusqliteToInvokeError> {
    let connection = open_not_create(&path)?;
    Ok(select_current_round(&connection)?)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_tournament,
            pick_tournament_file,
            get_tournament,
            get_players,
            create_player,
            get_current_round
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
