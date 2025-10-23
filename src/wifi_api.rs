use std::process::Command;
use std::sync::{LazyLock, Mutex};

pub static _WIFI_STRENGTH_GLOB: LazyLock<Mutex<WifiSignalBars>> = LazyLock::new(|| {Mutex::from(WifiSignalBars::NoSignal)});

#[derive(Debug, Clone, Copy)]
pub enum WifiSignalBars {
    NoSignal,  // 0 bars
    Weak,      // 1 bar
    Fair,      // 2 bars
    Good,      // 3 bars
    Excellent, // 4 bars
}

impl WifiSignalBars {
    pub fn to_path(self) -> String {
        match self {
            WifiSignalBars::NoSignal => {
                "/home/jasper/res/0-wifi.png"
            }
            WifiSignalBars::Weak => {
                "/home/jasper/res/1-wifi.png"
            }
            WifiSignalBars::Fair => {
                "/home/jasper/res/2-wifi.png"
            }
            WifiSignalBars::Good => {
                "/home/jasper/res/3-wifi.png"
            }
            WifiSignalBars::Excellent => {
                "/home/jasper/res/4-wifi.png"
            }
        }.to_string()
    }
}

fn signal_to_bars(signal: u32) -> WifiSignalBars {
    match signal {
        0 => WifiSignalBars::NoSignal,
        1..=25 => WifiSignalBars::Weak,
        26..=50 => WifiSignalBars::Fair,
        51..=75 => WifiSignalBars::Good,
        76..=100 => WifiSignalBars::Excellent,
        _ => WifiSignalBars::NoSignal,
    }
}

pub fn get_wifi_signal_bars() -> Option<WifiSignalBars> {
    let output = Command::new("nmcli")
        .args(["-t", "-f", "ACTIVE,SIGNAL", "dev", "wifi"])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        // Format: yes:<signal> or no:<signal>
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 && parts[0] == "yes" {
            if let Ok(signal_percent) = parts[1].parse::<u32>() {
                return Some(signal_to_bars(signal_percent));
            }
        }
    }

    None
}