# Utiliser une image officielle Rust pour construire le binaire
FROM rust:latest as builder

WORKDIR /app

COPY . .

RUN apt-get update && apt-get install -y libpq-dev
RUN cargo install diesel_cli --no-default-features --features postgres

# Utiliser une image officielle Debian pour exécuter l'application
FROM debian:buster-slim

COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
COPY ./entrypoint.sh /app/entrypoint.sh

RUN chmod +x /app/entrypoint.sh

WORKDIR /app

ENTRYPOINT ["sh", "/app/entrypoint.sh"]
