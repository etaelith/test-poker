use rusqlite::{Connection, Result};

pub fn setup_database() -> Result<Connection> {
    let conn = Connection::open(std::env::var("DB_PATH").expect("missing DISCORD_TOKEN"))?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY,
        user_name TEXT NOT NULL,
        user_id INTEGER NOT NULL UNIQUE,
        points INTEGER,
        bitmex BOOLEAN DEFAULT 0,
        created_at TIMESTAMP,
        wins INTEGER DEFAULT 0,
        bounties INTEGER DEFAULT 0,
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP);

        CREATE TABLE IF NOT EXISTS tournaments (
            tournament_id INTEGER PRIMARY KEY,
            tournament_date DATE NOT NULL UNIQUE,
            bounties INTEGER
        );

        CREATE TABLE IF NOT EXISTS bounties (
            bounty_id INTEGER PRIMARY KEY,
            bounty_winner BOOLEAN DEFAULT 0,
            bounty_date TIMESTAMP,
            user_id INTEGER,
            tournament_id INTEGER,
            coins INTEGER,
            FOREIGN KEY(user_id) REFERENCES users(user_id),
            FOREIGN KEY(tournament_id) REFERENCES tournaments(tournament_date)
        );

        CREATE TABLE IF NOT EXISTS rewardp (
            id INTEGER PRIMARY KEY,
            tournament_id INTEGER,
            user_id INTEGER,
            points INTEGER,
            FOREIGN KEY(tournament_id) REFERENCES tournaments(tournament_date),
            FOREIGN KEY(user_id) REFERENCES users(user_id)
          );

      ",
    )?;
    for i in 1..=10 {
        let user_name = format!("User{}", i);
        let user_id = i.to_string();
        let points: i64 = 1000 - i * 50;

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
    match Connection::open(std::env::var("DB_PATH").expect("missing DB_PATH")) {
        Ok(conn) => Ok(conn),
        Err(err) => {
            eprintln!("Error al abrir la conextion a la base de datos: {:?}", err);
            Err(err)
        }
    }
}
