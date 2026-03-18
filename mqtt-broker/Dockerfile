FROM eclipse-mosquitto:2.0.22-openssl
COPY ./mosquitto.conf /mosquitto/config/
COPY ./entrypoint.sh  /usr/bin/
RUN chmod +x /usr/bin/entrypoint.sh
ENTRYPOINT [ "entrypoint.sh" ]
