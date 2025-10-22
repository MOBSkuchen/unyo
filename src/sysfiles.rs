use std::fs;

pub const P_DEVICE_NAME: &str = "/home/jasper/mut-data/BLUETOOTH_DEVICE_NAME";

#[inline]
pub fn load_device_name_or_default() -> String {
    fs::read_to_string(P_DEVICE_NAME).unwrap_or(String::from("Raspi Audio Player"))
}