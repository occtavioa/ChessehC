// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod models;
mod pairing;
mod trf;
mod types;
mod utils;

const BBP_INPUT_FILE_PATH: (BaseDirectory, &str) = (BaseDirectory::AppLocalData, "input");
const BBP_OUTPUT_FILE_PATH: (BaseDirectory, &str) = (BaseDirectory::AppLocalData, "output");
const BBP_PAIRINGS_DIR_PATH: (BaseDirectory, &str) =
    (BaseDirectory::AppLocalData, "bbpPairings-v5.0.1");

use db::{
    create_schema, insert_pairing, insert_player, insert_round, insert_tournament, open_not_create,
    select_ongoing_games, select_pairings, select_pairings_by_round, select_players,
    select_tournament, update_current_round, select_last_inserted_player
};
use models::{ByePoint, Pairing, PairingKind, Player, Tournament, ByeInfo, GameInfo};
use pairing::{execute_bbp, parse_bbp_output};
use rusqlite::Connection;
use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
    path::PathBuf,
};
use tauri::{
    api::path::{resolve_path, BaseDirectory},
    AppHandle, Env, Manager,
};
use trf::{write_configuration, write_players_partial};
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
    let connection = open_not_create(&path).await?;
    Ok(select_tournament(&connection)?)
}

#[tauri::command]
async fn get_players(path: PathBuf) -> Result<Vec<Player>, InvokeErrorBind> {
    let connection = open_not_create(&path).await?;
    Ok(select_players(&connection)?)
}

#[tauri::command]
async fn create_player(path: PathBuf, player: Player) -> Result<Player, InvokeErrorBind> {
    let connection = open_not_create(&path).await?;
    insert_player(&connection, &player)?;
    let player = select_last_inserted_player(&connection)?;
    Ok(player)
}

#[tauri::command]
async fn get_current_round(path: PathBuf) -> Result<Option<u16>, InvokeErrorBind> {
    let connection = open_not_create(&path).await?;
    Ok(select_tournament(&connection)?.current_round)
}

#[tauri::command]
async fn make_pairing(path: PathBuf, app: AppHandle) -> Result<u16, InvokeErrorBind> {
    let connection = open_not_create(&path).await?;
    if !select_ongoing_games(&connection)?.is_empty() {
        return Err(InvokeErrorBind(String::from("Ongoing round")));
    }
    let bbp_input_file_path = get_bbp_input_file_path(&app)?;
    if !bbp_input_file_path.exists() {
        return Err(InvokeErrorBind(String::from("bbpPairings not found")));
    }
    let output_file_path = get_output_file_path(&app)?;
    let bbp_exec_path = get_bbp_exec_path(&app)?;

    let mut players = select_players(&connection)?;
    if players.len() < 2 {
        return Err(InvokeErrorBind(String::from("Not enough players")));
    }
    let mut pairings = select_pairings(&connection)?;
    let Tournament {
        number_rounds,
        current_round,
        ..
    } = select_tournament(&connection)?;

    let bbp_input_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&bbp_input_file_path)?;
    let mut buff = BufWriter::new(bbp_input_file);

    sort_players_initial(&mut players);
    sort_pairings(&mut pairings);

    let players = players;
    
    write_configuration(&mut buff, number_rounds)?;
    write_players_partial(&mut buff, &players, &pairings)?;
    buff.flush()?;
    execute_bbp(&bbp_input_file_path, &bbp_exec_path, &output_file_path).await?;

    let mut output_file = File::open(&output_file_path)?;
    let id_pairs = parse_bbp_output(&mut output_file)?;

    let current_round = current_round.unwrap_or_default() + 1;

    let pairings: Vec<Pairing> = id_pairs
        .iter()
        .map(|ip| Pairing {
            number_round: current_round,
            kind: match ip.1 == 0 {
                true => PairingKind::Bye(ByeInfo {
                    player: players.get(usize::from(ip.0 - 1)).unwrap().clone(),
                    bye_point: ByePoint::U,
                }),
                false => PairingKind::Game(GameInfo {
                    white_player: players.get(usize::from(ip.0 - 1)).unwrap().clone(),
                    black_player: players.get(usize::from(ip.1 - 1)).unwrap().clone(),
                    white_result: None,
                    black_result: None,
                }),
            },
        })
        .collect();

    update_current_round(current_round, &connection)?;
    insert_round(current_round, &connection)?;

    for pairing in pairings {
        insert_pairing(&pairing, &connection)?;
    }
    Ok(current_round)
}

#[tauri::command]
async fn get_pairings_by_round(path: PathBuf, round: u16) -> Result<Vec<Pairing>, InvokeErrorBind> {
    let connection = open_not_create(&path).await?;
    let pairings = select_pairings_by_round(round, &connection)?;
    Ok(pairings)
}

fn get_bbp_input_file_path(app: &AppHandle) -> tauri::api::Result<PathBuf> {
    resolve_path(
        &app.config(),
        &app.package_info(),
        &Env::default(),
        BBP_INPUT_FILE_PATH.1,
        Some(BBP_INPUT_FILE_PATH.0),
    )
}

fn get_output_file_path(app: &AppHandle) -> tauri::api::Result<PathBuf> {
    resolve_path(
        &app.config(),
        &app.package_info(),
        &Env::default(),
        BBP_OUTPUT_FILE_PATH.1,
        Some(BBP_OUTPUT_FILE_PATH.0),
    )
}

fn get_bbp_exec_path(app: &AppHandle) -> tauri::api::Result<PathBuf> {
    let mut exec_path = resolve_path(
        &app.config(),
        &app.package_info(),
        &Env::default(),
        BBP_PAIRINGS_DIR_PATH.1,
        Some(BBP_PAIRINGS_DIR_PATH.0),
    )?;
    exec_path.push("bbpPairings");
    exec_path.set_extension("exe");
    Ok(exec_path)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let output_file_path = get_output_file_path(&app.handle())?;
            if !output_file_path.exists() {
                File::create(&output_file_path)?;
            }
            let bbp_input_file_path = get_bbp_input_file_path(&app.app_handle())?;
            if !bbp_input_file_path.exists() {
                File::create(&bbp_input_file_path)?;
            }
            let bbp_exec_path = get_bbp_exec_path(&app.app_handle())?;
            if !bbp_exec_path.exists() {
                tauri::api::dialog::message(
                    app.get_window("main").as_ref(),
                    "bbpPairings not found",
                    format!("{:?} not found", bbp_exec_path.to_str().unwrap()),
                );
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_tournament,
            pick_tournament_file,
            get_tournament,
            get_players,
            create_player,
            get_current_round,
            make_pairing,
            get_pairings_by_round
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
