use std::thread;
use std::time::{Duration, Instant};
use sdl2::event::Event;
use sdl2::pixels::Color;
use crate::api::WeatherInfo;
use crate::ui_renderer::{init, surface_to_texture, AvailableFonts, Font, FontSize, UIContext};

pub const BACKGROUND_COLOR: Color = Color::RGB(35, 34, 34);
pub const WIDGET_COLOR: Color = Color::RGB(58, 57, 57);
pub const TEXT_COLOR: Color = Color::RGB(0, 0, 0);

pub fn video_main() -> Result<(), String> {
    let (window, mut event_pump) = init()?;
    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    
    let jetbrains = Font::load(AvailableFonts::JetbrainsMono, FontSize::SuperLarge.to_real_size());
    
    let mut ui = UIContext::new(canvas).unwrap();
    ui.clear(BACKGROUND_COLOR);

    let exit_time = Instant::now() + Duration::from_secs(3600);

    let weather_widget = WeatherInfo::auto_construct_widget(ui.size());

    'running: loop {
        if Instant::now() > exit_time {
            break 'running;
        }
        
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }

        let now = chrono::Local::now();
        let time_string = now.format("%H:%M:%S").to_string();
        let (txt_surface, size) = jetbrains.write_text(time_string.as_str(), Color::BLUE);
        let texture = surface_to_texture(&texture_creator, txt_surface);
        let center = ui.center_rect(size.one(), size.two());
        ui.clear(BACKGROUND_COLOR);
        ui.draw_texture(texture, center);
        ui.draw(&weather_widget, &texture_creator);
        ui.render();
        
        // Wait until next second
        let next_tick = Instant::now() + Duration::from_millis(200);
        while Instant::now() < next_tick {
            thread::sleep(Duration::from_millis(10));
        }
    };
    Ok(())
}