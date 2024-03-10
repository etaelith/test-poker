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
