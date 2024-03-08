use rusqlite::{Connection, Result};

pub fn setup_database() -> Result<Connection> {
    let conn = Connection::open("/app/data/my_database.db")?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY,
        user_name TEXT NOT NULL,
        user_id INTEGER NOT NULL UNIQUE,
        points INTEGER,
        created_at TIMESTAMP,
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP);
      ",
    )?;
    for i in 1..=10 {
        let user_name = format!("User{}", i);
        let user_id = i.to_string();
        let points: i64 = 1000 - i * 50; // Just an example for points calculation

        conn.execute(
            "INSERT INTO users (user_name, user_id, points,created_at) VALUES (?, ?, ?, CURRENT_TIMESTAMP)",
            &[&user_name, &user_id, &points.to_string()],
        )?;
    }

    conn.is_autocommit();
    println!("DB config check");
    Ok(conn)
}

pub fn connect_database() -> Result<Connection> {
    match Connection::open("/app/data/my_database.db") {
        Ok(conn) => {
            println!("Connexion a la base de datos establecida con exito.");
            Ok(conn)
        }
        Err(err) => {
            eprintln!("Error al abrir la conextion a la base de datos: {:?}", err);
            Err(err)
        }
    }
}
