use rusqlite::{params, Connection, Result};

use crate::data_structs::ResponseStatus;

pub fn user_exists(conn: &Connection, id_user: i64) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT EXISTS(SELECT 1 FROM users WHERE user_id = ?1)")?;
    let exists: bool = stmt.query_row(params![id_user], |row| row.get(0))?;
    Ok(exists)
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

pub fn insert_points(
    conn: &Connection,
    id_user: i64,
    chips: i64,
) -> Result<ResponseStatus, rusqlite::Error> {
    match conn.execute(
        "UPDATE users SET points = points + ?1, updated_at = CURRENT_TIMESTAMP WHERE user_id = ?2",
        params![chips, id_user],
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
