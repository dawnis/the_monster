use nannou::prelude::rgb::Rgb;
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
    pub scale: f32,
}

impl Rawr for Body {
    fn rawr(&self, d: &Draw) {
        d.ellipse()
            .x_y(self.centroid.0, self.centroid.1)
            .w_h(self.scale, self.scale)
            .color(STEELBLUE);
    }
}
