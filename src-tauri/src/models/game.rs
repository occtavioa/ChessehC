use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};

use super::{point::GamePoint, player::Player};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Game {
    pub id: i64,
    pub round_id: i64,
    pub white_id: i64,
    pub black_id: i64,
    pub state: GameState,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub enum GameState {
    #[default]
    Ongoing,
    Finished(GamePoint, GamePoint),
}

impl Game {
    pub fn new(white_id: i64, black_id: i64) -> Self {
        Game {
            white_id,
            black_id,
            ..Default::default()
        }
    }
    pub fn update_result(
        &self,
        white_point: &GamePoint,
        black_point: &GamePoint,
        connection: &Connection,
    ) -> Result<usize, rusqlite::Error> {
        connection.execute(
            "
                UPDATE GameByRound
                SET WhiteResult = (?1), BlackResult = (?2), Ongoing = FALSE
                WHERE Id = (?3)
            ",
            params![white_point, black_point, self.id]
        )
    }
    pub fn get_players(&self, connection: &Connection) -> Result<(Player, Player), rusqlite::Error> {
        connection.query_row(
            "
                SELECT w.*, b.*
                FROM GameByRound as gbr
                INNER JOIN Player AS w ON gbr.WhiteId = w.Id
                INNER JOIN Player AS b ON gbr.BlackId = b.Id
                WHERE gbr.Id = (?1)
            ",
            params![self.id],
            |row| Ok((Player {
                id: row.get(0)?,
                tournament_id: row.get(1)?,
                name: row.get(2)?,
                points: row.get(3)?,
                rating: row.get(4)?,
                title: row.get(5)?
            }, Player {
                id: row.get(5)?,
                tournament_id: row.get(6)?,
                name: row.get(7)?,
                points: row.get(8)?,
                rating: row.get(9)?,
                title: row.get(10)?
            }))
        )
    }
}
