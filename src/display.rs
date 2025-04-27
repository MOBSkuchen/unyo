use std::thread;
use std::time::{Duration, Instant};
use sdl2::event::Event;
use sdl2::pixels::Color;
use crate::api::WeatherInfo;
use crate::bluetooth::BLUETOOTH_DATA;
use crate::time_widget::TimeWidget;
use crate::ui_renderer::{init, UIContext, UIHelper};

pub const BACKGROUND_COLOR: Color = Color::RGB(35, 34, 34);
pub const WIDGET_COLOR: Color = Color::RGB(58, 57, 57);
pub const TEXT_COLOR: Color = Color::RGB(0, 0, 0);

pub fn video_main() -> Result<(), String> {
    let (window, mut event_pump) = init()?;
    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let uihelper = UIHelper::new(&texture_creator);
    
    let mut ui = UIContext::new(canvas).unwrap();
    let ui_size = ui.size();

    let exit_time = Instant::now() + Duration::from_secs(3600);

    let weather_widget = WeatherInfo::auto_construct_widget(&ui_size);
    let time_widget = TimeWidget::new(&ui_size);

    'running: loop {
        if Instant::now() > exit_time {
            break 'running;
        }
        
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }
        
        ui.clear(BACKGROUND_COLOR);
        ui.draw(&time_widget, &uihelper);
        ui.draw(&weather_widget, &uihelper);
        ui.render();
        
        // Wait until next second
        let next_tick = Instant::now() + Duration::from_millis(200);
        while Instant::now() < next_tick {
            thread::sleep(Duration::from_millis(10));
        }
    };
    Ok(())
}