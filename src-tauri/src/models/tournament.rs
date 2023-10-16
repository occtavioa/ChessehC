use rusqlite::{Connection, params, OptionalExtension};
use serde::{Deserialize, Serialize};

use super::{player::Player, round::Round, game::{Game, GameState}};

#[derive(Debug, Deserialize, Serialize)]
pub struct Tournament {
    pub id: i64,
    pub name: String,
    pub number_rounds: u16,
    pub current_round_id: Option<i64>,
}

impl Tournament {
    pub fn get_players(&self, connection: &Connection) -> Result<Vec<Player>, rusqlite::Error> {
        let mut statement = connection.prepare(
            "
                SELECT *
                FROM Player
                WHERE TournamentId = (?1)
            ",
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
    pub fn add_player(
        &self,
        player: &Player,
        connection: &Connection,
    ) -> Result<usize, rusqlite::Error> {
        connection.execute(
            "
                INSERT INTO Player
                VALUES (
                    NULL,
                    (?1),
                    (?2),
                    0,
                    (?3),
                    (?4)
                )
            ",
            params![self.id, player.name, player.rating, player.title],
        )
    }
    pub fn get_last_added_player(
        &self,
        connection: &Connection,
    ) -> Result<Player, rusqlite::Error> {
        connection.query_row(
            "
                SELECT *
                FROM Player
                WHERE ROWID = (SELECT max(ROWID) FROM Player)
            ",
            params![],
            |row| {
                Ok(Player {
                    id: row.get(0)?,
                    tournament_id: row.get(1)?,
                    name: row.get(2)?,
                    points: row.get(3)?,
                    rating: row.get(4)?,
                    title: row.get(5)?
                })
            },
        )
    }
    pub fn get_rounds(&self, connection: &Connection) -> Result<Vec<Round>, rusqlite::Error> {
        let mut statement = connection.prepare(
            "
                SELECT *
                FROM Round
                WHERE TournamentId = (?1)
            ",
        )?;
        let rounds_iter = statement.query_map(params![self.id], |row| {
            Ok(Round {
                id: row.get(0)?,
                tournament_id: row.get(1)?,
                number: row.get(2)?,
                date: row.get(3)?,
            })
        })?;
        rounds_iter.collect()
    }
    pub fn get_current_round(
        &self,
        connection: &Connection,
    ) -> Result<Option<Round>, rusqlite::Error> {
        connection
            .query_row(
                "
                SELECT Round.*
                FROM Round
                INNER JOIN Tournament ON Round.TournamentId = Tournament.Id
                WHERE Round.Id = Tournament.CurrentRoundId
            ",
                params![],
                |row| {
                    Ok(Round {
                        id: row.get(0)?,
                        tournament_id: row.get(1)?,
                        number: row.get(2)?,
                        date: row.get(3)?,
                    })
                },
            )
            .optional()
    }
    pub fn get_trf_config(&self) -> String {
        format!("XXC white1\r\nXXR {}", self.number_rounds)
    }
    pub fn update_current_round(&self, connection: &Connection) -> Result<usize, rusqlite::Error> {
        let mut current_round = self.get_current_round(connection)?.unwrap_or_default();
        current_round.number += 1;
        self.add_round(&current_round, connection)?;
        connection.execute(
            "
                UPDATE Tournament
                SET CurrentRoundId = (
                    SELECT Id
                    FROM Round
                    WHERE Number = (
                        SELECT max(Number)
                        FROM Round
                    )
                )
            ",
            params![],
        )
    }
    pub fn add_round(
        &self,
        round: &Round,
        connection: &Connection,
    ) -> Result<usize, rusqlite::Error> {
        connection.execute(
            "
                INSERT INTO Round
                VALUES (
                    NULL,
                    (?1),
                    (?2),
                    (?3)
                )
            ",
            params![self.id, round.number, round.date],
        )
    }
    pub fn get_round_by_id(
        &self,
        id: i64,
        connection: &Connection,
    ) -> Result<Option<Round>, rusqlite::Error> {
        connection
            .query_row(
                "
                SELECT Round.*
                FROM Round
                INNER JOIN Tournament ON Round.TournamentId = Tournament.Id
                WHERE Round.Id = (?1)
            ",
                params![id],
                |row| {
                    Ok(Round {
                        id: row.get(0)?,
                        tournament_id: row.get(1)?,
                        number: row.get(2)?,
                        date: row.get(3)?,
                    })
                },
            )
            .optional()
    }
    pub fn get_game_by_id(&self, game_id: i64, connection: &Connection) -> Result<Game, rusqlite::Error> {
        connection.query_row(
            "
                SELECT *
                FROM GameByRound
                WHERE GameByRound.Id = (?1)
            ",
            params![game_id],
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
        )
    }
}
