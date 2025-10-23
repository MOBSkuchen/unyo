use crate::logln;

pub fn detect_undervoltage() -> bool {
    let output = std::process::Command::new("vcgencmd").arg("get_throttled").output().ok();
    if let Some(output) = output {
        logln!("Run vcgencmd: {} ({})", String::from_utf8(output.stdout).unwrap(), output.status.code().unwrap());
    }
    false
}