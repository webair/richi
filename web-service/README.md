# Richi Remote Lock Web Service

## Start dependencies
```sh
docker compose -f docker-compose.yml up -d --build --remove-orphans
```

## Start web service
```sh
URL="http://localhost:3000" \
MQTT_BROKER_USERNAME="username" \
MQTT_BROKER_PASSWORD="password" \
MQTT_BROKER_HOST="localhost" \
NUKI_LOCK_IDS="474D4D1F" \
JWKS_URL="https://kaiupgcdozjalfocsddp.supabase.co/auth/v1/.well-known/jwks.json" \
cargo run
```
