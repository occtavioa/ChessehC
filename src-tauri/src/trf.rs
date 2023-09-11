use crate::models::{Pairing, Player};
use std::{
    fs::File,
    io::{BufWriter, Result, Write},
};

pub fn write_players_partial(
    buff: &mut BufWriter<File>,
    players: &[Player],
    pairings: &[Pairing],
) -> Result<()> {
    let players_lines: Vec<String> =
        players
            .iter()
            .enumerate()
            .map(|(i, p)| {
                format!(
                    "001 {:>4} {} {:>3} {:>33} {:>4} {:>3} {:>11} {:>10} {:>4} {:>4} {:>4}\r",
                    i,
                    "",
                    "",
                    &p.name,
                    p.rating,
                    "",
                    "",
                    "",
                    p.points,
                    "", // ranking, not necessary
                    pairings
                        .iter()
                        .filter(|p| match p {
                            Pairing::Bye {
                                starting_rank_player,
                                ..
                            } => *starting_rank_player == i as u16,
                            Pairing::Match {
                                black_starting_rank,
                                white_starting_rank,
                                ..
                            } =>
                                (*black_starting_rank == i as u16)
                                    || (*white_starting_rank == i as u16),
                        })
                        .map(|p| match p {
                            Pairing::Bye { bye_point, .. } => format!("0000 - {}", bye_point),
                            Pairing::Match {
                                white_starting_rank,
                                black_starting_rank,
                                white_result,
                                black_result,
                                ..
                            } =>
                                if *white_starting_rank == i as u16 {
                                    format!("{:>4} w {}", white_starting_rank, white_result)
                                } else {
                                    format!("{:>4} b {}", black_starting_rank, black_result)
                                },
                        })
                        .collect::<Vec<String>>()
                        .join("    ")
                )
            })
            .collect();
    buff.write(players_lines.join("").as_bytes())?;
    Ok(())
}
