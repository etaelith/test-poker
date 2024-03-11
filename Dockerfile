#Build stage
FROM rust:1.75.0-slim-bullseye as builder

WORKDIR /usr/src

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src


RUN cargo build --release

# Production stage
FROM debian:bullseye-slim

WORKDIR /app

COPY --from=builder /usr/src/target/release/idiotita-poker .

# Define el volumen para la base de datos
VOLUME /app/data

ENV DB_PATH=/app/data/my_database.db

ENV ROLE_ADMIN=1214016146559471656

CMD [ "./idiotita-poker" ]

# docker run -e DISCORD_TOKEN=<ID> DB_PATH=<DB_PATH> ROLE_ADMIN=<ROLE_ID> <containerImage>