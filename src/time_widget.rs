use chrono::{Datelike, Weekday};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::display::WIDGET_COLOR;
use crate::ui_renderer::{Drawable, UIContext, UIHelper, USize, EDGE_PADDING};

pub struct TimeWidget {
    position: Rect
}

impl TimeWidget {
    pub fn new(screen_size: &USize) -> Self {
        Self { position: screen_size.scale(0.4).to_rect(EDGE_PADDING(), EDGE_PADDING()) }
    }
    
    fn get_time_strs() -> (String, String) {
        let now = chrono::Local::now();

        let weekday = match now.weekday() {
            Weekday::Mon => {
                "Montag"
            }
            Weekday::Tue => {
                "Dienstag"
            }
            Weekday::Wed => {
                "Mittwoch"
            }
            Weekday::Thu => {
                "Donnerstag"
            }
            Weekday::Fri => {
                "Freitag"
            }
            Weekday::Sat => {
                "Samstag"
            }
            Weekday::Sun => {
                "Sonntag"
            }
        }.to_string();
        
        (format!("{weekday} der {}", now.format("%d.%m.%Y")), now.format("%H:%M:%S").to_string())
    }
}

impl Drawable for TimeWidget {
    fn draw(&self, ctx: &mut UIContext, texture_creator: &UIHelper) {
        let (date, time) = Self::get_time_strs();
        ctx.draw_rect(self.position, WIDGET_COLOR);
    }
}