use rusqlite::{params, Connection, Result};

use crate::{
    data_structs::ResponseStatus,
    db::{commands::table_reward::update_points, config::connect_database},
};

pub fn insert_users_rewards(
    campos: Vec<&str>,
    tournament_id: i64,
) -> Result<ResponseStatus, rusqlite::Error> {
    match connect_database() {
        Ok(conn) => {
            let mut index = 0;
            while index < campos.len() {
                if index + 13 <= campos.len() {
                    let jugador = &campos[index..index + 13];

                    if let Some(user_id) = check_user(&conn, jugador[1])? {
                        let points: i64 = jugador[0].parse().unwrap();
                        let invert_points = 11 - points;
                        println!("user_id: {}, points: {}", user_id, points);
                        conn.execute(
                            "INSERT INTO reward (points, user_id, tournament_id) VALUES (?1, ?2, ?3)",
                            params![invert_points, user_id, tournament_id],
                        )?;
                        let _ = update_points(user_id);
                    }
                }
                index += 13;
            }
        }
        Err(conn_err) => {
            eprintln!("Error connect db: {}", conn_err)
        }
    }
    Ok(ResponseStatus {
        success: true,
        success_description: None,
        error_message: None,
    })
}

fn check_user(conn: &Connection, user: &str) -> Result<Option<i64>> {
    let mut stmt =
        conn.prepare("SELECT user_id FROM users WHERE user_name = ?1 OR user_twitch = ?1")?;

    let user_id: Option<i64> = stmt.query_row(params![user], |row| row.get(0)).ok();

    Ok(user_id)
}
