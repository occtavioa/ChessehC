use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use super::{
    bye::Bye,
    game::{Game, GameState},
    player::Player,
};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Round {
    pub id: i64,
    pub tournament_id: i64,
    pub number: u16,
    pub date: String,
}

impl Round {
    pub fn add_bye(&self, bye: &Bye, connection: &Connection) -> Result<usize, rusqlite::Error> {
        connection.execute(
            "
                INSERT INTO ByeByRound
                VALUES (
                    NULL,
                    (?1),
                    (?2),
                    (?3)
                )
            ",
            params![self.id, bye.player_id, bye.bye_point],
        )
    }
    pub fn get_games(&self, connection: &Connection) -> Result<Vec<Game>, rusqlite::Error> {
        let mut statement = connection.prepare(
            "
                SELECT *
                FROM GameByRound
                WHERE RoundId = (?1)
            ",
        )?;
        let games_iter = statement.query_map(params![self.id], |row| {
            Ok(Game {
                id: row.get(0)?,
                round_id: row.get(1)?,
                white_id: row.get(2)?,
                black_id: row.get(3)?,
                state: match row.get(4)? {
                    true => GameState::Ongoing,
                    false => GameState::Finished(row.get(5)?, row.get(6)?),
                },
            })
        })?;
        games_iter.collect()
    }
    pub fn add_game(&self, game: &Game, connection: &Connection) -> Result<usize, rusqlite::Error> {
        connection.execute(
            "
                INSERT INTO GameByRound
                VALUES (
                    (?1),
                    (?2),
                    (?3),
                    (?4),
                    TRUE,
                    NULL,
                    NULL
                )
            ",
            params![game.id, self.id, game.white_id, game.black_id],
        )
    }
    pub fn add_game_autoinc_id(&self, game: &Game, connection: &Connection) -> Result<usize, rusqlite::Error> {
        connection.execute(
            "
                INSERT INTO GameByRound
                VALUES (
                    NULL,
                    (?1),
                    (?2),
                    (?3),
                    TRUE,
                    NULL,
                    NULL
                )
            ",
            params![self.id, game.white_id, game.black_id],
        )
    }
    pub fn get_standings(&self, connection: &Connection) -> Result<Vec<Player>, rusqlite::Error> {
        let mut statement = connection.prepare(
            "
                SELECT Player.Id, Player.TournamentId, Player.Name, ps.Points, Player.Rating, Player.Title
                FROM Player
                INNER JOIN PlayerStateByRound AS ps ON ps.PlayerId = Player.Id
                INNER JOIN Round ON Round.Id = ps.RoundId
                WHERE Round.Id = (?1)
            "
        )?;
        let players_iter = statement.query_map(params![self.id], |row| {
            Ok(Player {
                id: row.get(0)?,
                tournament_id: row.get(1)?,
                name: row.get(2)?,
                points: row.get(3)?,
                rating: row.get(4)?,
                title: row.get(5)?,
            })
        })?;
        players_iter.collect()
    }
    pub fn add_player_state(
        &self,
        player: &Player,
        connection: &Connection,
    ) -> Result<usize, rusqlite::Error> {
        connection.execute(
            "
                INSERT INTO PlayerStateByRound
                VALUES (
                    (?1),
                    (?2),
                    (?3)
                )
            ",
            params![player.id, self.id, player.points],
        )
    }
    pub fn get_byes(&self, connection: &Connection) -> Result<Vec<Bye>, rusqlite::Error> {
        let mut statement = connection.prepare(
            "
                SELECT ByeByRound.*
                FROM ByeByRound
                INNER JOIN Round ON ByeByRound.RoundId = Round.Id
                WHERE Round.Id = (?1)
            ",
        )?;
        let byes_iter = statement.query_map(params![self.id], |row| {
            Ok(Bye {
                id: row.get(0)?,
                round_id: row.get(1)?,
                player_id: row.get(2)?,
                bye_point: row.get(3)?,
            })
        })?;
        byes_iter.collect()
    }
    pub fn update_player_state(
        &self,
        player: &Player,
        connection: &Connection,
    ) -> Result<usize, rusqlite::Error> {
        connection.execute(
            "
                UPDATE PlayerStateByRound
                SET Points = (?1)
                WHERE PlayerId = (?2) AND RoundId = (?3)
            ",
            params![player.points, player.id, self.id],
        )
    }
}
