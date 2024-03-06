use rusqlite::{params, Result};

use crate::{data_structs::ResponseStatus, db_config::connect_database};

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
                        error_message: None,
                    }),
                    Err(err) => Ok(ResponseStatus {
                        success: false,
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
                        success: true,
                        error_message: None,
                    }),
                    Err(err) => Ok(ResponseStatus {
                        success: false,
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
                    error_message: None,
                }),
                Err(err) => Ok(ResponseStatus {
                    success: false,
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

            let users_data: Result<Vec<(String, i32)>, rusqlite::Error> = statement
                .query_map(params![], |row| {
                    Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?))
                })?
                .collect();

            println!("Datos users: {:?}", users_data);
            Ok(ResponseStatus {
                success: true,
                error_message: None,
            })
        }
        Err(conn_err) => Err(conn_err),
    }
}
