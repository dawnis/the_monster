use crate::monster::Rawr;
use nannou::color::encoding::Srgb;
use nannou::color::rgb::Rgb;
use nannou::prelude::*;

pub struct Legs {
    pub color: Rgb<Srgb, u8>,
    pub outline: Rgb<Srgb, u8>,
    pub scale: f32,
}

impl Rawr for Legs {
    fn rawr(&self, d: &Draw) {
        d.rect()
            .x_y(-self.scale / 2.0, -self.scale * 2.0)
            .w(self.scale / 4.0 + 5.0)
            .h(self.scale * 2.0 + 5.0)
            .color(self.outline);

        d.rect()
            .x_y(-self.scale / 2.0, -self.scale * 2.0)
            .w(self.scale / 4.0)
            .h(self.scale * 2.0)
            .color(self.color);

        d.rect()
            .x_y(self.scale / 2.0, -self.scale * 2.0)
            .w(self.scale / 4.0 + 5.0)
            .h(self.scale * 2.0 + 5.0)
            .color(self.outline);

        d.rect()
            .x_y(self.scale / 2.0, -self.scale * 2.0)
            .w(self.scale / 4.0)
            .h(self.scale * 2.0)
            .color(self.color);
    }
}
