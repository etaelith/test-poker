use rusqlite::{params, Result};

use crate::{
    data_structs::{ResponseStatus, TopTen},
    db_config::connect_database,
};

pub fn create_or_sum(
    tag_user: &str,
    id_user: i64,
    chips: i64,
) -> Result<ResponseStatus, rusqlite::Error> {
    let conn = connect_database();

    match conn {
        Ok(conn) => {
            // Verificar si el usuario ya existe
            let user_exists: bool = conn
                .query_row(
                    "SELECT COUNT(*) FROM users WHERE user_id = ?1",
                    params![id_user],
                    |row| row.get(0),
                )
                .unwrap_or(0)
                > 0;

            if user_exists {
                // Usuario existe, actualizar chips
                match conn.execute(
                    "UPDATE users SET points = points + ?1, updated_at = CURRENT_TIMESTAMP WHERE user_id = ?2",
                    params![chips, id_user],
                ) {
                    Ok(_) => Ok(ResponseStatus {
                        success: true,                        
                        success_description: None,
                        error_message: None,
                    }),
                    Err(err) => Ok(ResponseStatus {
                        success: false,                        success_description: None,
                        error_message: Some(err.to_string()),
                    }),
                }
            } else {
                // Usuario no existe, insertar nuevo registro
                match conn.execute(
                    "INSERT INTO users (user_id, user_name, points, created_at, updated_at) VALUES (?1, ?2, ?3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
                    params![id_user, tag_user, chips],
                ) {
                    Ok(_) => Ok(ResponseStatus {
                        success: true,                        success_description: None,
                        error_message: None,
                    }),
                    Err(err) => Ok(ResponseStatus {
                        success: false,                        success_description: None,
                        error_message: Some(err.to_string()),
                    }),
                }
            }
        }
        Err(conn_err) => Err(conn_err),
    }
}
pub fn update_amount(tag_user: i64, chips: i64) -> Result<ResponseStatus, rusqlite::Error> {
    let conn = connect_database();
    match conn {
        Ok(conn) => {
            match conn.execute(
                "UPDATE users SET points = points - ?1 WHERE user_id = ?2",
                params![chips, tag_user],
            ) {
                Ok(_) => Ok(ResponseStatus {
                    success: true,
                    success_description: None,
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
                Ok(data) => {
                    Ok(ResponseStatus {
                        success: true,
                        success_description: Some(serde_json::to_string(&data).unwrap()),
                        error_message: None,
                    })
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
