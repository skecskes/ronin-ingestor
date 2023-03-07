FROM rust:1.67 as builder
RUN apt update && apt install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/ronin
COPY ./migration ./migration
COPY ./src ./src
COPY ./.envrc ./.envrc
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo --locked install --path . --profile release

FROM debian:bullseye-slim

COPY --from=builder /usr/local/cargo/bin/ingestor /usr/local/bin/ingestor

ENV RUST_BACKTRACE=full

LABEL org.opencontainers.image.authors="Stefan Kecskes"
LABEL org.opencontainers.image.vendor="Stefan Kecskes"
LABEL org.opencontainers.image.source="git@github.com:skecskes/ronin-ingestor.git"
LABEL org.opencontainers.image.licenses="UNLICENSED"

ENV PGHOST="localhost"
ENV PGUSER="postgres"
ENV PGPASSWORD="postgres"
ENV PGPORT=5432
ENV PGDATABASE="devdb"
ENV DATABASE_URL="postgres://$POSTGRES_USER@$POSTGRES_HOST:$POSTGRES_PORT/$POSTGRES_DB"
ENV RPC_URL="http://ronin.tribally.xyz"
ENV BLOCKS_TO_INGEST=10000
ENV BLOCKS_CHUNK_SIZE=100

EXPOSE 3000

CMD ["ingestor"]