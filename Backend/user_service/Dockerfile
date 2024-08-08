FROM rust:1.69 as builder

LABEL authors="sony-level" \
      description="Dockerfile pour l'application user_service"

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /app

COPY --from=builder /app/target/release/user_service .

EXPOSE 8000


CMD ["./user_service"]

