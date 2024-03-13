use crate::{
    data_structs::{ResponseStatus, Tournaments},
    db::config::connect_database,
    discord::utils::format_unix_timestamp,
};

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
            success_description: Some(format!("Tournament Registred")),
            error_message: None,
        }),
        Err(err) => Ok(ResponseStatus {
            success: false,
            success_description: None,
            error_message: Some(err.to_string()),
        }),
    }
}

pub fn get_tournaments_date() -> Result<ResponseStatus, rusqlite::Error> {
    match connect_database() {
        Ok(conn) => {
            let mut statement = match conn
                .prepare("SELECT tournament_date FROM tournaments ORDER BY tournament_date DESC;")
            {
                Ok(stmt) => stmt,
                Err(err) => return Err(err),
            };
            let mut position_counter = 0;
            let tournaments_data: Result<Vec<Tournaments>, rusqlite::Error> = statement
                .query_map(params![], |row| {
                    position_counter += 1;
                    let timestamp: i64 = row.get(0)?;
                    match format_unix_timestamp(timestamp) {
                        Ok(date) => Ok(Tournaments {
                            tournament_date: date,
                        }),
                        Err(_) => Err(rusqlite::Error::InvalidQuery),
                    }
                })
                .and_then(|mapped_rows| mapped_rows.collect());
            match tournaments_data {
                Ok(data) => Ok(ResponseStatus {
                    success: true,
                    success_description: Some(serde_json::to_string(&data).unwrap()),
                    error_message: None,
                }),
                Err(err) => Ok(ResponseStatus {
                    success: false,
                    success_description: None,
                    error_message: Some(err.to_string()),
                }),
            }
        }
        Err(conn_err) => Err(conn_err),
    }
}
pub fn get_last_tournament() -> Result<ResponseStatus, rusqlite::Error> {
    match connect_database() {
        Ok(conn) => {
            let tournament_date: i64 = conn.query_row(
                "SELECT last(tournament_date) FROM tournaments",
                params![],
                |row| row.get(0),
            )?;

            let date: String =
                format_unix_timestamp(tournament_date).expect("Error on format_unix");

            let data = Tournaments {
                tournament_date: date,
            };

            Ok(ResponseStatus {
                success: true,
                success_description: Some(serde_json::to_string(&data).unwrap()),
                error_message: None,
            })
        }
        Err(conn_err) => Err(conn_err),
    }
}
