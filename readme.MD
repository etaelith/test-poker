# Discord Tournament Points Bot

![Logo](logo.jpg)

This project is a Dockerized Discord bot written in Rust, designed to keep track of points won in tournaments.

To learn more about the technologies used in this project, you can check out the following documentation:

```
\SRC
│   auth.rs
│   data_structs.rs
│   main.rs
│
├───db
│   │   config.rs
│   │   mod.rs
│   │   utils.rs
│   │
│   └───commands
│           mod.rs
│           table_bounties.rs
│           table_reward.rs
│           table_tournaments.rs
│           table_users.rs
│
└───discord
    │   handler.rs
    │   mod.rs
    │   utils.rs
    │
    └───commands
            bosses.rs
            bounties.rs
            mod.rs
            points.rs
            poker.rs
            tournaments.rs
```

## Deployment

### Docker (need update)

To deploy this project run

```bash
  docker build -t <project_name> .
```

```bash
  docker run -e DISCORD_TOKEN=<value> <containerImage>
```

### Cargo

```bash
   cargo build
```

```bash
   DISCORD_TOKEN=<value> CLIENT_ID=<value> CLIENT_SECRET=<value> PORT=<value> cargo run
```

## Discord msg handler

#### Write top 10 and pin msg

```http
   !top
```

## Discord Slash Commands

#### GET TOURNAMENTS

```http
   /get_tournaments
```

#### TOP 10 tournament

```http
   /top10
```

#### Tournament ID TOP 10

```http
   /top10_tournament ${fecha}
```

| Parameter | Type         | Description                      |
| :-------- | :----------- | :------------------------------- |
| `fecha`   | `DD/MM/YYYY` | **Required**. Date of tournament |

#### GET user (rank & points)

```http
   /search_user ${user}
```

| Parameter | Type     | Description                    |
| :-------- | :------- | :----------------------------- |
| `user`    | `string` | **Required**. ID/@Mention user |

#### GET user_info (general data)

```http
   /info_user ${user}
```

| Parameter | Type     | Description                    |
| :-------- | :------- | :----------------------------- |
| `user`    | `string` | **Required**. ID/@Mention user |

#### Create Tournament

```http
  /create_tournament ${fecha} ROLE NEEDED
```

| Parameter | Type         | Description                      |
| :-------- | :----------- | :------------------------------- |
| `fecha`   | `DD/MM/YYYY` | **Required**. Date of tournament |

#### Create guild_boss

```http
  /agree_boss ${user} ROLE NEEDED ONLY OWNER
```

| Parameter | Type     | Description                    |
| :-------- | :------- | :----------------------------- |
| `user`    | `string` | **Required**. ID/@Mention user |

#### Sumarized Points

```http
   /sum_points ${points} ${user} ${fecha} ROLE NEEDED
```

| Parameter | Type         | Description                      |
| :-------- | :----------- | :------------------------------- |
| `points`  | `INTEGER`    | **Required**. Number of BB       |
| `user`    | `string`     | **Required**. ID/@Mention user   |
| `fecha`   | `DD/MM/YYYY` | **Required**. Date of tournament |

#### Substract points

```http
   /substract_points ${points} ${user} ${fecha} ROLE NEEDED
```

| Parameter | Type         | Description                      |
| :-------- | :----------- | :------------------------------- |
| `points`  | `INTEGER`    | **Required**. Number of BB       |
| `user`    | `string`     | **Required**. ID/@Mention user   |
| `fecha`   | `DD/MM/YYYY` | **Required**. Date of tournament |

#### Delete points

```http
   /delete_points ${user} ${fecha} ROLE NEEDED
```

| Parameter | Type         | Description                      |
| :-------- | :----------- | :------------------------------- |
| `points`  | `INTEGER`    | **Required**. Number of BB       |
| `user`    | `string`     | **Required**. ID/@Mention user   |
| `fecha`   | `DD/MM/YYYY` | **Required**. Date of tournament |

#### Give bounty

```http
   /give_bounty ${points} ${user} ${fecha} ${win} ROLE NEEDED
```

| Parameter | Type         | Description                                     |
| :-------- | :----------- | :---------------------------------------------- |
| `points`  | `INTEGER`    | **Required**. Number of BB                      |
| `user`    | `string`     | **Required**. ID/@Mention user                  |
| `fecha`   | `DD/MM/YYYY` | **Required**. Date of tournament                |
| `win`     | `Bool`       | **Required**. Win tournament TRUE, Bounty FALSE |

#### Give verified

```http
   /verifed ${user} ${verify} ROLE NEEDED
```

| Parameter | Type     | Description                                       |
| :-------- | :------- | :------------------------------------------------ |
| `user`    | `string` | **Required**. ID/@Mention user                    |
| `verify`  | `Bool`   | **Required**. Checked on bitmex TRUE or not FALSE |

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
- [chrono](https://docs.rs/chrono/latest/chrono/)

#### Weird shit

- [ErrorPath](https://stackoverflow.com/questions/7250130/how-to-stop-mingw-and-msys-from-mangling-path-names-given-at-the-command-line/34386471#34386471)
