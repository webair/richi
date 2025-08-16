# Richi Remote Door

Sim Sala bim, Türe vom richi öffne dich!

## Notes

### Start docker containers
```sh
docker compose up -d --force-recreate --build --remove-orphans
```

### Create mosquitto user and password
```sh
docker exec -it richi_remote_door-mqtt-broker-1 mosquitto_passwd -c /mosquitto/config/password_file richi_lock
```

### Publish test Message
```sh
 docker exec -it richi-mosquitto mosquitto_pub -h localhost -t "test/topic" -m "Hello MQTT!" -u "richi_lock" -P "password"
```