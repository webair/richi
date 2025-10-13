use rumqttc::Event::Incoming;
use rumqttc::{AsyncClient, MqttOptions, Packet, QoS};
use std::time::Duration;

use crate::config;
use crate::error::Result;

const LOCK_ACTION_TOPIC: &str = "lockAction";
const LOCK_ACTION_UNLOCK: &str = "1";

pub async fn publish_open_lock_message() -> Result<()> {
    let mut mqtt_options = MqttOptions::new("", &config::config().mqtt_broker_host, 1883);
    mqtt_options.set_keep_alive(Duration::from_secs(5));
    mqtt_options.set_credentials(
        &config::config().mqtt_broker_username,
        &config::config().mqtt_broker_password,
    );
    let (client, mut event_loop) = AsyncClient::new(mqtt_options, 10);
    client
        .publish(
            format!(
                "nuki/{}/{}",
                config::config().nuki_lock_id,
                LOCK_ACTION_TOPIC
            ),
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
