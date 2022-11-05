use crate::Point;
use crate::monster::{Rawr, Mrgb};
use crate::parts::polygon::Polygon;
use nannou::prelude::*;

pub struct Body {
    pub centroid: Point,
    pub color: Mrgb,
    pub outline: Mrgb,
    pub scale: f32,
}

impl Body {
    pub fn bounding_box(&self) -> Rect {
        let tummy_shrink = 0.85;
        let box_actual_scale = 2.0 * tummy_shrink;
        Rect::from_x_y_w_h(
            self.centroid.x,
            self.centroid.y,
            box_actual_scale * self.scale,
            box_actual_scale * self.scale,
        )
    }
}

impl Rawr for Body {
    fn rawr(&self, d: &Draw) {
        Polygon::new(6, self.centroid, self.scale, self.color, self.outline).show(d);

        d.ellipse()
            .x_y(self.centroid.x, self.centroid.y)
            .w(self.scale / 1.2)
            .h(self.scale * 1.4)
            .color(VIOLET);
    }
}
