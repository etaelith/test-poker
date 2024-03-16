use crate::{
    data_structs::ResponseStatus,
    db::{
        config::connect_database,
        utils::{insert_user, user_exists},
    },
};

use rusqlite::{params, Result};

pub fn add_winner(
    bounty_winner: bool,
    user_id: i64,
    tag_user: &str,
    points: i64,
    date_tournament: i64,
) -> Result<ResponseStatus, rusqlite::Error> {
    match connect_database() {
        Ok(conn) => {
            let exists = user_exists(&conn, user_id)?;
            if exists {
                conn.execute(
                    "INSERT INTO bounties (bounty_winner, user_id, tournament_id,coins ) VALUES (?1,?2,?3,?4)",
                    params![bounty_winner, user_id, date_tournament, points],
                )?;
                let _ = update_bounties(user_id);
                Ok(ResponseStatus {
                    success: true,
                    success_description: None,
                    error_message: None,
                })
            } else {
                let insert_result = insert_user(&conn, user_id, tag_user);
                if insert_result.is_ok() {
                    conn.execute(
                        "INSERT INTO bounties (bounty_winner, user_id, tournament_id,coins ) VALUES (?1,?2,?3,?4)",
                        params![bounty_winner, user_id, date_tournament, points],
                    )?;
                    let _ = update_bounties(user_id);
                }
                return insert_result;
            }
        }
        Err(err) => Err(err),
    }
}
fn update_bounties(user_id: i64) -> Result<ResponseStatus, rusqlite::Error> {
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
