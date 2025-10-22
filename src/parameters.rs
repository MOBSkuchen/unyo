use std::fs;

pub const P_DEVICE_NAME: &str = "/home/jasper/parameters/BLUETOOTH_DEVICE_NAME";

#[inline]
pub fn load_device_name_or_default() -> String {
    fs::read_to_string(P_DEVICE_NAME).map(|s| s.trim().to_string()).unwrap_or(String::from("Raspi Audio Player"))
}