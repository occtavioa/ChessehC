use std::path::Path;
use crate::models::{Player, Tournament};
use rusqlite::{params, Connection, OpenFlags};

pub fn open_not_create(path: &Path) -> rusqlite::Result<Connection> {
    Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_WRITE
        | OpenFlags::SQLITE_OPEN_NO_MUTEX
        | OpenFlags::SQLITE_OPEN_URI)
}

pub fn create_schema(connection: &Connection) -> rusqlite::Result<()> {
    connection.execute_batch(
        "CREATE TABLE \"Tournament\" (
            \"Id\"      INTEGER,
            \"Name\"	TEXT NOT NULL,
            \"City\"	TEXT,
            \"FideFederation\"	    TEXT,
            \"DateStart\"	TEXT,
            \"DateEnd\"	TEXT,
            \"TypeTournament\"	TEXT,
            \"Format\"	    TEXT,
            \"ChiefArbiter\"	TEXT,
            \"DeputyChiefArbiter\"	TEXT,
            \"TimeControl\"	TEXT,
            \"NumberRounds\"	INTEGER,
            \"CurrentRound\"	INTEGER,
            PRIMARY KEY(\"Id\")
        );
        CREATE TABLE \"Players\" (
            \"Id\"	INTEGER,
            \"Name\"	TEXT NOT NULL,
            \"Sex\"	TEXT,
            \"Title\"	TEXT,
            \"Rating\"	INTEGER,
            \"FideFederation\"	TEXT,
            \"FideNumber\"	INTEGER,
            \"BirthDate\"	TEXT,
            PRIMARY KEY(\"Id\")
        );",
    )
}

pub fn insert_tournament(
    tournament: &Tournament,
    connection: &Connection,
) -> rusqlite::Result<usize> {
    connection.execute(
        "
        INSERT INTO \"Tournament\" VALUES
        (
            NULL,
            (?1),
            (?2),
            (?3),
            (?4),
            (?5),
            (?6),
            (?7),
            (?8),
            (?9),
            (?10),
            (?11),
            (?12)
        )
    ",
        params![
            &tournament.name,
            &tournament.city,
            &tournament.fide_federation,
            &tournament.date_start,
            &tournament.date_end,
            &tournament.type_tournament,
            &tournament.format,
            &tournament.chief_arbiter,
            &tournament.deputy_chief_arbiter,
            &tournament.time_control,
            &tournament.number_rounds,
            0,
        ],
    )
}

pub fn select_tournament(connection: &Connection) -> rusqlite::Result<Tournament> {
    connection.query_row("SELECT * FROM \"Tournament\"", [], |row| {
        Ok(Tournament {
            name: row.get(1)?,
            city: row.get(2)?,
            fide_federation: row.get(3)?,
            date_start: row.get(4)?,
            date_end: row.get(5)?,
            type_tournament: row.get(6)?,
            format: row.get(7)?,
            chief_arbiter: row.get(8)?,
            deputy_chief_arbiter: row.get(9)?,
            time_control: row.get(10)?,
            number_rounds: row.get(11)?,
            current_round: row.get(12)?,
        })
    })
}

pub fn select_players(connection: &Connection) -> rusqlite::Result<Vec<Player>> {
    let mut query: rusqlite::Statement<'_> = connection.prepare("SELECT * FROM \"Players\"")?;
    let players_iter = query.query_map([], |row| Ok(Player { name: row.get(1)?, sex: row.get(2)?, title: row.get(3)?, rating: row.get(4)?, fide_federation: row.get(5)?, fide_number: row.get(6)?, birth_date: row.get(7)? }))?;
    Ok(players_iter
        .map(|player| player.unwrap())
        .into_iter()
        .collect())
}

pub fn insert_player(connection: &Connection, player: &Player) -> rusqlite::Result<usize> {
    connection.execute(
        "
        INSERT INTO \"Players\" VALUES
        (
            NULL,
            (?1),
            (?2),
            (?3),
            (?4),
            (?5),
            (?6),
            (?7)
        )
    ",
        params![
            &player.name,
            &player.sex,
            &player.title,
            player.rating,
            &player.fide_federation,
            player.fide_number,
            &player.birth_date,
        ],
    )
}
