#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail

SSH_USERNAME=richi
SSH_HOST=docker.richi.be
PROJECT_NAME=richi_remote_lock
DEPLOY_DIR=/home/${SSH_USERNAME}/${PROJECT_NAME}

main() {
    ssh "${SSH_USERNAME}@${SSH_HOST}" "\
    mkdir -p ${DEPLOY_DIR}; \
    mkdir -p ${DEPLOY_DIR}/data/caddy_data ${DEPLOY_DIR}/data/logs"

    scp docker-compose.yml "${SSH_USERNAME}@${SSH_HOST}:${DEPLOY_DIR}"

    ssh "${SSH_USERNAME}@${SSH_HOST}" "\
    cd ${DEPLOY_DIR}; docker compose pull; \
    docker compose up --force-recreate --build -d; \
    docker image prune -f"
}

main
