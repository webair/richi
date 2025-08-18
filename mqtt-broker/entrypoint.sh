#!/usr/bin/env sh

set -o errexit
set -o nounset
set -o pipefail

password_file="/mosquitto/secrets/password_file"
mkdir -p "$(dirname "${password_file}")"
echo "${USERNAME}:${PASSWORD}" > "${password_file}"
chown mosquitto:mosquitto "${password_file}"
chmod 0700 "${password_file}"
mosquitto_passwd -U "${password_file}"
mosquitto -c /mosquitto/config/mosquitto.conf
exec "$@"
