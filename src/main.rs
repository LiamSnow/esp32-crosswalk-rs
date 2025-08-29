use anyhow::Result;
use esp_idf_svc::{eventloop::EspSystemEventLoop, hal::prelude::*};
use std::{sync::mpsc, thread};

use crate::controller::CrosswalkController;

mod controller;
mod mqtt;
mod wifi;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    mqtt_host: &'static str,
    #[default("")]
    mqtt_user: &'static str,
    #[default("")]
    mqtt_pass: &'static str,
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

#[derive(Debug, Clone)]
enum Command {
    Off,
    Hand,
    Man,
    Countdown,
    Count(u8),
}

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    // set by toml-cfg
    let app_config = CONFIG;

    let _wifi = wifi::wifi(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        peripherals.modem,
        sysloop,
    )?;

    let (cmd_tx, cmd_rx) = mpsc::channel();

    let pins = peripherals.pins;
    thread::spawn(move || {
        let mut controller = CrosswalkController::new(pins, cmd_rx).unwrap();
        if let Err(e) = controller.run() {
            log::error!("Controller thread error: {e}");
        }
    });

    mqtt::run(app_config, cmd_tx).unwrap();

    Ok(())
}
