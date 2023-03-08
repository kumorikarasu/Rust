FROM rust:1-buster as builder

WORKDIR /usr/src/app

COPY Cargo.toml .
COPY Cargo.lock .
COPY lib ./lib
COPY server ./server
COPY runner ./runner

RUN cargo build -p server
RUN cargo build -p runner

# FROM alpine:3.12
FROM debian:buster-slim

COPY --from=builder /usr/src/app/target/debug/server /usr/local/bin/server
COPY --from=builder /usr/src/app/target/debug/runner /usr/local/bin/runner
CMD ["/usr/local/bin/server"]
