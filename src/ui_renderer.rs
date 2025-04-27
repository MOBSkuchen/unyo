use std::sync::OnceLock;
use lazy_static::lazy_static;
use sdl2::EventPump;
use sdl2::gfx::primitives::{DrawRenderer};
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::ttf::{Sdl2TtfContext};
use sdl2::video::WindowContext;
use crate::errors::{UnyoError, UnyoResult};

const _TEXT_SIZE_CONST: f64 = 32_f64 / (1080 * 40) as f64;

pub static _EDGE_PADDING_GLOB: OnceLock<i32> = OnceLock::new();
pub static _TEXT_SIZE_MOD_GLOB: OnceLock<f64> = OnceLock::new();

pub fn get_custom_font_size(s: f64) -> u16 {
    (_TEXT_SIZE_MOD_GLOB.get().unwrap() * s) as u16
}

pub enum FontSize {
    SuperLarge = 16 * 10,
    LargeL = 15 * 10,
    LargeS = 9 * 10,
    MediumL = 5 * 10,
    MediumM = 4 * 10,
    MediumS = 3 * 10,
    Small = 2 * 10
}

impl From<FontSize> for f64 {
    fn from(size: FontSize) -> Self {
        (size as u32) as f64
    }
}

impl FontSize {
    #[allow(clippy::wrong_self_convention)]
    pub fn to_real_size(self) -> u16 {
        get_custom_font_size(self.into())
    }
}

#[allow(non_snake_case)]
pub fn EDGE_PADDING() -> i32 {
    *_EDGE_PADDING_GLOB.get().unwrap()
}

const FNT_PTH_ROBOTO: &str = "/home/jasper/res/Roboto-Medium.ttf";
const FNT_PTH_JETBRAINS_MONO: &str = "/home/jasper/res/JetBrainsMono-Medium.ttf";

lazy_static! {
    static ref TTF_CTX: Sdl2TtfContext = {
        let ttf_context: Sdl2TtfContext = sdl2::ttf::init().map_err(|e| e.to_string()).expect("Failed to init ttf");
        ttf_context
    };
}

#[derive(Copy, Clone)]
pub struct USize((u32, u32));

impl USize {
    #[inline]
    pub fn scale(&self, s: f32) -> USize {
        USize(((self.0.0 as f32 * s) as u32, (self.0.1 as f32 * s) as u32))
    }

    #[inline]
    pub fn scale_1(&self, s: f32) -> USize {
        USize(((self.0.0 as f32 * s) as u32, self.0.1))
    }

    #[inline]
    pub fn scale_1_2(&self, s1: f32, s2: f32) -> USize {
        USize(((self.0.0 as f32 * s1) as u32, (self.0.1 as f32 * s2) as u32))
    }

    #[inline]
    pub fn scale_2(&self, s: f32) -> USize {
        USize((self.0.0, (self.0.1 as f32 * s) as u32))
    }

    #[inline]
    pub fn one(&self) -> u32 {
        self.0.0
    }

    #[inline]
    pub fn two(&self) -> u32 {
        self.0.1
    }

    #[inline]
    pub fn to_rect(&self, x: i32, y: i32) -> Rect {
        Rect::new(x, y, self.0.0, self.0.1)
    }
}

impl Into<(u32, u32)> for USize {
    fn into(self) -> (u32, u32) {
        self.0
    }
}

impl From<(u32, u32)> for USize {
    fn from(value: (u32, u32)) -> Self {
        USize(value)
    }
}

pub enum AvailableFonts {
    Roboto,
    JetbrainsMono
}

impl AvailableFonts {
    pub fn to_path(&self) -> &'static str {
        match self {
            AvailableFonts::Roboto => {
                FNT_PTH_ROBOTO
            }
            AvailableFonts::JetbrainsMono => {
                FNT_PTH_JETBRAINS_MONO
            }
        }
    }
}

fn load_font<'a>(font_path: &str, size: u16) -> UnyoResult<sdl2::ttf::Font<'a, 'a>> {
    TTF_CTX.load_font(font_path, size)
        .map_err(|_| { UnyoError::UiLoadFont })
}

pub struct Font<'a>(sdl2::ttf::Font<'a, 'a>);

impl<'a> Font<'a> {
    pub fn new(font: sdl2::ttf::Font<'a, 'a>) -> Self {
        Self(font)
    }

    pub fn load_path(font: &str, size: u16) -> Self {
        load_font(font, size).expect("Failed to load font").into()
    }
    
    pub fn load(font: AvailableFonts, size: u16) -> Self {
        Self::load_path(font.to_path(), size)
    }

    pub fn char_dim(&self) -> USize {
        self.0.size_of_char('A').unwrap().into()
    }

    pub fn write_text(&self, text: &str, color: Color) -> (Surface, USize) {
        let size = self.0.size_of(text).unwrap();
        let surface = self.0.render(text).solid(color).unwrap();
        (surface, size.into())
    }

    pub fn size_of_text(&self, text: &str) -> USize {
        self.0.size_of(text).unwrap().into()
    }
}

impl<'a> From<sdl2::ttf::Font<'a, 'a>> for Font<'a> {
    fn from(value: sdl2::ttf::Font<'a, 'a>) -> Self {
        Font::new(value)
    }
}

pub struct FontOwner<'a> {
    pub jb_medium_l: Font<'a>,
    pub jb_medium_m: Font<'a>,
    pub jb_medium_s: Font<'a>,
    pub jb_large_l: Font<'a>,
    pub jb_large_s: Font<'a>,
}

impl<'a> FontOwner<'a> {
    pub fn new() -> Self {
        let jb_medium_l = Font::load(AvailableFonts::JetbrainsMono, FontSize::MediumL.to_real_size());
        let jb_medium_m = Font::load(AvailableFonts::JetbrainsMono, FontSize::MediumM.to_real_size());
        let jb_medium_s = Font::load(AvailableFonts::JetbrainsMono, FontSize::MediumS.to_real_size());

        let jb_large_l = Font::load(AvailableFonts::JetbrainsMono, FontSize::LargeL.to_real_size());
        let jb_large_s = Font::load(AvailableFonts::JetbrainsMono, FontSize::LargeS.to_real_size());

        Self {jb_medium_l, jb_medium_m, jb_medium_s, jb_large_l, jb_large_s}
    }
}

pub struct UIHelper<'a> {
    pub font_owner: FontOwner<'a>,
    pub texture_creator: &'a TextureCreator<WindowContext>
}

impl<'a> UIHelper<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        Self {font_owner: FontOwner::new(), texture_creator}
    }
    
    pub fn texture_from_surface(&self, surface: Surface) -> Texture {
        self.texture_creator.create_texture_from_surface(surface).unwrap()
    }
    
    pub fn image_texture(&self, path: &str) -> Texture {
        self.texture_creator.load_texture(path).unwrap()
    }
}

pub struct UIContext {
    canvas: WindowCanvas,
}

impl UIContext {
    pub fn new(canvas: WindowCanvas) -> UnyoResult<Self> {
        Ok(Self { canvas})
    }

    pub fn draw_texture(&mut self, texture: Texture, rect: Rect) {
        self.canvas.copy(&texture, None, rect).expect("Failed to draw texture");
    }

    pub fn draw_texture2(&mut self, texture: Texture, x: i32, y: i32, w: u32, h: u32) {
        self.canvas.copy(&texture, None, Rect::new(x, y, w, h)).expect("Failed to draw texture");
    }

    pub fn draw_texture3(&mut self, texture: &Texture, x: i32, y: i32, size: (u32, u32)) {
        self.canvas.copy(texture, None, Rect::new(x, y, size.0, size.1)).expect("Failed to draw texture");
    }

    pub fn draw_polygon(&mut self, vertices: Vec<(i16, i16)>, color: Color, filled: bool) {
        let mut vx = vec![];
        let mut vy = vec![];

        for vertex in vertices {
            vx.push(vertex.0);
            vy.push(vertex.1);
        }
        
        if filled {
            self.canvas
                .filled_polygon(vx.as_slice(), vy.as_slice(), color)
                .expect("Rendering a filled polygon failed!")
        }
        else {
            self.canvas
                .polygon(vx.as_slice(), vy.as_slice(), color)
                .expect("Rendering a polygon failed!")
        }
    }

    pub fn draw_rect(&mut self, rect: Rect, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(rect).expect("Failed to draw rectangle")
    }

    pub fn draw_line(&mut self, start: Point, end: Point, thickness: i32, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.thick_line(start.x as i16, start.y as i16, end.x as i16, end.y as i16, thickness as u8, color).expect("Failed to draw a line")
    }

    pub fn draw_text(&mut self, x: i32, y: i32, font: &Font, text: &str, color: Color, uihelper: &UIHelper) -> (i32, i32) {
        let (surface, size) = font.write_text(text, color);
        let size = size.into();
        let texture = uihelper.texture_from_surface(surface);
        self.draw_texture3(&texture, x, y, size);
        (x + size.0 as i32, y)
    }

    pub fn draw_image(&mut self, x: i32, y: i32, size: (u32, u32), path: &str, uihelper: &UIHelper) -> (i32, i32) {
        let texture = uihelper.image_texture(path);
        self.draw_texture3(&texture, x, y, size);
        (x + size.0 as i32, y)
    }
    
    pub fn center(&self, w: u32, h: u32) -> (i32, i32) {
        let (ww, wh) = self.canvas.window().size();
        (((ww - w) / 2) as i32, ((wh - h) / 2) as i32)
    }
    
    pub fn center_rect(&self, w: u32, h: u32) -> Rect {
        let center = self.center(w, h);
        Rect::new(center.0, center.1, w, h)
    }
    
    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }
    
    pub fn render(&mut self) {
        self.canvas.present();
    }
    
    pub fn size(&self) -> USize {
        self.canvas.window().size().into()
    }
    
    pub fn draw(&mut self, drawable: &impl Drawable, uihelper: &UIHelper) {
        drawable.draw(self, uihelper)
    }
}

pub trait Drawable {
    fn draw(&self, ctx: &mut UIContext, uihelper: &UIHelper);
}

pub fn surface_to_texture<'a>(texture_creator: &'a TextureCreator<WindowContext>, surface: Surface) -> Texture<'a> {
    texture_creator.create_texture_from_surface(surface).unwrap()
}

pub fn init() -> Result<(sdl2::video::Window, EventPump), String> {
    let sdl_context = sdl2::init()?;
    let event_pump = sdl_context.event_pump()?;
    let video_subsystem = sdl_context.video()?;
    // Currently only needs PNG, maybe add JPG support?
    let _image_context = sdl2::image::init(InitFlag::PNG)?;
    let (w, h) = video_subsystem.display_bounds(0).expect("Display not found!!").size();
    _EDGE_PADDING_GLOB.set((h / 108) as i32).expect("Failed to set global");
    _TEXT_SIZE_MOD_GLOB.set(h as f64 * _TEXT_SIZE_CONST).expect("Failed to set global");
    sdl_context.mouse().show_cursor(false);
    
    println!("Unyo running ({w}x{h})");

    // Try forcing SDL to use KMSDRM (no X11)
    std::env::set_var("SDL_VIDEODRIVER", "KMSDRM");

    let window = video_subsystem
        .window("unyo", w, h)
        .vulkan()
        .build()
        .map_err(|e| e.to_string())?;
    
    Ok((window, event_pump))
}