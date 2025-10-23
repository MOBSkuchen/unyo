use chrono::{Datelike, Weekday};
use sdl2::rect::{Point, Rect};
use crate::bluetooth::{_BLUETOOTH_DATA};
use crate::{fraction, get};
use crate::color::{color_from_hex, BG_SHADED, BG_TINTED, PB_EMPTY, PB_FULLY, TXT_DEFAULT, TXT_SUBTEXT};
use crate::parameters::load_device_name_or_default;
use crate::ui_renderer::{Drawable, UIContext, UIHelper, USize, IOTA};
use crate::utils::detect_undervoltage;
use crate::wifi_api::{_WIFI_STRENGTH_GLOB};

pub struct InfoWidget {
    position1: Rect,
    position2: Rect
}

fn format_time(seconds: u32) -> String {
    let minutes = seconds / 60;
    let seconds = seconds % 60;
    format!("{}:{:02}", minutes, seconds)
}


impl InfoWidget {
    pub fn new(screen_size: &USize) -> Self {
        let position1 = screen_size.scale_1_2(fraction(5, 9), 0.5).to_rect(IOTA(), IOTA());
        // let mut position2 = screen_size.scale_1_2(fraction(4, 9), 0.5);
        // position2.0 -= IOTA() as u32;
        Self {
            position1,
            position2: screen_size.scale_1_2(fraction(4, 9), 0.5).modify(-2 * IOTA(), -2 * IOTA()).to_rect(position1.x + position1.w, IOTA())
        }
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

impl Drawable for InfoWidget {
    fn draw(&self, ctx: &mut UIContext, uihelper: &UIHelper) {
        let path = get!(_WIFI_STRENGTH_GLOB).to_path();
        let xp = self.position1.x + 2 * IOTA();
        let jb_large_l_size = uihelper.font_owner.jb_large_l.char_dim();
        
        let (date, time) = Self::get_time_strs();
        
        ctx.draw_rect(self.position1, BG_TINTED);
        ctx.draw_rect(self.position2, BG_SHADED);

        let (x, y) = ctx.draw_text(xp, self.position1.y + 2 * IOTA(), &uihelper.font_owner.jb_large_l, time.as_str(), TXT_DEFAULT, uihelper);
        ctx.draw_image(x + jb_large_l_size.one() as i32, y - (jb_large_l_size.two() / 7) as i32, jb_large_l_size.scale_1(2f32).into(), path.as_str(), uihelper);

        // RED "UNDRVLT!" to notify of undervoltage
        if detect_undervoltage() {
            ctx.draw_text(x + 3 * jb_large_l_size.one() as i32, y - (jb_large_l_size.two() / 7) as i32, &uihelper.font_owner.jb_large_s, "VLT!", color_from_hex(0xFF0000), uihelper);
        }

        let (_, y) = ctx.draw_text(xp, y + 2 * jb_large_l_size.one() as i32, &uihelper.font_owner.jb_large_s, date.as_str(), TXT_SUBTEXT, uihelper);

        if let Some(track) = &*get!(_BLUETOOTH_DATA) {
            let title_y = y + 2 * jb_large_l_size.one() as i32;
            let artist_y = title_y + (uihelper.font_owner.jb_medium_l.char_dim().two() as f32 * 1.5) as i32;
            let line_y = artist_y + (1.8 * IOTA() as f32) as i32;
            
            let title_bounds = self.position1.w - IOTA() * 4 - xp;
            let artist_bounds = self.position1.w / 2 - 3 * xp;
            
            let size_of_title_text= uihelper.font_owner.jb_medium_l.size_of_text(track.title.as_str()).one() as i32;
            let size_of_artist_text= uihelper.font_owner.jb_medium_m.size_of_text(track.artist.as_str()).one() as i32;
            
            let delta_title = title_bounds - size_of_title_text;
            let delta_artist = artist_bounds - size_of_artist_text;

            // Title
            ctx.draw_text(xp + delta_title / 2, title_y, &uihelper.font_owner.jb_medium_l, track.title.as_str(), TXT_DEFAULT, uihelper);
            // Artist
            ctx.draw_text(xp + delta_artist / 2, artist_y, &uihelper.font_owner.jb_medium_m, track.artist.as_str(), TXT_SUBTEXT, uihelper);
            // Position
            let (x, _) = ctx.draw_text(artist_bounds + IOTA(), artist_y, &uihelper.font_owner.jb_medium_m, &format_time(track.position / 1000), TXT_SUBTEXT, uihelper);
            // Line
            let line_start = 5 * IOTA();
            let line_end = x + 40 * IOTA();
            let length = track.line_length(line_end - x - line_start);
            ctx.draw_line(Point::new(x + line_start, line_y), Point::new(line_end, line_y), IOTA(), PB_EMPTY);
            if length != 0 {
                ctx.draw_line(Point::new(x + line_start, line_y), Point::new(x + line_start + length, line_y), IOTA(), PB_FULLY);
            }
            // Duration
            ctx.draw_text(line_end + line_start, artist_y, &uihelper.font_owner.jb_medium_m, &format_time(track.duration / 1000), TXT_SUBTEXT, uihelper);

            // Tile 2

            let (_, y) = ctx.draw_text(self.position2.x + 2 * IOTA(), self.position2.y + IOTA(), &uihelper.font_owner.jb_medium_l, "Verbindung", TXT_DEFAULT, uihelper);
            ctx.draw_text(self.position2.x + 2 * IOTA(), y + (uihelper.font_owner.jb_medium_l.char_dim().two() as i32), &uihelper.font_owner.jb_medium_l_u, format!("Gerät: {}", track.name).as_str(), TXT_DEFAULT, uihelper);
        } else {
            ctx.draw_text(xp + 5 * IOTA(), y + 2 * jb_large_l_size.one() as i32, &uihelper.font_owner.jb_medium_l, "Suche nach geräten...", TXT_DEFAULT, uihelper);
            ctx.draw_text(xp + 5 * IOTA(), y + 3 * jb_large_l_size.one() as i32, &uihelper.font_owner.jb_medium_l, format!("Name: {}", load_device_name_or_default()).as_str(), TXT_SUBTEXT, uihelper);
        }
    }
}