use rusqlite::{Connection, Result};

pub fn setup_database() -> Result<Connection> {
    let conn = Connection::open("my_database.db")?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY,
        user_name TEXT NOT NULL,
        user_id INTEGER NOT NULL UNIQUE,
        points INTEGER,
        created_at TIMESTAMP,
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP);",
    )?;
    conn.is_autocommit();
    println!("DB config check");
    Ok(conn)
}

pub fn connect_database() -> Result<Connection> {
    match Connection::open("my_database.db") {
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
