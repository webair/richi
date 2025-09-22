#!/usr/bin/env sh

set -o errexit
set -o nounset

PASSWORD_FILE="/mosquitto/secrets/password_file"
CONFIG_FILE='/mosquitto/config/mosquitto.conf'

mkdir -p "$(dirname "${PASSWORD_FILE}")"
echo "${MQTT_BROKER_USERNAME}:${MQTT_BROKER_PASSWORD}" > "${PASSWORD_FILE}"
chown mosquitto:mosquitto "${PASSWORD_FILE}"
chmod 0700 "${PASSWORD_FILE}"
mosquitto_passwd -U "${PASSWORD_FILE}"
mosquitto -c "${CONFIG_FILE}"
exec "$@"
