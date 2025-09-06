# Richi Remote Door

Sim Sala bim, Türe vom richi öffne dich!

## Setup Development Environment

Copy the example environment file
```sh
cp .env.example .env
```
Open the newly created .env file in a text editor and fill in the required values.

## Run Locally

### Web Service
```sh
# Start dependencies
docker compose -f docker-compose.dev.yml up -d --build --remove-orphans

# Run webservice
cd ./web_service
WEB_SERVICE_HOST="http://localhost:3000" MQTT_BROKER_USERNAME="username" \
MQTT_BROKER_PASSWORD="password" MQTT_BROKER_HOST="localhost" NUKI_LOCK_ID="474D4D1F" \
cargo run
```

### Web Client
Make sure Web Service is running
```sh
cd ./web_client
npm run dev
```

### Full environment
```sh
docker compose -f docker-compose.yml -f docker-compose.local.yml up -d --build --remove-orphans
```

## Deploy to remote server
Build and publish docker images
```sh
./publish.sh
```

### TCP Proxy on Raspberry Pi
```sh
socat TCP4-LISTEN:1883 TCP-CONNECT:<url>:1883
```
