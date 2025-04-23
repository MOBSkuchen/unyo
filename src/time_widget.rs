use chrono::{Datelike, Weekday};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::display::WIDGET_COLOR;
use crate::fraction;
use crate::ui_renderer::{Drawable, UIContext, UIHelper, USize, EDGE_PADDING};

pub struct TimeWidget {
    position: Rect
}

impl TimeWidget {
    pub fn new(screen_size: &USize) -> Self {
        Self { position: screen_size.scale_1_2(fraction(5, 9), 0.5).to_rect(EDGE_PADDING(), EDGE_PADDING()) }
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
        
        (format!("{weekday} {}", now.format("%d.%m.%Y")), now.format("%H:%M:%S").to_string())
    }
}

impl Drawable for TimeWidget {
    fn draw(&self, ctx: &mut UIContext, uihelper: &UIHelper) {
        let xp = self.position.x + 2 * EDGE_PADDING();
        let jb_large_l_size = uihelper.font_owner.jb_large_l.char_dim();
        
        let (date, time) = Self::get_time_strs();
        
        ctx.draw_rect(self.position, WIDGET_COLOR);

        let (x, y) = ctx.draw_text(xp, self.position.y + 2 * EDGE_PADDING(), &uihelper.font_owner.jb_large_l, time.as_str(), Color::WHITE, uihelper);
        let (x, y) = ctx.draw_text(xp, y + 2 * jb_large_l_size.one() as i32, &uihelper.font_owner.jb_large_s, date.as_str(), Color::WHITE, uihelper);
    }
}