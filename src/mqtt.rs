use anyhow::Result;
use embedded_svc::mqtt::client::{EventPayload, QoS};
use esp_idf_svc::mqtt::client::{EspMqttClient, MqttClientConfiguration};
use std::sync::mpsc;

use crate::{Command, Config};

const TOPIC_STATE: &str = "crosswalk/state";
const TOPIC_COUNT: &str = "crosswalk/count";

pub fn run(app_config: Config, mut cmd_tx: mpsc::Sender<Command>) -> Result<()> {
    let mqtt_config = MqttClientConfiguration::default();
    let broker_url = if !app_config.mqtt_user.is_empty() {
        format!(
            "mqtt://{}:{}@{}",
            app_config.mqtt_user, app_config.mqtt_pass, app_config.mqtt_host
        )
    } else {
        format!("mqtt://{}", app_config.mqtt_host)
    };

    let (connected_tx, connected_rx) = mpsc::channel::<()>();

    let mut client =
        EspMqttClient::new_cb(
            &broker_url,
            &mqtt_config,
            move |message_event| match message_event.payload() {
                EventPayload::Received {
                    topic: Some(topic),
                    data,
                    ..
                } => process_message(topic, data, &mut cmd_tx),
                EventPayload::Connected(_) => {
                    log::info!("New broker");
                    let _ = connected_tx.send(());
                }
                EventPayload::Error(e) => log::error!("{e:?}"),
                _ => log::info!("Unknown: {:?}", message_event.payload()),
            },
        )?;

    while connected_rx.recv().is_ok() {
        let topics = [TOPIC_STATE, TOPIC_COUNT];

        for topic in &topics {
            client.subscribe(topic, QoS::AtLeastOnce)?;
            log::info!("Subscribed to {topic}");
        }
    }

    Ok(())
}

fn process_message(topic: &str, payload: &[u8], cmd_tx: &mut mpsc::Sender<Command>) {
    let payload = match std::str::from_utf8(payload) {
        Ok(s) => s.trim(),
        Err(e) => {
            log::error!("Failed to parse payload: {e}");
            return;
        }
    };

    let command = match topic {
        TOPIC_STATE => match payload {
            "OFF" => Some(Command::Off),
            "MAN" => Some(Command::Man),
            "HAND" => Some(Command::Hand),
            "COUNTDOWN" => Some(Command::Countdown),
            p => {
                log::warn!("Unknown state: {p}");
                None
            }
        },
        TOPIC_COUNT => payload.parse().ok().map(Command::Count),
        _ => {
            log::warn!("Unknown topic: {topic}");
            None
        }
    };

    if let Some(cmd) = command {
        if let Err(e) = cmd_tx.send(cmd) {
            log::error!("Failed to send command: {e}");
        }
    }
}
