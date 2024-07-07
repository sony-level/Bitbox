FROM rust:latest
LABEL authors="sony-level"

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release

COPY . .

RUN cargo build --release

EXPOSE 8000

CMD ["./target/release/auth_service"]

