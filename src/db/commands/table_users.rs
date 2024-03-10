use rusqlite::{params, Result};

use crate::{
    data_structs::{ResponseStatus, TopTen},
    db::{
        config::connect_database,
        utils::{insert_user, user_exists},
    },
};

pub fn substract_amount(user_id: i64, chips: i64) -> Result<ResponseStatus, rusqlite::Error> {
    let conn = connect_database();
    match conn {
        Ok(conn) => {
            match conn.execute(
                "UPDATE users SET points = points - ?1 WHERE user_id = ?2",
                params![chips, user_id],
            ) {
                Ok(_) => Ok(ResponseStatus {
                    success: true,
                    success_description: Some(format!("Substracted {chips} points to {user_id}")),
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

pub fn get_user_rank(tag_user: &str, id_user: i64) -> Result<ResponseStatus, rusqlite::Error> {
    let conn = connect_database();
    match conn {
        Ok(conn) => {
            let exists: bool = user_exists(&conn, id_user)?;
            if exists {
                let mut stmt = conn.prepare("SELECT points, (SELECT COUNT(*) FROM users WHERE points > u.points) + 1 AS rank FROM users u WHERE user_id = ?1")?;
                let mut rows = stmt.query(params![id_user])?;
                if let Some(row) = rows.next()? {
                    let points: i64 = row.get(0)?;
                    let rank: i64 = row.get(1)?;
                    Ok(ResponseStatus {
                        success: true,
                        success_description: Some(format!(
                            "User has {} points and is ranked {}.",
                            points, rank
                        )),
                        error_message: None,
                    })
                } else {
                    Err(rusqlite::Error::QueryReturnedNoRows)
                }
            } else {
                return insert_user(&conn, id_user, tag_user);
            }
        }
        Err(conn_err) => Err(conn_err),
    }
}
