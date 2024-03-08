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

CMD [ "./idiotita-poker" ]

# docker run -e DISCORD_TOKEN=<ID> <containerImage>