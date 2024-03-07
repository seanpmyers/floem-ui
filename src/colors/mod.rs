use floem::peniko::Color;

pub const RGB_MAX: u8 = 255;
pub const RGB_MIN: u8 = 0;
pub const RGBA_OPAQUE: u8 = 255;
pub const RBGA_TRANSPARENT: u8 = 0;

pub struct Colors {}

impl Colors {
    pub const BLACK: Color = Color {
        r: RGB_MIN,
        g: RGB_MIN,
        b: RGB_MIN,
        a: RGBA_OPAQUE,
    };

    pub const WHITE: Color = Color {
        r: RGB_MAX,
        g: RGB_MAX,
        b: RGB_MAX,
        a: RGBA_OPAQUE,
    };
}
