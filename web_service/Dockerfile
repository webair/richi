FROM rust:1.89.0-slim-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && \
    echo "fn main() {println!(\"dummy\");}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim

ENV WEB_SERVICE_HOST="<webservice host>"
ENV MQTT_BROKER_HOST="<mqtt broker host>"
ENV MQTT_BROKER_USERNAME="<mqtt broker username>"
ENV MQTT_BROKER_PASSWORD="<mqtt broker password>"

WORKDIR /app

COPY --from=builder /app/target/release/richi_remote_door_web_service .

EXPOSE 3000

CMD ["./richi_remote_door_web_service"]
