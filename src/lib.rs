use nannou::color::encoding::Srgb;
use nannou::color::rgb::Rgb;
use nannou::color::IntoLinSrgba;
use nannou::draw::mesh::vertex::Point;
use nannou::draw::properties::ColorScalar;
use nannou::prelude::*;
use std::iter::Map;

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

struct Eye {
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

struct Polygon {
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

pub struct Head {
    pub color: Rgb<Srgb, u8>,
    pub outline: Rgb<Srgb, u8>,
    pub scale: f32,
}

impl Rawr for Head {
    fn rawr(&self, d: &Draw) {
        Polygon::new(
            12,
            (0.0, self.scale * 2.0),
            self.scale,
            self.color,
            self.outline,
        )
        .show(d);

        Eye::new(
            self.scale / 2.0,
            (-self.scale / 4.0, self.scale * 2.0),
            WHITE,
            BLACK,
        )
        .show(d);

        Eye::new(
            self.scale / 2.0,
            (self.scale / 4.0, self.scale * 2.0),
            WHITE,
            BLACK,
        )
        .show(d);
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
        Polygon::new(6, self.centroid, self.scale, self.color, self.outline).show(d);

        d.ellipse()
            .x_y(self.centroid.0, self.centroid.1)
            .w(self.scale / 1.2)
            .h(self.scale * 1.4)
            .color(VIOLET);
    }
}

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
                self.scale / 2.0 + 2.0*2.0*deg_to_rad(5.0).sin() * self.scale / 2.0,
            ),
            self.scale/2.5,
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
                self.scale / 2.0 + 2.0*deg_to_rad(5.0).sin() * self.scale / 2.0,
            ),
            self.scale/2.5,
            self.color,
            self.outline,
        )
        .show(d);
    }
}

pub struct Legs {
    pub color: Rgb<Srgb, u8>,
    pub outline: Rgb<Srgb, u8>,
    pub scale: f32,
}

impl Rawr for Legs {
    fn rawr(&self, d: &Draw) {
        d.rect()
            .x_y(-self.scale / 2.0, -self.scale * 2.0)
            .w(self.scale / 4.0 + 5.0)
            .h(self.scale * 2.0 + 5.0)
            .color(self.outline);

        d.rect()
            .x_y(-self.scale / 2.0, -self.scale * 2.0)
            .w(self.scale / 4.0)
            .h(self.scale * 2.0)
            .color(self.color);

        d.rect()
            .x_y(self.scale / 2.0, -self.scale * 2.0)
            .w(self.scale / 4.0 + 5.0)
            .h(self.scale * 2.0 + 5.0)
            .color(self.outline);

        d.rect()
            .x_y(self.scale / 2.0, -self.scale * 2.0)
            .w(self.scale / 4.0)
            .h(self.scale * 2.0)
            .color(self.color);
    }
}
