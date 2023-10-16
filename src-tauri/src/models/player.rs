use rusqlite::{Connection, params, OptionalExtension};
use serde::{Deserialize, Serialize};

use super::{game::{Game, GameState}, bye::Bye, point::Point};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Player {
    pub id: i64,
    pub tournament_id: i64,
    pub name: String,
    pub points: f64,
    pub rating: u16,
}

impl Player {
    pub fn sum_point<P: Point>(
        &mut self,
        point: &P,
        connection: &Connection,
    ) -> Result<usize, rusqlite::Error> {
        self.points += point.get_value();
        connection.execute(
            "
                UPDATE Player
                SET Points = (?1)
                WHERE Id = (?2)
            ",
            params![self.points, self.id],
        )
    }
    pub fn get_game_by_round(
        &self,
        round_id: i64,
        connection: &Connection,
    ) -> Result<Option<Game>, rusqlite::Error> {
        connection.query_row(
            "
                SELECT GameByRound.*
                FROM GameByRound
                INNER JOIN Player ON GameByRound.WhiteId = Player.Id OR GameByRound.BlackId = Player.Id
                WHERE GameByRound.RoundId = (?1) AND Player.Id = (?2)
            ",
            params![round_id, self.id],
            |row| Ok(Game {
                id: row.get(0)?,
                round_id: row.get(1)?,
                white_id: row.get(2)?,
                black_id: row.get(3)?,
                state: match row.get(4)? {
                    true => GameState::Ongoing,
                    false => GameState::Finished(row.get(5)?, row.get(6)?),
                },
            })
        ).optional()
    }
    pub fn get_bye_by_round(
        &self,
        round_id: i64,
        connection: &Connection,
    ) -> Result<Option<Bye>, rusqlite::Error> {
        connection
            .query_row(
                "
                SELECT ByeByRound.*
                FROM ByeByRound
                INNER JOIN Player ON ByeByRound.PlayerId = Player.Id
                WHERE ByeByRound.RoundId = (?1) AND Player.Id = (?2)
            ",
                params![round_id, self.id],
                |row| {
                    Ok(Bye {
                        id: row.get(0)?,
                        round_id: row.get(1)?,
                        player_id: row.get(2)?,
                        bye_point: row.get(3)?,
                    })
                },
            )
            .optional()
    }
}
