use rusqlite::{params, Connection, Result};

use crate::data_structs::ResponseStatus;

use super::config::connect_database;

pub fn user_exists(conn: &Connection, id_user: i64) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT EXISTS(SELECT 1 FROM users WHERE user_id = ?1)")?;
    let exists: bool = stmt.query_row(params![id_user], |row| row.get(0))?;
    Ok(exists)
}
pub fn idiot_exists(conn: &Connection, id_user: i64) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT EXISTS(SELECT 1 FROM bosses WHERE user_id = ?1)")?;
    let exists: bool = stmt.query_row(params![id_user], |row| row.get(0))?;
    Ok(exists)
}
pub fn tournament_exists(tournament_date: i64) -> Result<bool, rusqlite::Error> {
    match connect_database() {
        Ok(conn) => {
            let mut stmt = conn
                .prepare("SELECT EXISTS(SELECT 1 FROM tournaments WHERE tournament_date = ?1)")?;
            let exists: bool = stmt.query_row(params![tournament_date], |row| row.get(0))?;
            Ok(exists)
        }
        Err(e) => Err(e),
    }
}
pub fn insert_idiot(conn: &Connection, id_user: i64, tag_user: &str) -> Result<ResponseStatus> {
    match user_exists(conn, id_user){
        Ok(true) => {
            match idiot_exists(conn, id_user){
                Ok(false) => {
                    match conn.execute("INSERT INTO bosses (user_id, user_name, created_at,updated_at) VALUES (?1, ?2, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP", params![id_user,tag_user],){
                        Ok(_) => Ok(ResponseStatus {
                            success: true,
                            success_description: Some(format!("Idiotita spawn")),
                            error_message: None,
                        }),
                        Err(err) => Ok(ResponseStatus {
                            success: false,
                            success_description: None,
                            error_message: Some(err.to_string()),
                        }),
                    }
                }
                Ok(true) => {
                    Ok(ResponseStatus {
                        success: true,
                        success_description: Some(format!("Idiotita exist")),
                        error_message: None,
                    })
                }
                Err(err) => Ok(ResponseStatus {
                    success: false,
                    success_description: None,
                    error_message: Some(err.to_string()),
                }),
            }
        }
        Ok(false) => {
            let _ = insert_user(conn, id_user, tag_user);
            match conn.execute("INSERT INTO bosses (user_id, user_name, created_at,updated_at) VALUES (?1, ?2, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)", params![id_user,tag_user],){
                Ok(_) => Ok(ResponseStatus {
                    success: true,
                    success_description: Some(format!("Idiotita spawn")),
                    error_message: None,
                }),
                Err(err) => Ok(ResponseStatus {
                    success: false,
                    success_description: None,
                    error_message: Some(err.to_string()),
                }),
            }
        }
        Err(err) => Ok(ResponseStatus {
            success: false,
            success_description: None,
            error_message: Some(err.to_string()),
        }),
    }
}
pub fn insert_user(conn: &Connection, id_user: i64, tag_user: &str) -> Result<ResponseStatus> {
    match conn.execute(
        "INSERT INTO users (user_id, user_name, points, created_at, updated_at) VALUES (?1, ?2, ?3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
        params![id_user, tag_user, 0],
    ) {
        Ok(_) => Ok(ResponseStatus {
            success: true,
            success_description: Some(format!("User created with 0 points")),
            error_message: None,
        }),
        Err(err) => Ok(ResponseStatus {
            success: false,
            success_description: None,
            error_message: Some(err.to_string()),
        }),
    }
}
pub fn insert_twitch_tag(
    id_user: i64,
    user_twitch: &str,
) -> Result<ResponseStatus, rusqlite::Error> {
    let conn = connect_database()?;
    match conn.execute(
        "UPDATE users SET user_twitch = ?1 WHERE user_id = ?2",
        params![user_twitch, id_user],
    ) {
        Ok(_) => Ok(ResponseStatus {
            success: true,
            success_description: Some(format!("Agreed {}", user_twitch)),
            error_message: None,
        }),
        Err(err) => Err(err.into()),
    }
}

pub fn insert_twitch(
    user_id: i64,
    user_name: String,
    twitch_connection_name: String,
) -> Result<ResponseStatus, rusqlite::Error> {
    match connect_database() {
        Ok(conn) => match user_exists(&conn, user_id.into()) {
            Ok(exists) => {
                if exists {
                    let _ = insert_twitch_tag(user_id, &twitch_connection_name);
                    Ok(ResponseStatus {
                        success: true,
                        success_description: Some("Twitch tag inserted successfully".to_string()),
                        error_message: None,
                    })
                } else {
                    let _ = insert_user(&conn, user_id.clone(), &user_name);
                    let _ = insert_twitch_tag(user_id.clone(), &twitch_connection_name);
                    Ok(ResponseStatus {
                        success: true,
                        success_description: Some(
                            "User and Twitch tag inserted successfully".to_string(),
                        ),
                        error_message: None,
                    })
                }
            }
            Err(e) => {
                eprintln!("Error checking if user exists: {}", e);
                Err(e)
            }
        },
        Err(conn_err) => {
            eprintln!("Error connecting db{}", conn_err);
            Err(conn_err)
        }
    }
}
