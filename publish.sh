#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail

DOCKER_USERNAME=webair

main() {
    docker login
    build_and_push_image "./mqtt_broker" "richi-lock-mqtt-broker"
    build_and_push_image "./reverse_proxy" "richi-lock-reverse-proxy"
    build_and_push_image "./web_service" "richi-lock-web-service"
    build_and_push_image "./web_client" "richi-lock-web-client"
}

build_and_push_image() {
    local build_dir="$1"
    local image_name="$2"
    echo "Building and pushing image for: ${image_name}"
    docker buildx build --platform "linux/amd64,linux/arm64" --push -t "${DOCKER_USERNAME}/${image_name}:latest" "./${build_dir}"
    echo "Successfully pushed ${image_name}:latest"
}

main
