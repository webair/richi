use rumqttc::Event::Incoming;
use rumqttc::{AsyncClient, ClientError, ConnectionError, MqttOptions, Packet, QoS};
use std::time::Duration;

const LOCK_ACTION_TOPIC: &str = "lockAction";
const LOCK_ACTION_UNLOCK: &str = "1";

pub enum PublishError {
    ConnectionError(String),
    ClientError(String),
}

impl From<ClientError> for PublishError {
    fn from(client_error: ClientError) -> PublishError {
        PublishError::ClientError(client_error.to_string())
    }
}

impl From<ConnectionError> for PublishError {
    fn from(connection_error: ConnectionError) -> PublishError {
        PublishError::ConnectionError(connection_error.to_string())
    }
}

pub async fn publish_open_lock_message() -> Result<(), PublishError> {
    let mut mqtt_options = MqttOptions::new("", &super::CONFIG.mqtt_broker_host, 1883);
    mqtt_options.set_keep_alive(Duration::from_secs(5));
    mqtt_options.set_credentials(
        &super::CONFIG.mqtt_broker_username,
        &super::CONFIG.mqtt_broker_password,
    );
    let (client, mut event_loop) = AsyncClient::new(mqtt_options, 10);
    client
        .publish(
            format!("nuki/{}/{}", &super::CONFIG.nuki_lock_id, LOCK_ACTION_TOPIC),
            QoS::ExactlyOnce,
            false,
            LOCK_ACTION_UNLOCK,
        )
        .await?;

    loop {
        if let Incoming(Packet::PubComp(_)) = event_loop.poll().await? {
            break;
        }
    }
    Ok(())
}
