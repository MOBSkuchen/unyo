use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use crate::api::WeatherInfo;
use crate::display::{TEXT_COLOR, WIDGET_COLOR};
use crate::ui_renderer::{Drawable, UIContext, UIHelper, EDGE_PADDING};

use chrono::{Datelike, Duration, Local, Timelike, Weekday};

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
    pub fn new(weather_info: WeatherInfo, w_size: (u32, u32)) -> Self {
        let (width, height) = w_size;
        let position = Rect::new(EDGE_PADDING(), height as i32 - (height / 3) as i32 - EDGE_PADDING(), width / 2, ((height / 3) as i32 - EDGE_PADDING()) as u32);
        Self {weather_info, position}
    }


    fn select_image_for_params(&self, rain: f64, cloud: Option<i64>, sun: Option<f64>) -> WeatherImage {
        if !self.weather_info.is_day {
            return WeatherImage::Moon;
        }
        
        if rain > 1_f64 {
            return WeatherImage::Rain;
        }
        
        if let Some(cloud) = cloud {
            if cloud > 60 {
                return WeatherImage::Cloud;
            }
        } else if let Some(sun) = sun {
            if sun > 45_000_f64 {
                return WeatherImage::Sun;
            }
        }
        
        WeatherImage::Sun
    }
}

impl Drawable for WeatherWidget {
    fn draw(&self, ctx: &mut UIContext, uihelper: &UIHelper) {
        let medium_l_char_size = uihelper.font_owner.jb_medium_l.char_dim();
        let medium_m_char_size = uihelper.font_owner.jb_medium_m.char_dim();
        let medium_s_char_size = uihelper.font_owner.jb_medium_s.char_dim();
        
        ctx.draw_rect(self.position, WIDGET_COLOR);
        let (x, y) = ctx.draw_text(self.position.x + EDGE_PADDING(), self.position.y + EDGE_PADDING(), &uihelper.font_owner.jb_medium_l, format!("WETTER (in {})", self.weather_info.city).as_str(), Color::GREY, uihelper);
        
        let w_current_p = self.select_image_for_params(self.weather_info.current.1, Some(self.weather_info.current.2), None);
        let (x, y) = ctx.draw_text(x + 3 * EDGE_PADDING(), y, &uihelper.font_owner.jb_medium_l, format!("Aktuell: {} °C", self.weather_info.current.0).as_str(), TEXT_COLOR, uihelper);
        ctx.draw_image(x + 5 * EDGE_PADDING(), y, medium_l_char_size.scale_1(2.5).into(), w_current_p.to_path(), uihelper);
        
        let (mut x, mut y) = (self.position.x + EDGE_PADDING(), y + 25 * EDGE_PADDING());
        let day_img_size = medium_s_char_size.scale_1(3_f32).scale_2(1.8).into();

        for (day, data) in self.weather_info.daily.iter().enumerate() {
            let name = day_of_week_with_offset(day as i64);
            
            let xp = x + EDGE_PADDING() * 8 * (day != 0) as i32;    // Bounding position for item
            
            let rebound = ctx.draw_text(xp, y, &uihelper.font_owner.jb_medium_l, name.as_str(), TEXT_COLOR, uihelper);
            x = rebound.0; y = rebound.1;
            
            let img = self.select_image_for_params(data.2, None, Some(data.3));
            ctx.draw_image(x + 2 * EDGE_PADDING(), y + EDGE_PADDING(), day_img_size, img.to_path(), uihelper);

            ctx.draw_text(xp, y + EDGE_PADDING() +  medium_s_char_size.two() as i32, &uihelper.font_owner.jb_medium_l, data.0.to_string().as_str(), TEXT_COLOR, uihelper);
        }
        
        y = self.position.y + 10 * EDGE_PADDING();
        x = self.position.x + EDGE_PADDING();
        
        let hour_img_size = medium_m_char_size.scale_1(3_f32).scale_2(1.8).into();

        for (hour, data) in self.weather_info.hourly.iter().enumerate() {
            let name = time_with_hour_offset(hour as i64);

            let xp = x + EDGE_PADDING() * 8 * (hour != 0) as i32;

            let rebound = ctx.draw_text(xp, y, &uihelper.font_owner.jb_medium_m, name.as_str(), TEXT_COLOR, uihelper);
            x = rebound.0 + 3 * EDGE_PADDING(); y = rebound.1;

            ctx.draw_text(xp + (medium_m_char_size.one() / 2) as i32, y + medium_m_char_size.two() as i32, &uihelper.font_owner.jb_medium_m, data.0.to_string().as_str(), TEXT_COLOR, uihelper);

            let img = self.select_image_for_params(data.1, Some(data.2), None);
            ctx.draw_image(xp + medium_m_char_size.one() as i32, y + 2 * medium_m_char_size.two() as i32, hour_img_size, img.to_path(), uihelper);

            if hour >= 4 {
                break
            }
        }

    }

    fn get_pos(&self) -> Rect {
        self.position
    }
}