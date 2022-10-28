use nannou::color::encoding::Srgb;
use nannou::color::rgb::Rgb;
use nannou::prelude::*;

pub struct Head {
    pub color: Rgb<Srgb, u8>,
    pub outline: Rgb<Srgb, u8>,
    pub scale: f32,
}

impl Rawr for Head {
    fn rawr(&self, d: &Draw) {
        Polygon::new(
            12,
            (0.0, self.scale * 2.0),
            self.scale,
            self.color,
            self.outline,
        )
        .show(d);

        Eye::new(
            self.scale / 2.0,
            (-self.scale / 4.0, self.scale * 2.0),
            WHITE,
            BLACK,
        )
        .show(d);

        Eye::new(
            self.scale / 2.0,
            (self.scale / 4.0, self.scale * 2.0),
            WHITE,
            BLACK,
        )
        .show(d);
    }
}
