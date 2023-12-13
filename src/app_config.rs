use serde::{Deserialize, Serialize};
use std::sync::RwLock;

#[derive(Default)]
pub struct ControlFreakConfig {
    pub ap_name: String,
    pub wifi_ssid: String,
    pub wifi_pass: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct ControlFreakState {
    pub wifi_state: RwLock<WifiState>,
}

type WifiState = String;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum NetworkStateChange {
    WifiDisconnected,
    IpAddressAssigned { ip: embedded_svc::ipv4::Ipv4Addr },
}
