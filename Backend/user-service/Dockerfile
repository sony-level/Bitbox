FROM rust:1.57 AS builder

WORKDIR /usr/src/user-service
COPY . .

RUN cargo install --path .

FROM debian:buster-slim

RUN apt-get update && apt-get install -y libpq-dev

COPY --from=builder /usr/local/cargo/bin/user-service /usr/local/bin/user-service

CMD ["user-service"]
