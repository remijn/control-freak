use std::time::Duration;

use crate::app_config::{ControlFreakConfig, NetworkStateChange};
use crate::display_wrapper::{DisplayType, DisplayWrapper};
use crate::services::wifi;
use anyhow::{self, Error};
use channel_bridge::{asynch::pubsub, asynch::*};
use edge_executor::{Executor, Task};
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::primitives::{Circle, Primitive, PrimitiveStyle};
use embedded_graphics::Drawable;
use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::geometry::Dimensions;
use embedded_graphics_core::pixelcolor::RgbColor;
use embedded_graphics_core::primitives::Rectangle;
use embedded_svc::utils::asyncify::Asyncify;
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration, Wifi as WifiTrait};
use esp_idf_hal::sys::esp_ota_mark_app_valid_cancel_rollback;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::delay;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::spi::{Dma, SpiConfig, SpiDeviceDriver, SpiDriverConfig};
use esp_idf_svc::hal::task::thread::ThreadSpawnConfiguration;
use esp_idf_svc::hal::units::MegaHertz;
use esp_idf_svc::netif::IpEvent;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::WifiEvent;
// use esp_idf_svc::wifi::{
//     AccessPointConfiguration, AsyncWifi, AuthMethod, ClientConfiguration, Configuration,
// };
use ssd1351::builder::Builder;

mod app_config;
mod display_wrapper;
mod errors;
mod services;
use crate::errors::*;

const TASK_MID_PRIORITY: u8 = 40;
const TASK_LOW_PRIORITY: u8 = 30;

pub static mut FRAME_BUFFER: &mut [u8] = &mut [0; 128 * 128 * 2];

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let sysloop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    log::info!("Hello, world!");

    let config = ControlFreakConfig {
        ap_name: "ControlFreakAP".into(),
        wifi_ssid: "Tochthok".into(),
        wifi_pass: "Tochthok_36".into(),
    };

    let (wifi, wifi_notif) = wifi(peripherals.modem, sysloop.clone(), Some(nvs), &config).unwrap();

    // ThreadSpawnConfiguration {
    //     name: Some(b"mid-prio-executor\0"),
    //     priority: TASK_MID_PRIORITY,
    //     ..Default::default()
    // }
    // .set()
    // .unwrap();

    // let exec = Executor::new();

    // let executor: Executor = Default::default();
    // let wifi_state_change = executor.spawn(async move {
    //     process_wifi_state_change(wifi_notif).await;
    // });

    // let fut = executor.run(wifi_state_change);

    // let mid_prio_execution = schedule::<8, _>(30000, move || {
    //     let executor = Executor::new();
    //     let mut tasks = vec![];

    //     log::info!("enter mid_prio_execution");
    //     executor.spawn_local_collect(process_wifi_state_change(wifi, wifi_notif), &mut tasks)?;

    //     // executor.spawn_local_collect(wind_speed_demo_publisher_task(), &mut tasks)?;

    //     // executor.spawn_local_collect(ota_task(), &mut tasks)?;
    //     // executor.spawn_local_collect(http_server_task(), &mut tasks)?;

    //     // executor.spawn_local_collect(
    //     //     process_netif_state_change(netif_notifier(sysloop.clone()).unwrap()),
    //     //     &mut tasks,
    //     // )?;
    //     log::info!("leave mid_prio_execution");
    //     Ok((executor, tasks))
    // });

    // ThreadSpawnConfiguration {
    //     name: Some(b"mqtt-executor\0"),
    //     priority: TASK_MID_PRIORITY,
    //     ..Default::default()
    // }
    // .set()?;

    // std::thread::sleep(core::time::Duration::from_millis(8000));
    // let (mqtt_topic_prefix, mqtt_client, mqtt_conn) = services::mqtt()?;

    // let mqtt_execution = schedule::<8, _>(8000, move || {
    //     let executor = EspExecutor::new();
    //     let mut tasks = heapless::Vec::new();
    //     log::info!("enter mqtt_execution");

    //     executor.spawn_local_collect(mqtt::receive_task(mqtt_conn), &mut tasks)?;
    //     log::info!("leave mqtt_execution");
    //     Ok((executor, tasks))
    // });

    // ThreadSpawnConfiguration {
    //     name: Some(b"low-prio-executor\0"),
    //     priority: TASK_LOW_PRIORITY,
    //     ..Default::default()
    // }
    // .set()?;

    // let low_prio_execution = schedule::<8, _>(8000, move || {
    //     let executor = EspExecutor::new();
    //     let mut tasks = heapless::Vec::new();
    //     log::info!("enter low_prio_execution");

    //     executor.spawn_local_collect(
    //         mqtt::send_task::<MQTT_MAX_TOPIC_LEN>(mqtt_topic_prefix, mqtt_client),
    //         &mut tasks,
    //     )?;
    //     log::info!("leave low_prio_execution");
    //     Ok((executor, tasks))
    // });

    // This is required to allow the low prio thread to start
    // std::thread::sleep(core::time::Duration::from_millis(2000));
    log::info!("before mid_prio_execution");
    // mid_prio_execution.join().unwrap();
    // log::info!("before mqtt_execution");
    // mqtt_execution.join().unwrap();
    // log::info!("before low_prio_execution");
    // low_prio_execution.join().unwrap();

    log::info!("tasks running");

    // unreachable!();

    // let mut wifi = EspWifi::new(peripherals.modem, sysloop, Some(nvs))?;

    // let ap_config = Configuration::AccessPoint(AccessPointConfiguration {
    //     ssid: "[ControlFreak]".into(),
    //     auth_method: AuthMethod::None,
    //     ..Default::default()
    // });

    // wifi.set_configuration(&config);

    // // Start Wifi
    // wifi.start()?;
    // // Connect Wifi
    // wifi.connect()?;

    // ThreadSpawnConfiguration {
    //     name: Some("network-thread".as_bytes()),
    //     stack_size: 10000,
    //     priority: 15,
    //     ..Default::default()
    // }
    // .set()
    // .unwrap();

    // let _thread_1 = std::thread::Builder::new()
    //     .spawn(move || {
    //         // Wait for connection to happen
    //         while !wifi.is_connected().unwrap() {
    //             // Get and print connetion configuration
    //             let config = wifi.get_configuration().unwrap();
    //             log::info!("Waiting for station {:?}", config);
    //         }
    //     })
    //     .unwrap();

    // _thread_1.join().unwrap();

    // log::info!("Connected");

    let pins = peripherals.pins;
    let spi = peripherals.spi2;

    let spi_miso = pins.gpio13;

    let spi_mosi = pins.gpio11;
    let spi_clk = pins.gpio12;
    let spi_cs = pins.gpio4;
    let spi_dc = pins.gpio38;
    let spi_reset = pins.gpio14;

    log::info!("Interface");
    let interface = SpiDeviceDriver::new_single(
        spi,
        spi_clk,
        spi_mosi,
        Some(spi_miso),
        Some(spi_cs),
        &SpiDriverConfig::new().dma(Dma::Disabled),
        &SpiConfig::new().baudrate(MegaHertz(40).into()),
    )
    .unwrap();

    log::info!("DisplayBuilder");
    let mut display: DisplayType<'_> = Builder::new()
        .connect_spi(interface, PinDriver::output(spi_dc).unwrap(), unsafe {
            FRAME_BUFFER
        })
        .into();

    let mut reset = PinDriver::output(spi_reset).unwrap();

    let mut delay = delay::FreeRtos;

    log::info!("Reset");
    display.reset(&mut reset, &mut delay).unwrap();

    log::info!("Init");
    display.init().unwrap();

    let mut display = DisplayWrapper { display };

    Rectangle::new(Point::zero(), display.bounding_box().size)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
        .draw(&mut display)
        .unwrap();

    log::info!("Circle");
    Circle::new(Point::new(16, 16), 40)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLUE))
        .draw(&mut display)
        .unwrap();

    log::info!("Flush to display");
    display.display.flush();

    loop {
        std::thread::sleep(Duration::from_millis(2000));
    }

    // Ok(())
}

// pub fn schedule<'a, const C: usize, M>(
//     stack_size: usize,
//     spawner: impl FnOnce() -> Result<(Executor<'a, C>, Vec<Task<()>>), InitError> + Send + 'static,
// ) -> std::thread::JoinHandle<()>
// // where
// //     M: Monitor + Wait + Default,
// {
//     std::thread::Builder::new()
//         .stack_size(stack_size)
//         .spawn(move || {
//             let (executor, tasks) = spawner().unwrap();

//             executor.run_tasks(|| true, tasks);
//         })
//         .unwrap()
// }

#[inline(always)]
pub fn netif_notifier(
    mut sysloop: EspSystemEventLoop,
) -> Result<impl Receiver<Data = IpEvent>, InitError> {
    Ok(pubsub::SvcReceiver::new(sysloop.as_async().subscribe()?))
}

pub async fn process_wifi_state_change(
    // mut wifi: impl WifiTrait,
    mut state_changed_source: impl Receiver<Data = WifiEvent>,
) {
    loop {
        let event = state_changed_source.recv().await.unwrap();

        match event {
            WifiEvent::StaConnected => {
                log::info!("WifiEvent: STAConnected");
            }
            WifiEvent::StaDisconnected => {
                log::info!("WifiEvent: STADisconnected");
                // let mut publisher = NETWORK_EVENT_CHANNEL.publisher().unwrap();
                // let _ = publisher.send(NetworkStateChange::WifiDisconnected).await;
                // let _ = wifi.connect();
            }
            _ => {
                log::info!("WifiEvent: other .....");
            }
        }
    }
}

pub async fn process_netif_state_change(mut state_changed_source: impl Receiver<Data = IpEvent>) {
    loop {
        let event = state_changed_source.recv().await.unwrap();

        match event {
            IpEvent::DhcpIpAssigned(assignment) => {
                log::info!("IpEvent: DhcpIpAssigned: {:?}", assignment.ip_settings.ip);

                // if an IP address has been succesfully assiggned we consider
                // the application working, no rollback required.
                unsafe { esp_ota_mark_app_valid_cancel_rollback() };

                // let mut publisher = NETWORK_EVENT_CHANNEL.publisher().unwrap();
                // let _ = publisher
                //     .send(NetworkStateChange::IpAddressAssigned {
                //         ip: assignment.ip_settings.ip,
                //     })
                //     .await;
            }
            _ => {
                log::info!("IpEvent: other .....");
            }
        }
    }
}
