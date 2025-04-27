use std::{task, thread};
use std::time::Duration;
use crate::bluetooth::UPDATE_BLUETOOTH_DATA;
use crate::wifi_api::refresh_wifi_connectivity;

const WIFI_STAT_SCHEDULE_UPDATE: Duration = Duration::from_secs(15);
const BT_DATA_SCHEDULE_UPDATE: Duration = Duration::from_millis(350);
// TODO: Implement global weather refreshing
const WEATHER_SCHEDULE_UPDATE: Duration = Duration::from_secs(10_000);

pub fn start_wifi_con_update_thread() {
    thread::spawn(|| {
        loop {
            refresh_wifi_connectivity();
            thread::sleep(WIFI_STAT_SCHEDULE_UPDATE)
        }
    });
}

async fn updt_bt_data() {
    loop {
        UPDATE_BLUETOOTH_DATA().await;
        tokio::time::sleep(BT_DATA_SCHEDULE_UPDATE).await;
    }
}

pub fn start_bt_data_update_thread() {
    tokio::spawn((async || {loop {
        UPDATE_BLUETOOTH_DATA().await;
        tokio::time::sleep(BT_DATA_SCHEDULE_UPDATE).await;
    }})());
}