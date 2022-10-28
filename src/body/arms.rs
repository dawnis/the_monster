use nannou::color::encoding::Srgb;
use nannou::color::rgb::Rgb;
use nannou::color::IntoLinSrgba;
use nannou::draw::mesh::vertex::Point;
use nannou::draw::properties::ColorScalar;
use nannou::prelude::*;
use std::iter::Map;

pub struct Arms {
    pub color: Rgb<Srgb, u8>,
    pub outline: Rgb<Srgb, u8>,
    pub scale: f32,
}

impl Rawr for Arms {
    fn rawr(&self, d: &Draw) {
        d.ellipse()
            .x_y(-self.scale * 2.0, self.scale / 2.0)
            .w(self.scale * 4.0)
            .h(self.scale / 8.0)
            .z_degrees(-5.0)
            .color(self.outline);

        d.ellipse()
            .x_y(-self.scale * 2.0, self.scale / 2.0)
            .w(self.scale * 4.0 - 3.0)
            .h(self.scale / 8.0 - 3.0)
            .z_degrees(-5.0)
            .color(self.color);

        Polygon::new(
            3,
            (
                -self.scale * 4.0,
                self.scale / 2.0 + 2.0 * 2.0 * deg_to_rad(5.0).sin() * self.scale / 2.0,
            ),
            self.scale / 2.5,
            self.color,
            self.outline,
        )
        .show(d);

        d.ellipse()
            .x_y(self.scale * 2.0, self.scale / 2.0)
            .w(self.scale * 4.0)
            .h(self.scale / 8.0)
            .z_degrees(5.0)
            .color(self.outline);

        d.ellipse()
            .x_y(self.scale * 2.0, self.scale / 2.0)
            .w(self.scale * 4.0 - 3.0)
            .h(self.scale / 8.0 - 3.0)
            .z_degrees(5.0)
            .color(self.color);

        Polygon::new(
            3,
            (
                self.scale * 4.0,
                self.scale / 2.0 + 2.0 * deg_to_rad(5.0).sin() * self.scale / 2.0,
            ),
            self.scale / 2.5,
            self.color,
            self.outline,
        )
        .show(d);
    }
}
