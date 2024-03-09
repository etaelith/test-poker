use crate::data_structs::ResponseStatus;
use chrono::prelude::*;
use rusqlite::{params, Connection, Result};

pub fn add_tournament(
    conn: &Connection,
    date_tournament: NaiveDate,
) -> Result<ResponseStatus, rusqlite::Error> {
    match conn.execute(
        "INSERT INTO tournaments (tournament_date ) VALUES (strftime('%s', ?1))",
        params![date_tournament.format("%Y-%m-%d").to_string()],
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
