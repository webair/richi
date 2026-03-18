FROM --platform=${BUILDPLATFORM} rust:1.89.0-slim-bookworm AS builder

WORKDIR /app


ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc

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

ARG TARGETARCH
ARG RUST_APP_NAME=richi-lock-web-service

COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    export RUST_TARGET=$(case ${TARGETARCH} in \
        "amd64") echo "x86_64-unknown-linux-gnu" ;; \
        "arm64") echo "aarch64-unknown-linux-gnu" ;; \
        *) echo "x86_64-unknown-linux-gnu" ;; \
    esac) && \
    cargo build --release --locked --target ${RUST_TARGET} && \
    rm -r src && \
    rm "target/${RUST_TARGET}/release/${RUST_APP_NAME}" && \
    rm -r "target/${RUST_TARGET}/release/.fingerprint/${RUST_APP_NAME}-"*

COPY src ./src
RUN export RUST_TARGET=$(case ${TARGETARCH} in \
        "amd64") echo "x86_64-unknown-linux-gnu" ;; \
        "arm64") echo "aarch64-unknown-linux-gnu" ;; \
        *) echo "x86_64-unknown-linux-gnu" ;; \
    esac) && \
    cargo build --release --locked --target ${RUST_TARGET} && \
    ln -s "/app/target/${RUST_TARGET}/release/richi_remote_lock_web_service" "richi_remote_lock_web_service-${TARGETARCH}" && \
    echo "Build Richi Remote Lock Web Service for target: ${RUST_TARGET}"


FROM --platform=${TARGETARCH:-$BUILDPLATFORM} debian:bookworm-slim

ENV WEB_SERVICE_HOST="<webservice host>"
ENV MQTT_BROKER_HOST="<mqtt broker host>"
ENV MQTT_BROKER_USERNAME="<mqtt broker username>"
ENV MQTT_BROKER_PASSWORD="<mqtt broker password>"
ARG TARGETARCH

WORKDIR /app

COPY --from=builder "/app/richi-lock-web-service-${TARGETARCH}" richi-lock-web-service

EXPOSE 3000

CMD ["./richi_remote_lock_web_service"]
