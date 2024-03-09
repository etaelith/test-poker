use rusqlite::{params, Result};

use crate::{data_structs::ResponseStatus, db::config::connect_database};

pub fn insert_reward(
    user_id: i64,
    chips: i64,
    tournament_id: i64,
) -> Result<ResponseStatus, rusqlite::Error> {
    let conn = connect_database();
    match conn {
        Ok(conn) => {
            match conn.execute(
                "UPDATE rewardp SET points = points - ?1 WHERE user_id = ?2 AND tournament_id = ?3",
                params![chips, user_id, tournament_id],
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

pub fn subtract_reward(
    user_id: i64,
    chips: i64,
    tournament_id: i64,
) -> Result<ResponseStatus, rusqlite::Error> {
    let conn = connect_database();
    match conn {
        Ok(conn) => {
            match conn.execute(
                "UPDATE rewardp SET points = points - ?1 WHERE user_id = ?2 AND tournament_id = ?3",
                params![chips, user_id, tournament_id],
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
