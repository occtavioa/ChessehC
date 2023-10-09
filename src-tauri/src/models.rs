use rusqlite::{
    params,
    types::{FromSql, FromSqlError, FromSqlResult},
    Connection, OptionalExtension, ToSql,
};
use serde::{Deserialize, Serialize};
use std::str::{from_utf8, FromStr};

use crate::db::get_player_by_id;

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
                    (?3)
                )
            ",
            params![self.id, player.name, player.rating],
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Player {
    pub id: i64,
    pub tournament_id: i64,
    pub name: String,
    pub points: f64,
    pub rating: u16,
}

impl Player {
    pub fn get_games(&self, connection: &Connection) -> Result<Vec<Game>, rusqlite::Error> {
        let mut statement = connection.prepare(
            "
                SELECT *
                FROM GameByRound
                INNER JOIN Player ON Player.Id = GameByRound.WhiteId = Player.Id OR GameByRound.BlackId = Player.Id
                WHERE Player.Id = (?1)
            "
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
    pub fn get_byes(&self, connection: &Connection) -> Result<Vec<Bye>, rusqlite::Error> {
        let mut statement = connection.prepare(
            "
            SELECT *
            FROM ByeByRound
            INNER JOIN Player ON ByeByRound.PlayerId = Player.Id
            WHERE Player.Id = (?1)
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
    pub fn sum_point<P: Point>(
        &mut self,
        point: &P,
        connection: &Connection,
    ) -> Result<usize, rusqlite::Error> {
        self.points += point.get_value();
        connection.execute(
            "
                UPDATE Player
                SET Points = (Points + (?1))
                WHERE Id = (?2)
            ",
            params![point.get_value(), self.id],
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

#[derive(Debug, Default, Deserialize, Serialize)]
pub enum ByePoint {
    U,
    #[default]
    Z,
}

pub trait Point {
    fn get_value(&self) -> f64;
}

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
                SELECT Player.Id, Player.TournamentId, Player.Name, ps.Points, Player.Rating
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
    pub fn update_player_state(&self, player: &Player, connection: &Connection) -> Result<usize, rusqlite::Error> {
        connection.execute(
            "
                UPDATE PlayerStateByRound
                SET Points = (?1)
                WHERE PlayerId = (?2) AND RoundId = (?3)
            ",
            params![player.points, player.id, self.id]
        )
    }
}

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
        Ok((get_player_by_id(self.white_id, connection)?, get_player_by_id(self.black_id, connection)?))
    }
}

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

#[derive(Debug, Deserialize, Serialize)]
pub enum GamePoint {
    W,
    D,
    L,
}

impl Point for GamePoint {
    fn get_value(&self) -> f64 {
        match self {
            Self::W => 1.0,
            Self::D => 1.0 / 2.0,
            Self::L => 0.0,
        }
    }
}

impl ToSql for GamePoint {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}

impl FromSql for GamePoint {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Text(t) => {
                match GamePoint::from_str(from_utf8(t).unwrap_or_default()) {
                    Ok(bp) => Ok(bp),
                    Err(_) => Err(FromSqlError::InvalidType),
                }
            }
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToString for GamePoint {
    fn to_string(&self) -> String {
        match self {
            Self::W => String::from("W"),
            Self::D => String::from("D"),
            Self::L => String::from("L"),
        }
    }
}

impl FromStr for GamePoint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "W" => Ok(Self::W),
            "D" => Ok(Self::D),
            "L" => Ok(Self::L),
            _ => Err(String::from("Invalid")),
        }
    }
}

impl Point for ByePoint {
    fn get_value(&self) -> f64 {
        match self {
            Self::U => 1.0,
            Self::Z => 0.0,
        }
    }
}

impl ToSql for ByePoint {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}

impl FromSql for ByePoint {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Text(t) => {
                match ByePoint::from_str(from_utf8(t).unwrap_or_default()) {
                    Ok(bp) => Ok(bp),
                    Err(_) => Err(FromSqlError::InvalidType),
                }
            }
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToString for ByePoint {
    fn to_string(&self) -> String {
        match self {
            Self::U => String::from("U"),
            Self::Z => String::from("Z"),
        }
    }
}

impl FromStr for ByePoint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::U),
            "Z" => Ok(Self::Z),
            _ => Err(String::from("Invalid")),
        }
    }
}
