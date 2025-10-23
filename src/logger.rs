use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::{LazyLock, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::{debug, get};

#[inline]
fn unique_stamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

#[inline]
fn get_logfile_path() -> String {
    // In debug mode, we only really care about one log file
    debug!({
        fs::remove_dir_all("/home/jasper/logs").expect("Failed to empty logs dir");
        fs::create_dir("/home/jasper/logs").expect("Failed to create logs dir");
        "/home/jasper/logs/DEBUG_LOG".to_string()
    },{
        format!("/home/jasper/logs/{}", unique_stamp())
    })
}

pub static _LOGFILE_HANDLE: LazyLock<Mutex<File>> =
    LazyLock::new(|| { Mutex::from(OpenOptions::new().append(true).create(true).open(get_logfile_path()).unwrap()) });

#[inline]
pub(crate) fn _log(s: &str) {
    get!(_LOGFILE_HANDLE).write_all(s.as_bytes()).unwrap();
}

#[inline]
pub(crate) fn _log_flush() {
    get!(_LOGFILE_HANDLE).flush().unwrap();
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        $crate::logger::_log(format!($($arg)*).as_str());
        $crate::logger::_log_flush();
    }};
}

#[macro_export]
macro_rules! logln {
    () => {
        $crate::logger::_log("\n");
        $crate::logger::_log_flush();
    };
    ($($arg:tt)*) => {{
        $crate::logger::_log(format!($($arg)*).as_str());
        $crate::logger::_log("\n");
        $crate::logger::_log_flush();
    }};
}