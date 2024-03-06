use rusqlite::{params, Connection, Result};

use crate::{data_structs::ResponseStatus, db_config::connect_database};

pub fn create_or_sum(
    mut conn: Connection,
    tag_user: &str,
    id_user: &i64,
    chips: i64,
) -> Result<i64, rusqlite::Error> {
    let tx = conn.transaction()?;
    tx.execute(
        "INSERT INTO users (tag_user, id_user, chips) VALUES (?1,?2,?3)",
        params![tag_user, id_user, chips],
    )?;
    tx.commit()?;
    Ok((chips))
}

pub fn update_amount(tag_user: &str, chips: i64) -> Result<ResponseStatus, rusqlite::Error> {
    let conn = connect_database();
    match conn {
        Ok(conn) => {
            match conn.execute(
                "UPDATE users SET chips = chips - ?1 WHERE id_user = ?2",
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
