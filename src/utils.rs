use std::sync::OnceLock;
pub static DEBUG: OnceLock<bool> = OnceLock::new();

// Detects an undervoltage by running `vcgencmd get_throttled`, if it returns throttled=0x0 everything is OK
// See: https://www.raspberrypi.com/documentation/computers/os.html#vcgencmd
#[inline]
pub fn detect_undervoltage() -> bool {
    std::process::Command::new("vcgencmd").arg("get_throttled").output().map(|t| t.stdout != b"throttled=0x0\n").unwrap_or(false)
}

#[macro_export]
macro_rules! get_mut {
    ($a: ident) => {
        *$a.lock().expect("Failed to mutate Mutex Lock")
    };
}

#[macro_export]
macro_rules! set {
    ($a: ident, $b: expr) => {
        get_mut!($a) = $b
    };
}

#[macro_export]
macro_rules! get {
    (once $a:expr) => {
        $a.get().expect("OnceLock not initialized")
    };
    // --- DEFAULT ARM ---
    ($a:expr) => {
        $a.lock().expect("Failed to open Mutex Lock")
    };
}

#[macro_export]
macro_rules! debug {
    ($dbg: block) => {
        if *get!(once $crate::utils::DEBUG) {
            $dbg
        }
    };
    ($dbg: block, $rel: block) => {
        if *get!(once $crate::utils::DEBUG) {
            $dbg
        } else {
            $rel
        }
    };
}