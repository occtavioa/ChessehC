use crate::models::{GamePlayerResult, Pairing, PairingKind, Player};
use std::{
    fs::File,
    io::{BufWriter, Result, Write},
};

pub fn write_configuration(buff: &mut BufWriter<File>, number_rounds: u16) -> Result<usize> {
    buff.write(format!("XXC white1\rXXR {}\r", number_rounds).as_bytes())
}

pub fn write_players_partial(
    buff: &mut BufWriter<File>,
    players: &[Player],
    pairings: &[Pairing],
) -> Result<usize> {
    let players_lines: Vec<String> = players
        .iter()
        .enumerate()
        .map(|(i, player)| {
            format!(
                "001 {:>4} {} {:>3} {:>33} {:>4} {:>3} {:>11} {:>10} {:>4.1} {:>4}  {}\r",
                i + 1,
                "",
                "",
                &player.name,
                player.rating,
                "",
                "",
                "",
                player.points,
                "",
                pairings
                    .iter()
                    .filter(|Pairing { kind, .. }| match kind {
                        PairingKind::Bye { player_id, .. } => *player_id == player.id,
                        PairingKind::Game {
                            black_id, white_id, ..
                        } => (*black_id == player.id) || (*white_id == player.id),
                    })
                    .map(|Pairing { kind, .. }| match kind {
                        PairingKind::Bye { bye_point, .. } =>
                            format!("0000 - {}", bye_point.to_string()),
                        PairingKind::Game {
                            white_id,
                            black_id,
                            white_result,
                            black_result,
                        } =>
                            if *white_id == player.id {
                                format!(
                                    "{:>4} w {}",
                                    players.iter().enumerate().find(|(.., p)| p.id == *black_id).unwrap().0 + 1,
                                    match_player_result_symbol(white_result.as_ref().unwrap())
                                )
                            } else {
                                format!(
                                    "{:>4} b {}",
                                    players.iter().enumerate().find(|(.., p)| p.id == *white_id).unwrap().0 + 1,
                                    match_player_result_symbol(black_result.as_ref().unwrap())
                                )
                            },
                    })
                    .collect::<Vec<String>>()
                    .join("  ")
            )
        })
        .collect();
    buff.write(players_lines.join("").as_bytes())
}

fn match_player_result_symbol(mpr: &GamePlayerResult) -> char {
    match mpr {
        GamePlayerResult::W => '1',
        GamePlayerResult::D => '=',
        GamePlayerResult::L => '0',
    }
}
