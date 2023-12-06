// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod models;
mod pairing;
mod trf;
mod types;
mod utils;

const BBP_INPUT_FILE_PATH: (BaseDirectory, &str) = (BaseDirectory::Desktop, "input");
const BBP_OUTPUT_FILE_PATH: (BaseDirectory, &str) = (BaseDirectory::Desktop, "output");
const BBP_PAIRINGS_DIR_PATH: (BaseDirectory, &str) = (BaseDirectory::Desktop, "bbpPairings-v5.0.1");

use db::{create_schema, insert_tournament, open_not_create, select_tournament};
use models::{
    bye::Bye,
    game::{Game, GameState},
    player::{self, Player},
    point::{ByePoint, GamePoint},
    round::Round,
    tournament::Tournament,
};
use pairing::{execute_bbp, parse_bbp_output};
use rusqlite::Connection;
use serde_json::{Value, Map};
use std::{
    fs::{remove_file, File, OpenOptions},
    io::{BufWriter, Write},
    path::{Path, PathBuf}, sync::Mutex,
};
use tauri::{
    api::{path::{resolve_path, BaseDirectory}, http::{ClientBuilder, Client, HttpRequestBuilder, Body}},
    AppHandle, Env, Manager, State,
};
use trf::{get_partial_players_lines, get_players_lines};
use types::InvokeErrorBind;
use utils::{helper, sort_players_initial, sort_players_ranked};

#[tauri::command]
async fn pick_tournament_file() -> Option<PathBuf> {
    tauri::api::dialog::blocking::FileDialogBuilder::new()
        .add_filter("chessehc tournament file", &["ctf"])
        .pick_file()
}

#[tauri::command]
async fn create_tournament(tournament: Tournament) -> Result<Option<PathBuf>, InvokeErrorBind> {
    if tournament.name.is_empty() || tournament.number_rounds < 5 {
        return Err("Invalid input".into());
    }
    match tauri::api::dialog::blocking::FileDialogBuilder::new()
        .set_file_name(&tournament.name)
        .add_filter("chessehc tournament file", &["ctf"])
        .save_file()
    {
        None => Ok(None),
        Some(path) => {
            if path.exists() {
                remove_file(&path)?;
            }
            File::create(&path)?;
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
    let tournament = select_tournament(&connection)?;
    Ok(tournament.get_players(&connection)?)
}

#[tauri::command]
async fn add_player(path: PathBuf, player: Player) -> Result<Player, InvokeErrorBind> {
    let connection: Connection = open_not_create(&path).await?;
    let tournament: Tournament = select_tournament(&connection)?;
    tournament.add_player(&player, &connection)?;
    let mut player: Player = tournament.get_last_added_player(&connection)?;
    let rounds: Vec<Round> = tournament.get_rounds(&connection)?;
    rounds
        .iter()
        .map(|r: &Round| {
            let bye: Bye = Bye::new(player.id, ByePoint::Z);
            r.add_bye(&bye, &connection)?;
            player.sum_point(&ByePoint::Z, &connection)?;
            r.add_player_state(&player, &connection)?;
            Ok(())
        })
        .collect::<Result<(), rusqlite::Error>>()?;
    Ok::<Player, InvokeErrorBind>(player)
}

#[tauri::command]
async fn get_current_round(path: PathBuf) -> Result<Option<Round>, InvokeErrorBind> {
    let connection = open_not_create(&path).await?;
    Ok(select_tournament(&connection)?.get_current_round(&connection)?)
}

#[tauri::command]
async fn make_pairing(path: PathBuf, app: AppHandle, client: State<'_, Client>) -> Result<u16, InvokeErrorBind> {
    let connection: Connection = open_not_create(&path).await?;
    let bbp_exec_path: PathBuf = get_bbp_exec_path(&app)?;
    if !bbp_exec_path.exists() {
        return Err("bbpPairings not found".into());
    }
    let bbp_input_file_path: PathBuf = get_bbp_input_file_path(&app)?;
    let bbp_output_file_path: PathBuf = get_bbp_output_file_path(&app)?;

    let tournament: Tournament = select_tournament(&connection)?;
    if let Some(r) = &tournament.get_current_round(&connection)? {
        let games: Vec<Game> = r.get_games(&connection)?;
        if !games
            .iter()
            .filter(|&g| matches!(g.state, GameState::Ongoing))
            .collect::<Vec<&Game>>()
            .is_empty()
        {
            return Err("Ongoing round".into());
        }
        if r.number >= tournament.number_rounds {
            return Err("Number of rounds reached".into());
        }
    }
    let mut players = tournament.get_players(&connection)?;
    if players.len() < 2 {
        return Err("Not enough players".into());
    }
    sort_players_initial(&mut players);
    let players: Vec<(u16, Player)> = players
        .into_iter()
        .enumerate()
        .map(|(i, p)| (i as u16 + 1, p))
        .collect();

    let rounds = tournament.get_rounds(&connection)?;

    let trf_players_lines: Vec<String> = get_partial_players_lines(&players, &rounds, &connection)?;
    let trf_config: String = tournament.get_partial_trf_config();
    let mut buff_input = BufWriter::new(
        OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(&bbp_input_file_path)?,
    );
    buff_input.write(format! {"{}\r\n", trf_config}.as_bytes())?;
    buff_input.write(format! {"{}\r\n", trf_players_lines.join("\r\n")}.as_bytes())?;
    buff_input.flush()?;

    let output = execute_bbp(&bbp_input_file_path, &bbp_exec_path, &bbp_output_file_path).await?;

    match output.code() {
        Some(0) => {}
        Some(1) => return Err("No valid pairing".into()),
        Some(3) => return Err("Invalid request".into()),
        Some(4) => return Err("Data size could not be handled".into()),
        Some(5) => return Err("Error on file acces".into()),
        _ => return Err("Unexpected error".into()),
    }

    let mut output_file = File::open(&bbp_output_file_path)?;
    let id_pairs: Vec<(u16, u16)> = parse_bbp_output(&mut output_file)?;
    if id_pairs.is_empty() {
        return Err("No valid pairing".into());
    }

    let (game_pairs, bye_pairs) = id_pairs.split_at(
        id_pairs
            .iter()
            .enumerate()
            .find(|(_, p)| p.1 == 0)
            .unwrap_or((id_pairs.len(), &(0, 0)))
            .0,
    );
    let mut games: Vec<Game> = game_pairs
        .iter()
        .map(|&(w, b)| {
            let white = &players
                .iter()
                .find(|(i, _)| *i == w)
                .ok_or(format!("Invalid white id {w}"))?
                .1;
            let black = &players
                .iter()
                .find(|(i, _)| *i == b)
                .ok_or(format!("Invalid black id {b}"))?
                .1;
            if white.id == black.id {
                return Err(format!("Same players w:{w}\tb:{b}"));
            }
            Ok(Game::new(white.id, black.id))
        })
        .collect::<Result<Vec<Game>, String>>()?;
    let mut byes: Vec<Bye> = bye_pairs
        .iter()
        .map(|&(p, b)| {
            if b != 0 {
                return Err(format!("Invalid bye {b}"));
            }
            let player = &players
                .iter()
                .find(|(i, _)| *i == p)
                .ok_or(format!("Invalid player id {p}"))?
                .1;
            Ok(Bye::new(player.id, ByePoint::U))
        })
        .collect::<Result<Vec<Bye>, String>>()?;

    tournament.update_current_round(&connection)?;
    let current_round = tournament
        .get_current_round(&connection)?
        .ok_or("Error updating round")?;

    for g in &mut games {
        let mut req_map = Map::new();
        req_map.insert(String::from("tournamentId"), Value::from(tournament.id));
        req_map.insert(String::from("round"), Value::from(current_round.number));
        req_map.insert(String::from("whiteId"), Value::from(g.white_id));
        req_map.insert(String::from("blackId"), Value::from(g.black_id));
        req_map.insert(String::from("whitePoint"), Value::Null);
        req_map.insert(String::from("blackPoint"), Value::Null);
        req_map.insert(String::from("ongoing"), Value::from(true));
        let req = HttpRequestBuilder::new("POST", "http://localhost:5000/games").unwrap().body(Body::Json(Value::from(req_map)));
        if let Ok(res) = client.send(req).await {
            let mut res_data = res.read().await.unwrap();
            let game_id = res_data.data["Id"].take().as_i64();
            g.id = game_id.unwrap_or_default();
        }
    }
    
    games
        .into_iter()
        .map(|g| current_round.add_game(&g, &connection))
        .collect::<Result<Vec<usize>, rusqlite::Error>>()?;
    byes.iter()
        .map(|b| current_round.add_bye(b, &connection))
        .collect::<Result<Vec<usize>, rusqlite::Error>>()?;

    for b in &mut byes {
        let mut req_map = Map::new();
        req_map.insert(String::from("tournamentId"), Value::from(tournament.id));
        req_map.insert(String::from("round"), Value::from(current_round.number));
        req_map.insert(String::from("playerId"), Value::from(b.player_id));
        req_map.insert(String::from("byePoint"), Value::from(b.bye_point.to_string()));
        let req = HttpRequestBuilder::new("POST", "http://localhost:5000/byes").unwrap().body(Body::Json(Value::from(req_map)));
        let _ = client.send(req).await;
    }
    
    let byes = current_round.get_byes(&connection)?;
    byes.into_iter()
        .map(|b| {
            let mut player = b.get_player(&connection)?;
            player.sum_point(&b.bye_point, &connection)?;
            Ok(())
        })
        .collect::<Result<(), rusqlite::Error>>()?;

    let players = tournament.get_players(&connection)?;
    players
        .into_iter()
        .map(|p| current_round.add_player_state(&p, &connection))
        .collect::<Result<Vec<usize>, rusqlite::Error>>()?;

    Ok(current_round.number)
}

#[tauri::command]
async fn get_pairings_by_round(
    round_id: i64,
    path: PathBuf,
) -> Result<(Vec<Game>, Vec<Bye>), InvokeErrorBind> {
    let connection = open_not_create(&path).await?;
    let round: Round = select_tournament(&connection)?
        .get_round_by_id(round_id, &connection)?
        .ok_or("Invalid round id")?;
    Ok((round.get_games(&connection)?, round.get_byes(&connection)?))
}

#[tauri::command]
async fn get_standings_by_round(
    round_id: i64,
    path: PathBuf,
) -> Result<Vec<Player>, InvokeErrorBind> {
    let connection = open_not_create(&path).await?;
    let round: Round = select_tournament(&connection)?
        .get_round_by_id(round_id, &connection)?
        .ok_or("Invalid round id")?;
    let mut players = round.get_standings(&connection)?;
    sort_players_ranked(&mut players);
    Ok(players)
}

#[tauri::command]
async fn get_rounds(path: PathBuf) -> Result<Vec<Round>, InvokeErrorBind> {
    let connection = open_not_create(&path).await?;
    Ok(select_tournament(&connection)?.get_rounds(&connection)?)
}

#[tauri::command]
async fn set_game_result(
    game_id: i64,
    white_point: GamePoint,
    black_point: GamePoint,
    path: PathBuf,
) -> Result<(), InvokeErrorBind> {
    let connection = open_not_create(&path).await?;
    let tournament: Tournament = select_tournament(&connection)?;
    let game: Game = tournament.get_game_by_id(game_id, &connection)?;
    if matches!(game.state, GameState::Finished(_, _)) {
        return Err("Game result already set".into());
    }
    let current_round = tournament
        .get_current_round(&connection)?
        .ok_or("No current round")?;
    if current_round.id != game.round_id {
        return Err("Invalid round".into());
    }
    game.update_result(&white_point, &black_point, &connection)?;
    let (mut white, mut black) = game.get_players(&connection)?;
    white.sum_point(&white_point, &connection)?;
    black.sum_point(&black_point, &connection)?;
    current_round.update_player_state(&white, &connection)?;
    current_round.update_player_state(&black, &connection)?;
    Ok(())
}

#[tauri::command]
async fn get_game_players(
    game_id: i64,
    path: PathBuf,
) -> Result<(Player, Player), InvokeErrorBind> {
    let connection = open_not_create(&path).await?;
    let tournament = select_tournament(&connection)?;
    let game = tournament.get_game_by_id(game_id, &connection)?;
    Ok(game.get_players(&connection)?)
}

#[tauri::command]
async fn make_trf_file(path: PathBuf, app: AppHandle) -> Result<String, InvokeErrorBind> {
    let connection = open_not_create(&path).await?;
    let tournament = select_tournament(&connection)?;
    let rounds = tournament.get_rounds(&connection)?;
    let mut players = tournament.get_players(&connection)?;
    sort_players_ranked(&mut players);
    let mut players: Vec<(usize, Player)> = players
        .into_iter()
        .enumerate()
        .map(|(i, p)| (i + 1, p))
        .collect();
    helper(&mut players);
    let players: Vec<(usize, usize, Player)> = players
        .into_iter()
        .enumerate()
        .map(|(i, (f, p))| (i + 1, f, p))
        .collect();

    let tournament_data = tournament.get_trf_config(&connection)?;
    let players_lines = get_players_lines(&players, &rounds, &connection)?;

    let desktop_path = get_desktop_path(&app)?;
    let mut trf_path = desktop_path.join(tournament.name);
    trf_path.set_extension("trf");

    let mut buff_trf = BufWriter::new(
        OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(&trf_path)?,
    );
    buff_trf.write(format! {"{}", tournament_data}.as_bytes())?;
    buff_trf.write(format! {"{}\r\n", players_lines.join("\r\n")}.as_bytes())?;
    buff_trf.flush()?;

    Ok(trf_path.into_os_string().into_string()?)
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

fn get_bbp_output_file_path(app: &AppHandle) -> tauri::api::Result<PathBuf> {
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

fn get_desktop_path(app: &AppHandle) -> tauri::api::Result<PathBuf> {
    Ok(resolve_path(
        &app.config(),
        &app.package_info(),
        &Env::default(),
        "",
        Some(BaseDirectory::Desktop),
    )?)
}

fn main() {
    tauri::Builder::default()
        .manage::<Client>(ClientBuilder::new().build().unwrap())
        .setup(|app| {
            let bbp_output_file_path = get_bbp_output_file_path(&app.handle())?;
            if !bbp_output_file_path.exists() {
                File::create(&bbp_output_file_path)?;
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
            add_player,
            get_current_round,
            make_pairing,
            get_standings_by_round,
            get_pairings_by_round,
            get_rounds,
            set_game_result,
            get_game_players,
            make_trf_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
