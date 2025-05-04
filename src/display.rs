use std::thread;
use std::time::{Duration, Instant};
use sdl2::event::Event;
use crate::color::BG_DARKEST;
use crate::info_widget::InfoWidget;
use crate::ui_renderer::{init, UIContext, UIHelper};
use crate::weather_widget::WeatherWidget;

pub fn video_main() -> Result<(), String> {
    let (window, mut event_pump) = init()?;
    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let uihelper = UIHelper::new(&texture_creator);
    
    let mut ui = UIContext::new(canvas).unwrap();
    let ui_size = ui.size();

    let exit_time = Instant::now() + Duration::from_secs(3600);

    let weather_widget = WeatherWidget::new(&ui_size);
    let info_widget = InfoWidget::new(&ui_size);

    'running: loop {
        if Instant::now() > exit_time {
            break 'running;
        }
        
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }
        
        ui.clear(BG_DARKEST);
        ui.draw(&info_widget, &uihelper);
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