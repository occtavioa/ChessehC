use rusqlite::Connection;

use crate::models::{GameState, Player, Round};

pub enum Error {
    Ctf(rusqlite::Error),
    PairingNotFound,
    InvalidPairing,
}

impl From<rusqlite::Error> for Error {
    fn from(value: rusqlite::Error) -> Self {
        Error::Ctf(value)
    }
}

pub fn get_players_lines(
    players: &[(u16, Player)],
    rounds: &[Round],
    connection: &Connection,
) -> Result<Vec<String>, Error> {
    let lines = players
        .iter()
        .map(|(starting_rank, player)| {
            let player_data = format!(
                "001 {:>4} {:>1}{:>3} {:>33} {:>4} {:>3} {:>11} {:>10} {:>4.1} {:>4}",
                starting_rank, "", "", player.name, player.rating, "", "", "", player.points, 0
            );
            let rounds_data = rounds
                .iter()
                .map(|r| {
                    if let Some(g) = player.get_game_by_round(r.id, connection)? {
                        let opponent_starting_rank = &players
                            .iter()
                            .find(|(_, p)| {
                                p.id == (if player.id == g.white_id {
                                    g.black_id
                                } else {
                                    g.white_id
                                })
                            })
                            .ok_or(Error::InvalidPairing)?
                            .0;
                        if let GameState::Finished(wp, bp) = &g.state {
                            Ok(format!(
                                "{:>4} {:>1} {:>1}",
                                opponent_starting_rank,
                                if g.white_id == player.id { "w" } else { "b" },
                                if g.white_id == player.id {
                                    wp.to_string()
                                } else {
                                    bp.to_string()
                                }
                            ))
                        } else {
                            Err(Error::InvalidPairing)
                        }
                    } else if let Some(b) = player.get_bye_by_round(r.id, connection)? {
                        Ok(format!("0000   {:>1}", b.bye_point.to_string()))
                    } else {
                        Err(Error::PairingNotFound)
                    }
                })
                .collect::<Result<Vec<String>, Error>>()?;
            Ok(format!("{}  {}", player_data, rounds_data.join("  ")))
        })
        .collect::<Result<Vec<String>, Error>>()?;
    Ok(lines)
}
