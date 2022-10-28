use nannou::color::encoding::Srgb;
use nannou::color::rgb::Rgb;
use nannou::prelude::*;

pub struct Eye {
    radius: f32,
    centroid: (f32, f32),
    sclera: Rgb<Srgb, u8>,
    iris: Rgb<Srgb, u8>,
}

impl Eye {
    pub fn new(
        radius: f32,
        centroid: (f32, f32),
        sclera: Rgb<Srgb, u8>,
        iris: Rgb<Srgb, u8>,
    ) -> Self {
        Eye {
            radius,
            centroid,
            sclera,
            iris,
        }
    }

    pub fn show(&self, draw: &Draw) {
        draw.ellipse()
            .x_y(self.centroid.0, self.centroid.1)
            .w(self.radius)
            .h(self.radius)
            .color(self.sclera);

        draw.ellipse()
            .x_y(self.centroid.0, self.centroid.1)
            .w(self.radius / 2.0)
            .h(self.radius / 2.0)
            .color(self.iris);
    }
}
