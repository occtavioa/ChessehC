use crate::models::tournament::Tournament;
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
            \"CurrentRoundId\"  INTEGER,
            PRIMARY KEY(\"Id\")
        );
        CREATE TABLE \"Player\" (
            \"Id\"	INTEGER,
            \"TournamentId\"    INTEGER NOT NULL,
            \"Name\"	TEXT NOT NULL,
            \"Points\"  REAL NOT NULL,
            \"Rating\"	INTEGER,
            \"Title\"   TEXT,
            PRIMARY KEY(\"Id\")
        );
        CREATE TABLE \"Round\" (
            \"Id\"	INTEGER,
            \"TournamentId\"    INTEGER NOT NULL,
            \"Number\"  INTEGER NOT NULL,
            \"Date\"    TEXT,
            PRIMARY KEY(\"Id\")
        );
        CREATE TABLE \"PlayerStateByRound\" (
            \"PlayerId\"    INTEGER NOT NULL,
            \"RoundId\" INTEGER NOT NULL,
            \"Points\"  REAL
        );
        CREATE TABLE \"GameByRound\" (
            \"Id\"  INTEGER,
            \"RoundId\" INTEGER NOT NULL,
            \"WhiteId\" INTEGER NOT NULL,
            \"BlackId\" INTEGER NOT NULL,
            \"Ongoing\" INTEGER NOT NULL,
            \"WhiteResult\" TEXT,
            \"BlackResult\" TEXT,
            PRIMARY KEY (\"Id\")
        );
        CREATE TABLE \"ByeByRound\" (
            \"Id\"  INTEGER,
            \"RoundId\" INTEGER NOT NULL,
            \"PlayerId\"    INTEGER NOT NULL,
            \"ByePoint\"    TEXT NOT NULL,
            PRIMARY KEY (\"Id\")
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
        params![&tournament.name, &tournament.number_rounds,],
    )
}

pub fn select_tournament(connection: &Connection) -> Result<Tournament> {
    connection.query_row("SELECT * FROM \"Tournament\"", [], |row| {
        Ok(Tournament {
            id: row.get(0)?,
            name: row.get(1)?,
            number_rounds: row.get(2)?,
            current_round_id: row.get(3)?,
        })
    })
}
