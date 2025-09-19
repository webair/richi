FROM --platform=${BUILDPLATFORM} rust:1.89.0-slim-bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    pkgconf \
    gcc-aarch64-linux-gnu \
    gcc-x86-64-linux-gnu \
    libc6-dev-arm64-cross \
    libc6-dev-amd64-cross \
    && rm -rf /var/lib/apt/lists/

RUN rustup target add x86_64-unknown-linux-gnu && \
    rustup target add aarch64-unknown-linux-gnu

ARG TARGETPLATFORM

COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    export RUST_TARGET=$(case ${TARGETPLATFORM} in \
        "linux/amd64") echo "x86_64-unknown-linux-gnu" ;; \
        "linux/arm64") echo "aarch64-unknown-linux-gnu" ;; \
        *) echo "x86_64-unknown-linux-gnu" ;; \
    esac) && \
    cargo build --release --locked && \
    rm -rf src

COPY src ./src
RUN export RUST_TARGET=$(case ${TARGETPLATFORM} in \
        "linux/amd64") echo "x86_64-unknown-linux-gnu" ;; \
        "linux/arm64") echo "aarch64-unknown-linux-gnu" ;; \
        *) echo "x86_64-unknown-linux-gnu" ;; \
    esac) && \
    cargo build --release --locked && \
    echo "Build Richi Remote Lock Web Service for target: ${RUST_TARGET}"


FROM --platform=${TARGETARCH:-$BUILDPLATFORM} debian:bookworm-slim

ENV WEB_SERVICE_HOST="<webservice host>"
ENV MQTT_BROKER_HOST="<mqtt broker host>"
ENV MQTT_BROKER_USERNAME="<mqtt broker username>"
ENV MQTT_BROKER_PASSWORD="<mqtt broker password>"

WORKDIR /app

COPY --from=builder /app/target/release/richi_remote_lock_web_service .

EXPOSE 3000

CMD ["./richi_remote_lock_web_service"]
