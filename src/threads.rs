use crate::{get_mut, logln};
use std::thread;
use std::time::Duration;
use std::panic;
use std::panic::PanicHookInfo;
use std::sync::OnceLock;
use crate::api::UPDATE_WEATHER_INFO;
use crate::bluetooth::{_BLUETOOTH_CTL, _BLUETOOTH_DATA};
use crate::{get, set};
use crate::wifi_api::{get_wifi_signal_bars, WifiSignalBars, _WIFI_STRENGTH_GLOB};

const WIFI_STAT_SCHEDULE_UPDATE: Duration = Duration::from_secs(15);
const BT_DATA_SCHEDULE_UPDATE: Duration = Duration::from_millis(350);
const WEATHER_SCHEDULE_UPDATE: Duration = Duration::from_secs(10_000);

pub static RUN_THREADS: OnceLock<bool> = OnceLock::new();

fn start_wifi_con_update_thread() {
    thread::spawn(|| {
        while *get!(once RUN_THREADS) {
            set!(_WIFI_STRENGTH_GLOB, get_wifi_signal_bars().unwrap_or(WifiSignalBars::NoSignal));
            thread::sleep(WIFI_STAT_SCHEDULE_UPDATE)
        }
    });
}

fn start_bt_data_update_thread() {
    tokio::spawn(async {while *get!(once RUN_THREADS) {
        set!(_BLUETOOTH_DATA, get!(once _BLUETOOTH_CTL).poll().await);
        tokio::time::sleep(BT_DATA_SCHEDULE_UPDATE).await;
    }});
}

fn start_weather_update_thread() {
    thread::spawn(|| {
        while *get!(once RUN_THREADS) {
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

pub fn panic_hook(panic_hook_info: &PanicHookInfo) {
    // Terminate all threads
    RUN_THREADS.set(false).expect("Failed to terminate threads");
    // Log trace
    logln!("Ran into panic during execution: {panic_hook_info}");
    // Exit
    std::process::exit(1);
}