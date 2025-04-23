use crate::display::video_main;

mod display;
mod ui_renderer;
mod api;
mod errors;
mod weather_widget;

fn main() {
    video_main().expect("FAILED");
}