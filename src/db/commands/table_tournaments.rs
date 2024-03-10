use crate::data_structs::ResponseStatus;

use rusqlite::{params, Connection, Result};

pub fn add_tournament(
    conn: &Connection,
    date_tournament: i64,
) -> Result<ResponseStatus, rusqlite::Error> {
    match conn.execute(
        "INSERT INTO tournaments (tournament_date ) VALUES (?1)",
        params![date_tournament],
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
