
FROM rust:latest


WORKDIR /usr/src/app


COPY Cargo.toml diesel.toml ./


COPY src ./src

COPY migrations ./migrations

# Installer diesel_cli pour gérer les migrations
RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo build --release


CMD ["./target/release/common"]
