use crate::models::{Pairing, PairingKind, Player, Tournament, GameInfo, ByeInfo};
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
        CREATE TABLE \"GameByRound\" (
            \"RoundId\" INTEGER NOT NULL,
            \"WhiteId\" INTEGER NOT NULL,
            \"BlackId\" INTEGER NOT NULL,
            \"WhiteResult\" TEXT,
            \"BlackResult\" TEXT,
            \"Ongoing\" INTEGER NOT NULL
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
        params![&tournament.name, &tournament.number_rounds,],
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
        params![&player.name, player.rating,],
    )
}

pub fn select_matches(connection: &Connection) -> Result<Vec<Pairing>> {
    let mut statement = connection.prepare(
        "
        SELECT Round.Number, White.*, Black.*, GameByRound.WhiteResult, GameByRound.BlackResult
        FROM GameByRound
        INNER JOIN Round ON Round.Id = GameByRound.RoundId
        INNER JOIN Player AS White ON White.Id = GameByRound.WhiteId
        INNER JOIN Player AS Black ON Black.Id = GameByRound.BlackId
        "
    )?;
    let matches_iter = statement.query_map(params![], |row| {
        Ok(Pairing {
            number_round: row.get(0)?,
            kind: PairingKind::Game(GameInfo {
                white_player: Player { id: row.get(1)?, name: row.get(2)?, points: row.get(3)?, rating: row.get(4)? },
                black_player: Player { id: row.get(5)?, name: row.get(6)?, points: row.get(7)?, rating: row.get(8)? },
                white_result: row.get(9)?,
                black_result: row.get(10)?,
            }),
        })
    })?;
    Ok(matches_iter
        .filter(|m| m.is_ok())
        .map(|m| m.unwrap())
        .collect())
}

pub fn select_byes(connection: &Connection) -> Result<Vec<Pairing>> {
    let mut statement = connection.prepare(
            "
            SELECT Round.Number, Player.*, ByeByRound.ByePoint
            FROM ByeByRound
            INNER JOIN Round ON Round.Id = ByeByRound.RoundId
            INNER JOIN Player ON Player.Id = ByeByRound.PlayerId
            "
        )?;
    let byes_iter = statement.query_map(params![], |row| {
        Ok(Pairing {
            number_round: row.get(0)?,
            kind: PairingKind::Bye(ByeInfo {
                player: Player { id: row.get(1)?, name: row.get(2)?, points: row.get(3)?, rating: row.get(4)? },
                bye_point: row.get(5)?,
            }),
        })
    })?;
    Ok(byes_iter
        .filter(|b| b.is_ok())
        .map(|b| b.unwrap())
        .collect())
}

pub fn insert_pairing(pairing: &Pairing, connection: &Connection) -> Result<usize> {
    match &pairing.kind {
        PairingKind::Bye(ByeInfo {
            player,
            bye_point,
        }) => connection.execute(
            "
            INSERT INTO ByeByRound VALUES
            (
                (SELECT Id FROM Round WHERE Number = (?1) LIMIT 1),
                (?2),
                (?3)
            )
            ",
            params![pairing.number_round, player.id, bye_point],
        ),
        PairingKind::Game(GameInfo {
            white_player,
            black_player,
            white_result,
            black_result,
        }) => connection.execute(
            "
            INSERT INTO GameByRound VALUES
            (
                (SELECT Id FROM Round WHERE Number = (?1) LIMIT 1),
                (?2),
                (?3),
                (?4),
                (?5),
                TRUE
            )",
            params![
                pairing.number_round,
                white_player.id,
                black_player.id,
                white_result,
                black_result
            ],
        ),
    }
}

pub fn select_pairings(connection: &Connection) -> Result<Vec<Pairing>> {
    let mut matches = select_matches(&connection)?;
    let byes = select_byes(&connection)?;
    matches.extend(byes);
    Ok(matches)
}

pub fn update_current_round(next_round: u16, connection: &Connection) -> Result<()> {
    connection.execute(
        "UPDATE Tournament SET CurrentRound = (?1)",
        params![next_round],
    )?;
    Ok(())
}

pub fn insert_round(number_round: u16, connection: &Connection) -> Result<()> {
    connection.execute(
        "INSERT INTO Round VALUES
        (
            NULL,
            (?1),
            NULL
        )",
        params![number_round],
    )?;
    Ok(())
}

pub fn select_ongoing_games(connection: &Connection) -> Result<Vec<Pairing>> {
    let mut stmnt = connection.prepare(
        "
        SELECT Round.Number, White.*, Black.*, GameByRound.WhiteResult, GameByRound.BlackResult
        FROM GameByRound
        INNER JOIN Round ON GameByRound.RoundId=Round.Id
        INNER JOIN Player AS White ON White.Id = GameByRound.WhiteId
        INNER JOIN Player AS Black ON Black.Id = GameByRound.BlackId
        WHERE GameByRound.Ongoing=TRUE
        "
    )?;
    let games_iter = stmnt.query_map(params![], |row| {
        Ok(Pairing {
            number_round: row.get(0)?,
            kind: PairingKind::Game(GameInfo {
                white_player: Player { id: row.get(1)?, name: row.get(2)?, points: row.get(3)?, rating: row.get(4)? },
                black_player: Player { id: row.get(5)?, name: row.get(6)?, points: row.get(7)?, rating: row.get(8)? },
                white_result: row.get(9)?,
                black_result: row.get(10)?,
            }),
        })
    })?;
    Ok(games_iter
        .filter(|g| g.is_ok())
        .map(|g| g.unwrap())
        .collect())
}

pub fn select_matches_by_round(number_round: u16, connection: &Connection) -> Result<Vec<Pairing>> {
    let mut statement = connection.prepare(
        "
        SELECT Round.Number, White.*, Black.*, GameByRound.WhiteResult, GameByRound.BlackResult
        FROM GameByRound
        INNER JOIN Round ON GameByRound.RoundId=Round.Id
        INNER JOIN Player AS White ON White.Id = GameByRound.WhiteId
        INNER JOIN Player AS Black ON Black.Id = GameByRound.BlackId
        WHERE Round.Number=(?1)"
    )?;
    let matches_iter = statement.query_map(params![number_round], |row| {
        Ok(Pairing {
            number_round: row.get(0)?,
            kind: PairingKind::Game(GameInfo {
                white_player: Player { id: row.get(1)?, name: row.get(2)?, points: row.get(3)?, rating: row.get(4)? },
                black_player: Player { id: row.get(5)?, name: row.get(6)?, points: row.get(7)?, rating: row.get(8)? },
                white_result: row.get(9)?,
                black_result: row.get(10)?,
            }),
        })
    })?;
    Ok(matches_iter
        .filter(|m| m.is_ok())
        .map(|m| m.unwrap())
        .collect())
}

pub fn select_byes_by_round(number_round: u16, connection: &Connection) -> Result<Vec<Pairing>> {
    let mut statement = connection.prepare(
        "
        SELECT Round.Number, Player.*, ByeByRound.ByePoint
        FROM ByeByRound
        INNER JOIN Round ON Round.Id = ByeByRound.RoundId
        INNER JOIN Player ON Player.Id = ByeByRound.PlayerId
        WHERE Round.Number = (?1)
        "
    )?;
    let byes_iter = statement.query_map(params![number_round], |row| {
        Ok(Pairing {
            number_round: row.get(0)?,
            kind: PairingKind::Bye(ByeInfo {
                player: Player { id: row.get(1)?, name: row.get(2)?, points: row.get(3)?, rating: row.get(4)? },
                bye_point: row.get(5)?,
            }),
        })
    })?;
    Ok(byes_iter
        .filter(|b| b.is_ok())
        .map(|b| b.unwrap())
        .collect())
}

pub fn select_pairings_by_round(
    number_round: u16,
    connection: &Connection,
) -> Result<Vec<Pairing>> {
    let mut matches = select_matches_by_round(number_round, &connection)?;
    let byes = select_byes_by_round(number_round, &connection)?;
    matches.extend(byes);
    Ok(matches)
}

pub fn select_last_inserted_player(connection: &Connection) -> Result<Player> {
    connection.query_row(
        "SELECT * FROM Player ORDER BY Id DESC LIMIT 1", params![], |row| 
        Ok(Player { id: row.get(0)?, name: row.get(1)?, points: row.get(2)?, rating: row.get(3)? })
    )
}
