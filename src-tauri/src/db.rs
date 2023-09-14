use crate::models::{Pairing, PairingKind, Player, Tournament};
use rusqlite::{params, Connection, OpenFlags, Result};
use std::path::Path;

pub async fn open_not_create(path: &Path) -> Result<Connection> {
    Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_NO_MUTEX
            | OpenFlags::SQLITE_OPEN_URI,
    )
}

pub fn create_schema(connection: &Connection) -> Result<()> {
    connection.execute_batch(
        "
        CREATE TABLE \"Tournament\" (
            \"Id\"      INTEGER,
            \"Name\"	TEXT NOT NULL,
            \"NumberRounds\"	INTEGER NOT NULL,
            \"CurrentRound\"	INTEGER,
            PRIMARY KEY(\"Id\")
        );
        CREATE TABLE \"Player\" (
            \"Id\"	INTEGER,
            \"Name\"	TEXT NOT NULL,
            \"Points\"  REAL NOT NULL,
            \"Rating\"	INTEGER,
            PRIMARY KEY(\"Id\")
        );
        CREATE TABLE \"Round\" (
            \"Id\"	INTEGER,
            \"Number\"  INTEGER NOT NULL,
            \"Date\"    TEXT,
            PRIMARY KEY(\"Id\")
        );
        CREATE TABLE \"PlayerStateByRound\" (
            \"PlayerId\"    INTEGER,
            \"RoundId\" INTEGER,
            \"Points\"  REAL
        );
        CREATE TABLE \"MatchByRound\" (
            \"RoundId\" INTEGER NOT NULL,
            \"WhiteId\" INTEGER NOT NULL,
            \"BlackId\" INTEGER NOT NULL,
            \"WhiteResult\" TEXT,
            \"BlackResult\" TEXT
        );
        CREATE TABLE \"ByeByRound\" (
            \"RoundId\" INTEGER NOT NULL,
            \"PlayerId\"    INTEGER NOT NULL,
            \"ByePoint\"    TEXT NOT NULL
        );
        ",
    )
}

pub fn insert_tournament(tournament: &Tournament, connection: &Connection) -> Result<usize> {
    connection.execute(
        "
        INSERT INTO \"Tournament\" VALUES
        (
            NULL,
            (?1),
            (?2),
            NULL
        )
        ",
        params![
            &tournament.name,
            &tournament.number_rounds,
        ],
    )
}

pub fn select_tournament(connection: &Connection) -> Result<Tournament> {
    connection.query_row("SELECT * FROM \"Tournament\"", [], |row| {
        Ok(Tournament {
            name: row.get(1)?,
            number_rounds: row.get(2)?,
            current_round: row.get(3)?,
        })
    })
}

pub fn select_players(connection: &Connection) -> Result<Vec<Player>> {
    let mut query = connection.prepare("SELECT * FROM \"Player\"")?;
    let players_iter = query
        .query_map([], |row| {
            Ok(Player {
                id: row.get(0)?,
                name: row.get(1)?,
                points: row.get(2)?,
                rating: row.get(3)?,
            })
        })?
        .filter(|p| p.is_ok());
    Ok(players_iter
        .map(|player| player.unwrap())
        .into_iter()
        .collect())
}

pub fn insert_player(connection: &Connection, player: &Player) -> Result<usize> {
    connection.execute(
        "
        INSERT INTO \"Player\" VALUES
        (
            NULL,
            (?1),
            0,
            (?2)
        )
        ",
        params![
            &player.name,
            player.rating,
        ],
    )
}

pub fn select_current_round(connection: &Connection) -> Result<Option<u16>> {
    connection.query_row("SELECT CurrentRound FROM \"Tournament\"", [], |row| {
        Ok(row.get(0)?)
    })
}

pub fn select_pairings(connection: &Connection) -> Result<Vec<Pairing>> {
    let mut matches = select_matches(&connection)?;
    let byes = select_byes(&connection)?;
    matches.extend(byes);
    Ok(matches)
}

pub fn select_matches(connection: &Connection) -> Result<Vec<Pairing>> {
    let mut statement = connection.prepare(
        "SELECT Round.Number, MatchByRound.* FROM MatchByRound INNER JOIN Round ON Round.Id = MatchByRound.RoundId"
    )?;
    let matches_iter = statement.query_map(params![], |row| {
        Ok(Pairing {
            number_round: row.get(0)?,
            kind: PairingKind::Match {
                white_id: row.get(2)?,
                black_id: row.get(3)?,
                white_result: row.get(4)?,
                black_result: row.get(5)?,
            },
        })
    })?;
    Ok(matches_iter.map(|m| m.unwrap()).collect())
}

pub fn select_byes(connection: &Connection) -> Result<Vec<Pairing>> {
    let mut statement = connection.prepare(
        "SELECT Round.Number, ByeByRound.* FROM ByeByRound INNER JOIN Round ON Round.Id = ByeByRound.RoundId"
    )?;
    let byes_iter = statement.query_map(params![], |row| {
        Ok(Pairing {
            number_round: row.get(0)?,
            kind: PairingKind::Bye {
                player_id: row.get(2)?,
                bye_point: row.get(3)?,
            },
        })
    })?;
    Ok(byes_iter.map(|b| b.unwrap()).collect())
}

pub fn select_number_rounds(connection: &Connection) -> Result<u16> {
    connection.query_row("SELECT NumberRounds FROM Tournament", params![], |row| row.get(0))
}
