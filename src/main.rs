use std::fs;
use crate::bluetooth::{set_bluetooth_device_name, BluetoothController};
use crate::display::video_main;
use crate::parameters::load_device_name_or_default;
use crate::threads::{init_threads};
use crate::utils::{detect_undervoltage, DEBUG};

mod display;
mod ui_renderer;
mod api;
mod errors;
mod weather_widget;
mod info_widget;
mod wifi_api;
mod threads;
mod bluetooth;
mod color;
mod parameters;
mod logger;
mod utils;

pub(crate) const fn fraction(a: i32, b: i32) -> f32 {
    a as f32 / b as f32
}

#[tokio::main]
async fn main() {
    // Check for debug mode
    DEBUG.set(fs::exists("/home/jasper/parameters/DEBUG").expect("Failed to check for existence of DEBUG file")).expect("Failed to init DEBUG");
    // Init and set Bluetooth controller
    bluetooth::_BLUETOOTH_CTL.set(BluetoothController::new().await.expect("Failed to init bt-ctl")).expect("Failed to set bt-ctl");
    init_threads();
    video_main().expect("FAILED");
}

/*
use ffmpeg_the_third as ffmpeg;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use std::env;
use std::thread::sleep;
use std::time::{Duration, Instant};
use sdl2::rect::Rect;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("SDL_VIDEODRIVER", "KMSDRM");
    ffmpeg::init().unwrap();

    let file_path = env::args().nth(1).expect("Please provide a video file path");

    let mut ictx = ffmpeg::format::input(&file_path)?;
    let input = ictx
        .streams()
        .best(ffmpeg::media::Type::Video)
        .ok_or(ffmpeg::Error::StreamNotFound)?;
    let video_stream_index = input.index();

    let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
    let mut decoder = context_decoder.decoder().video()?;

    let mut scaler = ffmpeg::software::scaling::Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        ffmpeg::format::Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        ffmpeg::software::scaling::flag::Flags::BILINEAR,
    )?;

    // SDL2
    let sdl_ctx = sdl2::init()?;
    let video_subsystem = sdl_ctx.video()?;
    let window = video_subsystem
        .window("FFmpeg + SDL2", decoder.width(), decoder.height())
        .fullscreen_desktop()
        .vulkan()
        .build()?;
    let mut canvas = window.into_canvas().accelerated().present_vsync().build()?;
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGB24,
        decoder.width() as u32,
        decoder.height() as u32,
    )?;
    let mut event_pump = sdl_ctx.event_pump()?;

    let mut frame = ffmpeg::util::frame::Video::empty();
    let mut rgb_frame = ffmpeg::util::frame::Video::empty();

    let frame_rate = input.avg_frame_rate();
    let frame_duration = Duration::from_millis(1);/* if frame_rate.denominator() > 0 {
        Duration::from_secs_f64(frame_rate.denominator() as f64 / frame_rate.numerator() as f64)
    } else {
        Duration::from_millis(1)
    }; */

    for (stream, packet) in ictx.packets().map(|x| {x.unwrap()}) {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet)?;
            while decoder.receive_frame(&mut frame).is_ok() {
                scaler.run(&frame, &mut rgb_frame)?;

                // SDL2 render
                texture.update(None, rgb_frame.data(0), rgb_frame.stride(0) as usize)?;
                // canvas.clear();
                canvas.copy(&texture, None, Some(Rect::new(0,0, 1920, 1080)))?;
                canvas.present();

                // Event check
                for event in event_pump.poll_iter() {
                    if let Event::Quit {..}
                    | Event::KeyDown { keycode: Some(Keycode::Escape), .. } = event {
                        return Ok(());
                    }
                }

                // sleep(frame_duration);
            }
        }
    }

    // decoder.send_eof()?;
    // while decoder.receive_frame(&mut frame).is_ok() {
    //     scaler.run(&frame, &mut rgb_frame)?;
    //     texture.update(None, rgb_frame.data(0), rgb_frame.stride(0) as usize)?;
    //     canvas.clear();
    //     canvas.copy(&texture, None, None)?;
    //     canvas.present();
    //     // sleep(frame_duration);
    // }

    Ok(())
}
*/