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
