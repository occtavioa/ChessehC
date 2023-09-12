use crate::models::{MatchPlayerResult, PairingKind, Player, Pairing};
use std::{
    fs::File,
    io::{BufWriter, Result, Write},
};

pub fn write_players_partial(
    buff: &mut BufWriter<File>,
    players: &[Player],
    pairings: &[Pairing],
) -> Result<()> {
    let players_lines: Vec<String> = players
        .iter()
        .enumerate()
        .map(|(i, player)| {
            format!(
                "001 {:>4} {} {:>3} {:>33} {:>4} {:>3} {:>11} {:>10} {:>4} {:>4}  {}\r",
                i,
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
                        PairingKind::Match {
                            black_id, white_id, ..
                        } => (*black_id == player.id) || (*white_id == player.id),
                    })
                    .map(|Pairing { kind, .. }| match kind {
                        PairingKind::Bye { bye_point, .. } =>
                            format!("0000 - {}", bye_point.to_string()),
                        PairingKind::Match {
                            white_id,
                            black_id,
                            white_result,
                            black_result,
                            ..
                        } =>
                            if *white_id == player.id {
                                format!(
                                    "{:>4} w {}",
                                    i + 1,
                                    match white_result {
                                        MatchPlayerResult::W => "1",
                                        MatchPlayerResult::D => "=",
                                        MatchPlayerResult::L => "0",
                                    }
                                )
                            } else {
                                format!(
                                    "{:>4} b {}",
                                    i + 1,
                                    match black_result {
                                        MatchPlayerResult::W => "1",
                                        MatchPlayerResult::D => "=",
                                        MatchPlayerResult::L => "0",
                                    }
                                )
                            },
                    })
                    .collect::<Vec<String>>()
                    .join("  ")
            )
        })
        .collect();
    buff.write(players_lines.join("").as_bytes())?;
    Ok(())
}
