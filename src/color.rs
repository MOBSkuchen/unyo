// COLORS.
// * BACKGROUND:
//  - 2c2e2d   # DARKEST
//  - 464847   # SHADED
//  - 7b7c7c   # TINTED
// * TEXT:
//  - e5e5e5   # DEFAULT
//  - cacbca   # SUBTEXT
//  - 959696   # WEATHER

use sdl2::pixels::Color;

pub(crate) const fn color_from_hex(hex: u32) -> Color {
    let r = ((hex >> 16) & 0xFF) as u8;
    let g = ((hex >> 8) & 0xFF) as u8;
    let b = (hex & 0xFF) as u8;
    Color::RGB(r, g, b)
}

pub const BG_DARKEST: Color = color_from_hex(0x2c2e2d);
pub const BG_SHADED: Color = color_from_hex(0x464847);
pub const BG_TINTED: Color = color_from_hex(0x7b7c7c);

pub const TXT_DEFAULT: Color = color_from_hex(0xe5e5e5);
pub const TXT_SUBTEXT: Color = color_from_hex(0xcacbca);
pub const TXT_WEATHER: Color = color_from_hex(0x9559696);

pub const PB_EMPTY: Color = color_from_hex(0xd186e0);
pub const PB_FULLY: Color = color_from_hex(0x86e08d);

pub const DIV_LINE: Color = color_from_hex(0x86e08d);