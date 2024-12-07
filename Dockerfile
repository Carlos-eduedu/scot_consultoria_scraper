FROM rust:1.83-bullseye AS builder

WORKDIR /usr/src/scot_consultoria_scraper

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release && rm src/*.rs && rm target/release/deps/scot_consultoria_scraper*

COPY ./src ./src

RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates curl

ENV SENTRY_DSN=https://27a04f51b6464b07ea26001363159018@o4505302233120768.ingest.us.sentry.io/4508410318815232
COPY --from=builder /usr/src/scot_consultoria_scraper/target/release/scot_consultoria_scraper .

CMD ["./scot_consultoria_scraper"]
