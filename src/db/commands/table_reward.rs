use rusqlite::{params, Result};

use crate::{
    data_structs::ResponseStatus,
    db::{
        config::connect_database,
        utils::{insert_user, update_points, user_exists},
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
            println!("Error: {}", conn_err)
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
                    "UPDATE reward SET points = points ?1 WHERE user_id = ?2 AND tournament_id = ?3",
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
            println!("Error: {}", conn_err)
        }
    }
    Ok(ResponseStatus {
        success: true,
        success_description: None,
        error_message: None,
    })
}
