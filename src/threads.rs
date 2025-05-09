use std::thread;
use std::time::Duration;
use crate::api::UPDATE_WEATHER_INFO;
use crate::bluetooth::UPDATE_BLUETOOTH_DATA;
use crate::wifi_api::refresh_wifi_connectivity;

const WIFI_STAT_SCHEDULE_UPDATE: Duration = Duration::from_secs(15);
const BT_DATA_SCHEDULE_UPDATE: Duration = Duration::from_millis(350);
const WEATHER_SCHEDULE_UPDATE: Duration = Duration::from_secs(10_000);

fn start_wifi_con_update_thread() {
    thread::spawn(|| {
        loop {
            refresh_wifi_connectivity();
            thread::sleep(WIFI_STAT_SCHEDULE_UPDATE)
        }
    });
}

fn start_bt_data_update_thread() {
    tokio::spawn((async || {loop {
        UPDATE_BLUETOOTH_DATA().await;
        tokio::time::sleep(BT_DATA_SCHEDULE_UPDATE).await;
    }})());
}

fn start_weather_update_thread() {
    thread::spawn(|| {
        loop {
            UPDATE_WEATHER_INFO();
            thread::sleep(WEATHER_SCHEDULE_UPDATE)
        }
    });
}

pub fn init_threads() {
    start_wifi_con_update_thread();
    start_weather_update_thread();
    start_bt_data_update_thread();
}