FROM rust:1-bookworm AS build

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --locked

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build /app/target/release/helius /usr/local/bin/helius

ENV HELIUS_DB_PATH=/data/tracker.db

WORKDIR /data

RUN mkdir -p /data

VOLUME ["/data"]

ENTRYPOINT ["helius"]
