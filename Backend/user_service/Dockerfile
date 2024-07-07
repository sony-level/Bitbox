FROM ubuntu:latest
LABEL authors="sony-level"

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY user_service ./user_service

RUN cargo build --release

CMD ["./target/release/user_service"]


