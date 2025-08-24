# Richi Remote Door

Sim Sala bim, Türe vom richi öffne dich!

## Setup Development Environment

Copy the example environment file
```sh
cp .env.example .env
```
Open the newly created .env file in a text editor and fill in the required values.

## Run Locally

Web Service
```sh
cd ./web_service
direnv exec ../. cargo run
```

Docker
```sh
docker compose -f docker-compose.yml -f docker-compose-dev.yml up -d --build --remove-orphans
```

## Deploy to remote server
Build and publish docker images
```sh
./publish.sh
```

## Notes

### Start Docker Containers
```sh
docker compose up -d --build --remove-orphans
```

### Publish Test Message
```sh
 docker exec -it  mosquitto_pub -h localhost -t "test/topic" -m "Hello MQTT!" -u "username" -P "password"
```
