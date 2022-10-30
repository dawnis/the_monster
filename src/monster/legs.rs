use crate::monster::Rawr;
use nannou::color::encoding::Srgb;
use nannou::color::rgb::Rgb;
use nannou::prelude::*;

pub struct Legs {
    pub color: Rgb<Srgb, u8>,
    pub outline: Rgb<Srgb, u8>,
    pub scale: f32,
    pub bounding_rect: Rect,
}

impl Rawr for Legs {
    fn rawr(&self, d: &Draw) {
        d.rect()
            .x(self.bounding_rect.x() - self.scale/2.5)
            .y(self.bounding_rect.y())
            .w(self.scale / 4.0 + 5.0)
            .h(self.scale * 2.0 + 5.0)
            .color(self.outline);

        d.rect()
            .x(self.bounding_rect.x() - self.scale/2.5)
            .y(self.bounding_rect.y())
            .w(self.scale / 4.0)
            .h(self.scale * 2.0)
            .color(self.color);

        d.rect()
            .x(self.bounding_rect.x() + self.scale/2.5)
            .y(self.bounding_rect.y())
            .w(self.scale / 4.0 + 5.0)
            .h(self.scale * 2.0 + 5.0)
            .color(self.outline);

        d.rect()
            .x(self.bounding_rect.x() + self.scale/2.5)
            .y(self.bounding_rect.y())
            .w(self.scale / 4.0)
            .h(self.scale * 2.0)
            .color(self.color);
    }
}
