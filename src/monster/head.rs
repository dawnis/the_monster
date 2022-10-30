use crate::parts::{eye::Eye, polygon::Polygon};
use crate::monster::Rawr;
use nannou::color::encoding::Srgb;
use nannou::color::rgb::Rgb;
use nannou::prelude::*;

pub struct Head {
    pub color: Rgb<Srgb, u8>,
    pub outline: Rgb<Srgb, u8>,
    pub scale: f32,
    pub bounding_rect: Rect,
}

impl Rawr for Head {
    fn rawr(&self, d: &Draw) {
        Polygon::new(
            12,
            (self.bounding_rect.x(), self.bounding_rect.y()),
            self.scale,
            self.color,
            self.outline,
        )
        .show(d);

        Eye::new(
            self.scale / 2.0,
            (self.bounding_rect.x()-self.scale / 3.0, self.bounding_rect.y()),
            WHITE,
            BLACK,
        )
        .show(d);

        Eye::new(
            self.scale / 2.0,
            (self.bounding_rect.x()+self.scale / 3.0, self.bounding_rect.y()),
            WHITE,
            BLACK,
        )
        .show(d);
    }
}
