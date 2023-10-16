use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};

use super::{point::ByePoint, player::Player};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Bye {
    pub id: i64,
    pub round_id: i64,
    pub player_id: i64,
    pub bye_point: ByePoint,
}

impl Bye {
    pub fn new(player_id: i64, point: ByePoint) -> Self {
        Bye {
            player_id,
            bye_point: point,
            ..Default::default()
        }
    }
    pub fn get_player(&self, connection: &Connection) -> Result<Player, rusqlite::Error> {
        connection.query_row(
            "
                SELECT Player.*
                FROM Player
                INNER JOIN ByeByRound ON ByeByRound.PlayerId = Player.Id
                WHERE ByeByRound.Id = (?1)
            ",
            params![self.id],
            |row| {
                Ok(Player {
                    id: row.get(0)?,
                    tournament_id: row.get(1)?,
                    name: row.get(2)?,
                    points: row.get(3)?,
                    rating: row.get(4)?,
                })
            },
        )
    }
}
