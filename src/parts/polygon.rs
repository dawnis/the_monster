use nannou::color::encoding::Srgb;
use nannou::color::rgb::Rgb;
use nannou::prelude::*;

pub struct Polygon {
    sides: usize,
    centroid: (f32, f32),
    scale: f32,
    fill: Rgb<Srgb, u8>,
    outline: Rgb<Srgb, u8>,
}

impl Polygon {
    pub fn new(
        sides: usize,
        centroid: (f32, f32),
        scale: f32,
        fill: Rgb<Srgb, u8>,
        outline: Rgb<Srgb, u8>,
    ) -> Self {
        Polygon {
            sides,
            centroid,
            scale,
            fill,
            outline,
        }
    }

    pub fn show(&self, draw: &Draw) {
        assert!(self.sides <= 360);
        let step = 360usize / self.sides;
        let points = (0..=360).step_by(step.into()).map(|i| {
            let radian = deg_to_rad(i as f32);
            let x = radian.sin() * self.scale + self.centroid.0;
            let y = radian.cos() * self.scale + self.centroid.1;
            (pt2(x, y), self.fill)
        });
        draw.polygon().points_colored(points);
        let points = (0..=360).step_by(step.into()).map(|i| {
            let radian = deg_to_rad(i as f32);
            let x = radian.sin() * self.scale + self.centroid.0;
            let y = radian.cos() * self.scale + self.centroid.1;
            (pt2(x, y), self.outline)
        });
        draw.polyline().weight(3.0).points_colored(points);
    }
}
