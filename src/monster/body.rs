use crate::parts::{eye::Eye, polygon::Polygon};
use crate::monster::Rawr;
use nannou::color::encoding::Srgb;
use nannou::color::rgb::Rgb;
use nannou::prelude::*;

pub struct Body {
    pub centroid: (f32, f32),
    pub color: Rgb<Srgb, u8>,
    pub outline: Rgb<Srgb, u8>,
    pub scale: f32,
}

impl Rawr for Body {
    fn rawr(&self, d: &Draw) {
        Polygon::new(6, self.centroid, self.scale, self.color, self.outline).show(d);

        d.ellipse()
            .x_y(self.centroid.0, self.centroid.1)
            .w(self.scale / 1.2)
            .h(self.scale * 1.4)
            .color(VIOLET);
    }
}
