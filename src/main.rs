use crate::display::video_main;
use crate::threads::start_wifi_con_update_thread;

mod display;
mod ui_renderer;
mod api;
mod errors;
mod weather_widget;
mod time_widget;
mod wifi_api;
mod threads;

pub(crate) const fn fraction(a: i32, b: i32) -> f32 {
    a as f32 / b as f32
}

fn main() {
    start_wifi_con_update_thread();
    video_main().expect("FAILED");
}