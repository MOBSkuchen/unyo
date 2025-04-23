use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::api::WeatherInfo;
use crate::display::{TEXT_COLOR, WIDGET_COLOR};
use crate::ui_renderer::{Drawable, UIContext, UIHelper, USize, EDGE_PADDING};

use chrono::{Datelike, Duration, Local, Timelike, Weekday};
use crate::fraction;

fn day_of_week_with_offset(days_offset: i64) -> String {
    let today = Local::now().date_naive();
    let offset_date = today + Duration::days(days_offset + 1);
    let wd = offset_date.weekday();
    
    match wd {
        Weekday::Mon => {
            "MON"
        }
        Weekday::Tue => {
            "DIE"
        }
        Weekday::Wed => {
            "MIT"
        }
        Weekday::Thu => {
            "DON"
        }
        Weekday::Fri => {
            "FRE"
        }
        Weekday::Sat => {
            "SAM"
        }
        Weekday::Sun => {
            "SON"
        }
    }.to_string()
}

pub fn time_with_hour_offset(hours_offset: i64) -> String {
    let now = Local::now().naive_local();
    let offset_time = now + Duration::hours(hours_offset);
    format!("{}:00", offset_time.time().hour().to_string())
}

enum WeatherImage {
    Sun,
    Moon,
    Rain,
    Cloud
}

impl WeatherImage {
    pub fn to_path(&self) -> &str {
        match self {
            WeatherImage::Sun => {
                "/home/jasper/res/sun.png"
            }
            WeatherImage::Moon => {
                "/home/jasper/res/moon.png"
            }
            WeatherImage::Rain => {
                "/home/jasper/res/rain.png"
            }
            WeatherImage::Cloud => {
                "/home/jasper/res/cloudy.png"
            }
        }
    }
}

pub struct WeatherWidget {
    weather_info: WeatherInfo,
    position: Rect
}

impl WeatherWidget {
    pub fn new(weather_info: WeatherInfo, w_size: &USize) -> Self {
        let position = w_size.scale_1_2(fraction(5, 9), 0.5).to_rect(EDGE_PADDING(), (w_size.two() / 2) as i32 - EDGE_PADDING());
        Self {weather_info, position}
    }


    fn select_image_for_params(
        &self,
        rain: f64,
        cloud: Option<i64>,
        sun: Option<f64>,
        is_day: Option<bool>,
    ) -> WeatherImage {
        if is_day.is_some_and(|t| {!t}) {
            return WeatherImage::Moon;
        }

        if rain > 0.0 {
            return WeatherImage::Rain;
        }

        match (cloud, sun) {
            (Some(cloud_pct), _) if cloud_pct > 50 => WeatherImage::Cloud,
            (_, Some(sun_secs)) if sun_secs > 3600.0 => WeatherImage::Sun, // arbitrary "sunny" threshold
            _ => WeatherImage::Cloud,
        }
    }
}

fn add_degree(x: f64) -> String {
    format!("{x}°C")
}

impl Drawable for WeatherWidget {
    fn draw(&self, ctx: &mut UIContext, uihelper: &UIHelper) {
        let medium_l_char_size = uihelper.font_owner.jb_medium_l.char_dim();
        let medium_m_char_size = uihelper.font_owner.jb_medium_m.char_dim();
        let medium_s_char_size = uihelper.font_owner.jb_medium_s.char_dim();
        
        ctx.draw_rect(self.position, WIDGET_COLOR);
        let (x, y) = ctx.draw_text(self.position.x + EDGE_PADDING(), self.position.y + EDGE_PADDING(), &uihelper.font_owner.jb_medium_l, format!("WETTER (in {})", self.weather_info.city).as_str(), Color::GREY, uihelper);
        
        let w_current_p = self.select_image_for_params(self.weather_info.current.1, Some(self.weather_info.current.2), None, Some(self.weather_info.is_day));
        let (x, y) = ctx.draw_text(x + (medium_l_char_size.one() * 4) as i32, y, &uihelper.font_owner.jb_medium_l, format!("Aktuell: {} °C", self.weather_info.current.0).as_str(), TEXT_COLOR, uihelper);
        ctx.draw_image(x + (medium_l_char_size.one() * 3) as i32, y, medium_l_char_size.scale_1(2.5).into(), w_current_p.to_path(), uihelper);
        
        let (mut x, mut y) = (self.position.x + EDGE_PADDING(), y + 40 * EDGE_PADDING());
        let day_img_size = medium_s_char_size.scale_1_2(3f32, 1.8).into();
        let hour_img_size = medium_m_char_size.scale_1_2(3.5, 2f32).into();

        for (day, data) in self.weather_info.daily.iter().enumerate() {
            let name = day_of_week_with_offset(day as i64);
            
            let xp = x + EDGE_PADDING() * 8 * (day != 0) as i32;    // Bounding position for item
            
            let rebound = ctx.draw_text(xp, y, &uihelper.font_owner.jb_medium_l, name.as_str(), TEXT_COLOR, uihelper);
            x = rebound.0; y = rebound.1;
            
            let img = self.select_image_for_params(data.2, None, Some(data.3), None);
            ctx.draw_image(x + 2 * EDGE_PADDING(), y + EDGE_PADDING(), day_img_size, img.to_path(), uihelper);

            ctx.draw_text(xp, y + 3 * EDGE_PADDING() +  medium_s_char_size.two() as i32, &uihelper.font_owner.jb_medium_s, add_degree(data.0).as_str(), TEXT_COLOR, uihelper);
        }
        
        y = self.position.y + 15 * EDGE_PADDING();
        x = self.position.x + 5 * EDGE_PADDING();

        for (hour, data) in self.weather_info.hourly.iter().enumerate() {
            let dstr = add_degree(data.0);
            let name = time_with_hour_offset(hour as i64);

            let xp = x + EDGE_PADDING() * 6 * (hour != 0) as i32;

            let rebound = ctx.draw_text(xp, y, &uihelper.font_owner.jb_medium_l, name.as_str(), TEXT_COLOR, uihelper);
            x = rebound.0 + 3 * EDGE_PADDING(); y = rebound.1;
            
            // Center text, add a full char to the right if there is no comma/point, add 0.3 if there is
            let (_, ty) = ctx.draw_text(if dstr.len() == 5 { xp + medium_m_char_size.one() as i32 } else { xp + (medium_m_char_size.one() as f32 * 0.3) as i32 }, y + (1.5 * medium_m_char_size.two() as f32) as i32, &uihelper.font_owner.jb_medium_m, dstr.as_str(), TEXT_COLOR, uihelper);

            let img = self.select_image_for_params(data.1, Some(data.2), None, None);
            ctx.draw_image(xp + (medium_m_char_size.one() as f32 * 1.25) as i32, ty + (1.75 * medium_m_char_size.two() as f32) as i32, hour_img_size, img.to_path(), uihelper);

            if hour >= 4 {
                break
            }
        }

    }
}