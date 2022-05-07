FROM rust:1.60.0-buster as builder
RUN USER=root cargo new --bin app
WORKDIR /app

COPY ./Cargo.lock ./
COPY ./Cargo.toml ./
RUN rustup component add rustfmt
RUN cargo build --release
RUN cargo build --release
RUN rm -rf src target

COPY ./src ./src

RUN cargo install --locked --path .

FROM debian:buster-slim
RUN apt update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/recruitment_api /usr/local/bin/recruitment_api
CMD [ "recruitment_api" ]