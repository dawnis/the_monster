use std::iter::Map;
use nannou::color::encoding::Srgb;
use nannou::color::rgb::Rgb;
use nannou::prelude::*;

pub trait Rawr {
    fn rawr(&self, d: &Draw);
}

pub struct Monster {
    pub parts: Vec<Box<dyn Rawr>>,
}

impl Monster {
    pub fn make(&self, d: &Draw) {
        for part in self.parts.iter() {
            part.rawr(d);
        }
    }
}

pub struct Body {
    pub centroid: (f32, f32),
    pub color: Rgb<Srgb, u8>,
    pub outline: Rgb<Srgb, u8>,
    pub scale: f32,
}

impl Rawr for Body {
    fn rawr(&self, d: &Draw) {
    let points = (0..=360).step_by(72).map(|i| {
        let radian = deg_to_rad(i as f32);
        let x = radian.sin() * self.scale + self.centroid.0;
        let y = radian.cos() * self.scale + self.centroid.1;
        (pt2(x, y), self.color)
    });
    d.polygon().points_colored(points);
    let points = (0..=360).step_by(72).map(|i| {
        let radian = deg_to_rad(i as f32);
        let x = radian.sin() * self.scale + self.centroid.0;
        let y = radian.cos() * self.scale + self.centroid.1;
        (pt2(x, y), self.outline)
    });
    d.polyline().weight(3.0).points_colored(points);
    }
}
