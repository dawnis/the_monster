use nannou::color::encoding::Srgb;
use nannou::color::rgb::Rgb;
use nannou::prelude::*;

#[derive(Debug)]
pub struct Weapon {
    shape: String,
    size: f32,
    centroid: (f32, f32),
    color: Rgb<Srgb, u8>,
    outline: Rgb<Srgb, u8>,
}

impl Weapon {
    pub fn new(
        shape: String,
        size: f32,
        centroid: (f32, f32),
        color: Rgb<Srgb, u8>,
        outline: Rgb<Srgb, u8>,
    ) -> Self {
        Weapon {
            shape,
            size,
            centroid,
            color,
            outline,
        }
    }

    pub fn show(&self, draw: &Draw) {
        if self.shape == "circle" {
            draw.ellipse()
                .x_y(self.centroid.0, self.centroid.1 + self.size * 5.0)
                .w(self.size + 5.0)
                .h(self.size * 10.0 + 5.0)
                .color(self.outline);

            draw.ellipse()
                .x_y(self.centroid.0, self.centroid.1 + self.size * 5.0)
                .w(self.size)
                .h(self.size * 10.0)
                .color(self.color);

            draw.ellipse()
                .x_y(self.centroid.0, self.centroid.1)
                .w(self.size * 2.0 + 5.0)
                .h(self.size * 2.0 + 5.0)
                .color(self.outline);

            draw.ellipse()
                .x_y(self.centroid.0, self.centroid.1)
                .w(self.size * 2.0)
                .h(self.size * 2.0)
                .color(self.color);
        }
    }
}
