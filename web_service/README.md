# Richi Remote Lock Web Service

## Start dependencies
```sh
docker compose -f docker-compose.yml up -d --build --remove-orphans
```

## Start web service
```sh
WEB_SERVICE_HOST="http://localhost:3000" MQTT_BROKER_USERNAME="username" \
MQTT_BROKER_PASSWORD="password" MQTT_BROKER_HOST="localhost" NUKI_LOCK_ID="474D4D1F" \
cargo run
```
