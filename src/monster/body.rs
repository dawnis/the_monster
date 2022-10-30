use crate::monster::Rawr;
use crate::parts::{eye::Eye, polygon::Polygon};
use nannou::color::encoding::Srgb;
use nannou::color::rgb::Rgb;
use nannou::prelude::*;

pub struct Body {
    pub centroid: (f32, f32),
    pub color: Rgb<Srgb, u8>,
    pub outline: Rgb<Srgb, u8>,
    pub scale: f32,
}

impl Body {
    pub fn bounding_box(&self) -> Rect {
        let tummy_shrink = 0.85;
        let box_actual_scale = 2.0 * tummy_shrink;
        Rect::from_x_y_w_h(
            self.centroid.0,
            self.centroid.1,
            box_actual_scale * self.scale,
            box_actual_scale * self.scale,
        )
    }
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
