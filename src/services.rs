// use channel_bridge::Receiver;
use channel_bridge::{asynch::pubsub, asynch::*};
use embedded_svc::utils::asyncify::Asyncify;
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration, Wifi as WifiTrait};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::modem::WifiModemPeripheral;
use esp_idf_svc::hal::peripheral::Peripheral;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::sys::EspError;
use esp_idf_svc::wifi::{EspWifi, WifiEvent};

use crate::app_config::ControlFreakConfig;

pub fn wifi<'d>(
    modem: impl Peripheral<P = impl WifiModemPeripheral + 'd> + 'd,
    mut sysloop: EspSystemEventLoop,
    partition: Option<EspDefaultNvsPartition>,
    config: &ControlFreakConfig,
) -> Result<(impl WifiTrait + 'd, impl Receiver<Data = WifiEvent>), EspError> {
    let mut wifi = EspWifi::new(modem, sysloop.clone(), partition)?;

    log::info!("Wifi name {}", config.wifi_ssid);

    if config.wifi_pass.is_empty() {
        wifi.set_configuration(&Configuration::Client(ClientConfiguration {
            ssid: config.wifi_ssid.as_str().into(),
            auth_method: AuthMethod::None,
            ..Default::default()
        }))?;
    } else {
        wifi.set_configuration(&Configuration::Client(ClientConfiguration {
            ssid: config.wifi_ssid.as_str().into(),
            password: config.wifi_pass.as_str().into(),
            ..Default::default()
        }))?;
    }

    wifi.start()?;

    wifi.connect()?;

    Ok((
        wifi,
        pubsub::SvcReceiver::new(sysloop.as_async().subscribe()?),
    ))
}
