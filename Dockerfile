FROM rust:latest as builder

WORKDIR /usr/src

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src

RUN cargo build --release
RUN ls -la /usr/src/target/release/
# Production stage
FROM debian:bullseye-slim
COPY --from=builder /usr/src/target/release/idiotita-poker .
RUN ls -la .
# Define el volumen para la base de datos
VOLUME /usr/database

CMD [ "./idiotita-poker" ]

# docker run -e DISCORD_TOKEN=<ID> <containerImage>