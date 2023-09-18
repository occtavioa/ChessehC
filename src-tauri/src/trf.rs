use crate::models::{GamePlayerResult, Pairing, PairingKind, Player, ByeInfo, GameInfo};
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
        .map(|(i, p)| {
            format!(
                "001 {:>4} {} {:>3} {:>33} {:>4} {:>3} {:>11} {:>10} {:>4.1} {:>4}  {}\r",
                i + 1,
                "",
                "",
                &p.name,
                p.rating,
                "",
                "",
                "",
                p.points,
                "",
                pairings
                    .iter()
                    .filter(|Pairing { kind, .. }| match kind {
                        PairingKind::Bye(ByeInfo{player, ..}) => player.id == p.id,
                        PairingKind::Game(GameInfo {
                            black_player, white_player, ..
                        }) => (black_player.id == p.id) || (white_player.id == p.id),
                    })
                    .map(|Pairing { kind, .. }| match kind {
                        PairingKind::Bye(ByeInfo { bye_point, .. }) =>
                            format!("0000 - {}", bye_point.to_string()),
                        PairingKind::Game(GameInfo {
                            white_player,
                            black_player,
                            white_result,
                            black_result,
                            ..
                        }) =>
                            if white_player.id == p.id {
                                format!(
                                    "{:>4} w {}",
                                    players
                                        .iter()
                                        .enumerate()
                                        .find(|(.., player)| player.id == black_player.id)
                                        .unwrap()
                                        .0
                                        + 1,
                                    match_player_result_symbol(white_result.as_ref().unwrap())
                                )
                            } else {
                                format!(
                                    "{:>4} b {}",
                                    players
                                        .iter()
                                        .enumerate()
                                        .find(|(.., player)| player.id == white_player.id)
                                        .unwrap()
                                        .0
                                        + 1,
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
