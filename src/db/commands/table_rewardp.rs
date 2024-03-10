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
    let conn = connect_database();
    println!(
        "User_id: {}, \n tag_id: {}, \n chips: {},\n tournament_id: {}",
        user_id, tag_user, chips, tournament_id
    );
    match conn {
        Ok(conn) => {
            let exists: bool = user_exists(&conn, user_id)?;
            if exists {
                println!("Existe el usuario: {}", exists);
                conn.execute(
                    "INSERT INTO rewardp (points, user_id, tournament_id) VALUES (?1, ?2, ?3)",
                    params![chips, user_id, tournament_id],
                )?;
            } else {
                println!("No existe el usuario: {}", exists);
                let insert_result = insert_user(&conn, user_id, tag_user);
                if insert_result.is_ok() {
                    println!("Existe el usuario ahora aftert .is_ok(): ");
                    conn.execute(
                        "INSERT INTO rewardp (points, user_id, tournament_id) VALUES (?1, ?2, ?3)",
                        params![chips, user_id, tournament_id],
                    )?;
                }
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
