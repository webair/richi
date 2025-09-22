# Richi Remote Lock

Hokuspokus simsalabich, Schloss vom richi, öffne dich!

## Setup Development Environment

Copy the example environment file
```sh
cp .env.example .env
```
Open the newly created .env file in a text editor and fill in the required values.

## Run Dev environment
```sh
docker compose -f docker-compose.yml -f docker-compose.dev.yml up -d --build --remove-orphans
```

## Build and publish docker images to hub.docker.com
```sh
./ci/publish.sh
```

## Setup Production Environment

- Make sure to copy the docker-compose.yml file to the production server
- Copy .env.example file to .env and fill in the required values
- Create the used data folders next to the docker-compose.yml file
```sh
mkdir -p data/caddy_data data/logs
```


## Deploy to Production Environment
```sh
./ci/deploy.sh
```

### TCP Proxy on Raspberry Pi
```sh
socat TCP4-LISTEN:1883 TCP-CONNECT:<url>:1883
```
