use rusqlite::{params, Result};

use crate::{
    data_structs::ResponseStatus,
    db::{
        config::connect_database,
        utils::{insert_user, user_exists},
    },
};

pub fn insert_reward(
    user_id: i64,
    tag_user: &str,
    chips: i64,
    tournament_id: i64,
) -> Result<ResponseStatus, rusqlite::Error> {
    match connect_database() {
        Ok(conn) => {
            let exists: bool = user_exists(&conn, user_id)?;
            if exists {
                conn.execute(
                    "INSERT INTO reward (points, user_id, tournament_id) VALUES (?1, ?2, ?3)",
                    params![chips, user_id, tournament_id],
                )?;
                let _ = update_points(user_id);
            } else {
                let insert_result = insert_user(&conn, user_id, tag_user);
                if insert_result.is_ok() {
                    conn.execute(
                        "INSERT INTO reward (points, user_id, tournament_id) VALUES (?1, ?2, ?3)",
                        params![chips, user_id, tournament_id],
                    )?;
                }
                let _ = update_points(user_id);
                return insert_result;
            }
        }
        Err(conn_err) => {
            eprintln!("Error: {}", conn_err)
        }
    }
    Ok(ResponseStatus {
        success: true,
        success_description: None,
        error_message: None,
    })
}

pub fn substract_reward(
    user_id: i64,
    tag_user: &str,
    chips: i64,
    tournament_id: i64,
) -> Result<ResponseStatus, rusqlite::Error> {
    match connect_database() {
        Ok(conn) => {
            let exists: bool = user_exists(&conn, user_id)?;
            if exists {
                conn.execute(
                    "UPDATE reward SET points = points - ?1 WHERE user_id = ?2 AND tournament_id = ?3",
                    params![chips, user_id, tournament_id],
                )?;
                let _ = update_points(user_id);
            } else {
                let insert_result = insert_user(&conn, user_id, tag_user);
                let _ = update_points(user_id);
                return insert_result;
            }
        }
        Err(conn_err) => {
            eprintln!("Error: {}", conn_err)
        }
    }
    Ok(ResponseStatus {
        success: true,
        success_description: None,
        error_message: None,
    })
}

pub fn delete_reward(user_id: i64, tournament_id: i64) -> Result<ResponseStatus, rusqlite::Error> {
    match connect_database() {
        Ok(conn) => {
            let exists: bool = user_exists(&conn, user_id)?;

            if exists {
                conn.execute(
                    "DELETE FROM reward WHERE user_id = ?1 AND tournament_id = ?2",
                    params![user_id, tournament_id],
                )?;
                let _ = update_points(user_id);

                return Ok(ResponseStatus {
                    success: true,
                    success_description: None,
                    error_message: None,
                });
            } else {
                return Ok(ResponseStatus {
                    success: false,
                    success_description: None,
                    error_message: Some("User not found".to_string()),
                });
            }
        }
        Err(conn_err) => {
            eprintln!("Error: {}", conn_err)
        }
    }
    Ok(ResponseStatus {
        success: true,
        success_description: None,
        error_message: None,
    })
}
fn update_points(user_id: i64) -> Result<ResponseStatus, rusqlite::Error> {
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
