use crate::display::video_main;

mod display;
mod ui_renderer;
mod api;
mod errors;
mod weather_widget;
mod time_widget;

pub(crate) const fn fraction(a: i32, b: i32) -> f32 {
    a as f32 / b as f32
}

fn main() {
    video_main().expect("FAILED");
}