use rusqlite::{Connection, Result};

pub fn setup_database() -> Result<Connection> {
    let conn = Connection::open(std::env::var("DB_PATH").expect("missing DB_PATH"))?;
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

        CREATE TABLE IF NOT EXISTS reward (
            id INTEGER PRIMARY KEY,
            tournament_id INTEGER,
            user_id INTEGER,
            points INTEGER,
            FOREIGN KEY(tournament_id) REFERENCES tournaments(tournament_date),
            FOREIGN KEY(user_id) REFERENCES users(user_id)
          );
        
        CREATE TABLE IF NOT EXISTS bosses (
            boss_id INTEGER PRIMARY KEY,
            user_id INTEGER,
            user_name TEXT NOT NULL,
            created_at TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY(user_id) REFERENCES users(user_id)
        );

        CREATE UNIQUE INDEX idx_bounty_winner ON bounties (tournament_id) WHERE bounty_winner = 1;
        CREATE UNIQUE INDEX idx_reward ON reward (tournament_id, user_id);
      ",
    )?;

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
