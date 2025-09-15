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

## Build and publish docker images to hub.docer.com
```sh
./publish.sh
```

### TCP Proxy on Raspberry Pi
```sh
socat TCP4-LISTEN:1883 TCP-CONNECT:<url>:1883
```
