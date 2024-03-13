use crate::{
    data_structs::{ResponseStatus, TopTen, Tournaments},
    db::config::connect_database,
    discord::utils::format_unix_timestamp,
};

use rusqlite::{params, Connection, Result};
// List of tournaments
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
// Top10 specific tournament
pub fn get_top_tournament(tournament_id: i64) -> Result<ResponseStatus, rusqlite::Error> {
    match connect_database() {
        Ok(conn) => {
            let mut statement = match conn.prepare(
                "
                    SELECT u.user_name, r.points 
                    FROM reward r
                    JOIN users u ON u.user_id = r.user_id
                    WHERE r.tournament_id = ?
                    ORDER BY r.points DESC 
                    LIMIT 10;
                ",
            ) {
                Ok(stmt) => stmt,
                Err(err) => return Err(err),
            };
            let mut position_counter = 0;
            let users_data: Result<Vec<TopTen>, rusqlite::Error> = statement
                .query_map(params![tournament_id], |row| {
                    position_counter += 1;
                    Ok(TopTen {
                        name: row.get(0)?,
                        position: position_counter,
                        points: row.get(1)?,
                    })
                })
                .and_then(|mapped_rows| mapped_rows.collect());

            match users_data {
                Ok(data) => {
                    let response = ResponseStatus {
                        success: true,
                        success_description: Some(serde_json::to_string(&data).unwrap()),
                        error_message: None,
                    };
                    Ok(response)
                }
                Err(err) => {
                    eprintln!("Error al obtener datos de usuarios: {:?}", err);
                    Ok(ResponseStatus {
                        success: false,
                        success_description: None,
                        error_message: Some(err.to_string()),
                    })
                }
            }
        }
        Err(conn_err) => Err(conn_err),
    }
}

pub fn get_top() -> Result<ResponseStatus, rusqlite::Error> {
    match connect_database() {
        Ok(conn) => {
            let mut statement = match conn
                .prepare("SELECT user_name, points FROM users ORDER BY points DESC LIMIT 10;")
            {
                Ok(stmt) => stmt,
                Err(err) => return Err(err),
            };
            let mut position_counter = 0;
            let users_data: Result<Vec<TopTen>, rusqlite::Error> = statement
                .query_map(params![], |row| {
                    position_counter += 1;
                    Ok(TopTen {
                        name: row.get(0)?,
                        position: position_counter,
                        points: row.get(1)?,
                    })
                })
                .and_then(|mapped_rows| mapped_rows.collect());
            match users_data {
                Ok(data) => Ok(ResponseStatus {
                    success: true,
                    success_description: Some(serde_json::to_string(&data).unwrap()),
                    error_message: None,
                }),
                Err(err) => {
                    eprintln!("Error al obtener datos de usuarios: {:?}", err);
                    Ok(ResponseStatus {
                        success: false,
                        success_description: None,
                        error_message: Some(err.to_string()),
                    })
                }
            }
        }
        Err(conn_err) => Err(conn_err),
    }
}

// Role Needed
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
