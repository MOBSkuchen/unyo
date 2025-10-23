// Detects an undervoltage by running `vcgencmd get_throttled`, if it returns throttled=0x0 everything is OK
// See: https://www.raspberrypi.com/documentation/computers/os.html#vcgencmd
#[inline]
pub fn detect_undervoltage() -> bool {
    !std::process::Command::new("vcgencmd").arg("get_throttled").output().map(|t| t.stdout == b"throttled=0x0\n").unwrap_or(false)
}