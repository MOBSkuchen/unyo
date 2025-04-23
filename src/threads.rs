use std::thread;
use std::time::Duration;
use crate::wifi_api::refresh_wifi_connectivity;

const WIFI_STAT_SCHEDULE_UPDATE: Duration = Duration::from_secs(15);

// TODO: Implement global weather refreshing
const WEATHER_SCHEDULE_UPDATE: Duration = Duration::from_secs(10_000);

pub fn start_wifi_con_update_thread() {
    thread::spawn(|| {
        refresh_wifi_connectivity();
        thread::sleep(WIFI_STAT_SCHEDULE_UPDATE)
    });
}