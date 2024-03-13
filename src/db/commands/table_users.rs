use rusqlite::{params, Result};

use crate::{
    data_structs::ResponseStatus,
    db::{
        config::connect_database,
        utils::{insert_user, user_exists},
    },
};

pub fn get_user_rank(tag_user: &str, id_user: i64) -> Result<ResponseStatus, rusqlite::Error> {
    match connect_database() {
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

pub fn verified_bitmex(
    id_user: i64,
    state: bool,
    tag_user: &str,
) -> Result<ResponseStatus, rusqlite::Error> {
    let conn = connect_database()?;
    let exists = user_exists(&conn, id_user)?;
    if exists {
        conn.execute(
            "UPDATE users SET bitmex = ?1 WHERE user_id = ?2",
            params![state, id_user],
        )?;
        Ok(ResponseStatus {
            success: true,
            success_description: Some(format!("Tournament Registred")),
            error_message: None,
        })
    } else {
        let insert_result = insert_user(&conn, id_user, tag_user);
        if insert_result.is_ok() {
            conn.execute(
                "UPDATE users SET bitmex = ?1 WHERE user_id = ?2",
                params![state, id_user],
            )?;
        }
        insert_result
    }
}
