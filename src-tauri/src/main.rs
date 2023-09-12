// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod models;
mod trf;
mod types;
mod utils;

use db::{
    create_schema, insert_player, insert_tournament, open_not_create, select_current_round,
    select_pairings, select_players, select_tournament,
};
use models::{PairingKind, Player, Tournament};
use rusqlite::Connection;
use std::{
    fs::{remove_file, File},
    io::{BufWriter, Write},
    path::PathBuf,
};
use trf::write_players_partial;
use types::InvokeErrorBind;
use utils::{sort_pairings, sort_players_initial};

#[tauri::command]
async fn pick_tournament_file() -> Option<PathBuf> {
    tauri::api::dialog::blocking::FileDialogBuilder::new()
        .add_filter("chessehc tournament file", &["ctf"])
        .pick_file()
}

#[tauri::command]
async fn create_tournament(tournament: Tournament) -> Result<Option<PathBuf>, InvokeErrorBind> {
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
async fn get_tournament(path: PathBuf) -> Result<Tournament, InvokeErrorBind> {
    let connection = open_not_create(&path)?;
    Ok(select_tournament(&connection)?)
}

#[tauri::command]
async fn get_players(path: PathBuf) -> Result<Vec<Player>, InvokeErrorBind> {
    let connection = open_not_create(&path)?;
    Ok(select_players(&connection)?)
}

#[tauri::command]
async fn create_player(path: PathBuf, player: Player) -> Result<Player, InvokeErrorBind> {
    let connection = open_not_create(&path)?;
    insert_player(&connection, &player)?;
    Ok(player)
}

#[tauri::command]
async fn get_current_round(path: PathBuf) -> Result<Option<u16>, InvokeErrorBind> {
    let connection = open_not_create(&path)?;
    Ok(select_current_round(&connection)?)
}

#[tauri::command]
async fn make_pairing(path: PathBuf) -> Result<u16, InvokeErrorBind> {
    let connection = open_not_create(&path)?;
    let mut players = select_players(&connection)?;
    let trf_file_path = PathBuf::from(path.parent().unwrap_or(&path).join("trf"));
    let mut buff = BufWriter::new(File::open(&trf_file_path)?);
    let mut pairings = select_pairings(&connection)?;
    sort_players_initial(&mut players);
    sort_pairings(&mut pairings);
    write_players_partial(&mut buff, &players, &pairings)?;
    buff.flush()?;
    // remove_file(&trf_file_path)?;
    Ok(2)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_tournament,
            pick_tournament_file,
            get_tournament,
            get_players,
            create_player,
            get_current_round,
            make_pairing
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
