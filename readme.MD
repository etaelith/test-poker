# Discord Tournament Points Bot

This project is a Dockerized Discord bot written in Rust, designed to keep track of points won in tournaments.

To learn more about the technologies used in this project, you can check out the following documentation:

## Discord Slash Commands

#### Create Tournament

```http
  discord_slash: /create_tournament ${fecha}
```

| Parameter | Type         | Description                      |
| :-------- | :----------- | :------------------------------- |
| `fecha`   | `DD/MM/YYYY` | **Required**. Date of tournament |

#### Sumarized Points

```http
   discord_slash: /sum_points ${points} ${user} ${fecha}
```

| Parameter | Type         | Description                      |
| :-------- | :----------- | :------------------------------- |
| `points`  | `INTEGER`    | **Required**. Number of BB       |
| `user`    | `string`     | **Required**. ID/@Mention user   |
| `fecha`   | `DD/MM/YYYY` | **Required**. Date of tournament |

#### Substract points

```http
   discord_slash: /sum_points ${points} ${user} ${fecha}
```

| Parameter | Type         | Description                      |
| :-------- | :----------- | :------------------------------- |
| `points`  | `INTEGER`    | **Required**. Number of BB       |
| `user`    | `string`     | **Required**. ID/@Mention user   |
| `fecha`   | `DD/MM/YYYY` | **Required**. Date of tournament |

#### Give bounty

```http
   discord_slash: /sum_points ${points} ${user} ${fecha} ${win}
```

| Parameter | Type         | Description                                     |
| :-------- | :----------- | :---------------------------------------------- |
| `points`  | `INTEGER`    | **Required**. Number of BB                      |
| `user`    | `string`     | **Required**. ID/@Mention user                  |
| `fecha`   | `DD/MM/YYYY` | **Required**. Date of tournament                |
| `win`     | `Bool`       | **Required**. Win tournament TRUE, Bounty FALSE |

#### TOP 10 tournament

```http
   discord_slash: /poker_top
```

#### GET user

```http
   discord_slash: /poker_search ${user}
```

| Parameter | Type     | Description                    |
| :-------- | :------- | :----------------------------- |
| `user`    | `string` | **Required**. ID/@Mention user |

## SQL

```sql
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY,
    user_name TEXT NOT NULL,
    user_id INTEGER NOT NULL UNIQUE,
    points INTEGER,
    bitmex BOOLEAN DEFAULT 0,
    created_at TIMESTAMP,
    wins INTEGER DEFAULT 0,
    bounties INTEGER DEFAULT 0,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

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

CREATE UNIQUE INDEX idx_bounty_winner ON bounties (tournament_id) WHERE bounty_winner = 1;
CREATE UNIQUE INDEX idx_reward ON reward (tournament_id, user_id);
```

### Tech Stack

- [Rust](https://www.rust-lang.org/)
- [Docker](https://www.docker.com/)
- [SQLite](https://sqlite.org/)

### CRATES

- [tokio](https://docs.rs/tokio/latest/tokio/)
- [serenity](https://docs.rs/serenity/latest/serenity/)
- [poise](https://docs.rs/poise/latest/poise/)
- [serde](https://docs.rs/serde/latest/serde/)
- [serde_json](https://docs.rs/serde_json/latest/serde_json/index.html)
- [rusqlite](https://docs.rs/rusqlite/latest/rusqlite/)

```

```
