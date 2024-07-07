FROM rust:latest
LABEL authors="sony-level"

WORKDIR /usr/src/app

COPY Cargo.toml diesel.toml  Cargoo.lock./

COPY common ./common

COPY migrations ./migrations

# Installer diesel_cli pour gérer les migrations
RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo build --release

CMD ["./target/release/common"]
