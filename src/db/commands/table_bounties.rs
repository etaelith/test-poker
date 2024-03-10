use crate::{data_structs::ResponseStatus, db::config::connect_database};

use rusqlite::{params, Result};

pub fn add_winner(
    bounty_winner: bool,
    user_id: i64,
    points: i64,
    date_tournament: i64,
) -> Result<ResponseStatus, rusqlite::Error> {
    let conn = connect_database();
    match conn {
        Ok(conn) => {
            match conn.execute(
                "INSERT INTO bounties (bounty_winner, user_id, tournament_id,coins ) VALUES (?1,?2,?3,?4)",
                params![bounty_winner, user_id, date_tournament, points],
            ) {
                Ok(_) => Ok(ResponseStatus {
                    success: true,
                    success_description: Some(format!("Tournament Registry")),
                    error_message: None,
                }),
                Err(err) => Ok(ResponseStatus {
                    success: false,
                    success_description: None,
                    error_message: Some(err.to_string()),
                }),
            }
        }
        Err(err) => Err(err)

    }
}
