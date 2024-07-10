FROM rust:latest AS builder
LABEL authors="sony-level"
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /app/target/release/auth_service /usr/local/bin/
CMD ["auth_service"]
