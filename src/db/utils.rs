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

pub fn update_points(user_id: i64) -> Result<ResponseStatus, rusqlite::Error> {
    match connect_database() {
        Ok(conn) => {
            match conn.execute(
                "UPDATE users
                SET points = COALESCE((SELECT sum(points) FROM reward WHERE user_id = ?1), 0)
                WHERE user_id = ?1;",
                params![user_id],
            ) {
                Ok(_) => Ok(ResponseStatus {
                    success: true,
                    success_description: Some(format!("Updated points to {user_id}")),
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

pub fn update_bounties(user_id: i64) -> Result<ResponseStatus, rusqlite::Error> {
    match connect_database() {
        Ok(conn) => {
            match conn.execute(
                "UPDATE users
                SET wins = (
                    SELECT COUNT(*)
                    FROM bounties
                    WHERE user_id = ?1 AND bounty_winner = TRUE
                )
                WHERE user_id = ?1;",
                params![user_id],
            ) {
                Ok(_) => match conn.execute(
                    "UPDATE users
                    SET bounties = (
                        SELECT COUNT(*)
                        FROM bounties
                        WHERE user_id = ?1 AND bounty_winner = FALSE
                    )
                    WHERE user_id = ?1;",
                    params![user_id],
                ) {
                    Ok(_) => Ok(ResponseStatus {
                        success: true,
                        success_description: Some(format!("Bounties updated for: {user_id}")),
                        error_message: None,
                    }),
                    Err(err) => Ok(ResponseStatus {
                        success: false,
                        success_description: None,
                        error_message: Some(err.to_string()),
                    }),
                },
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
