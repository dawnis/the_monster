pub mod arms;
pub mod body;
pub mod head;
pub mod legs;
use crate::monster::{arms::Arms, body::Body, head::Head, legs::Legs};
use crate::{Mrgb, Point};
use nannou::prelude::*;

pub trait Rawr {
    fn rawr(&self, d: &Draw);
}

pub struct Monster {
    pub parts: Vec<Box<dyn Rawr>>,
}

impl Monster {
    pub fn new(location: Point, scale: f32, color: Mrgb, outline: Mrgb) -> Self {
        let mbody = Body {
            centroid: location,
            color: color,
            outline: outline,
            scale: scale,
        };

        let tummy_rect = mbody.bounding_box();

        Monster {
            parts: vec![
                Box::new(Arms {
                    scale: scale,
                    color: color,
                    outline: outline,
                    left_bounding_rect: Rect::from_w_h(scale * 2.5, scale / 4.0)
                        .left_of(tummy_rect)
                        .align_top_of(tummy_rect),
                    right_bounding_rect: Rect::from_w_h(scale * 2.5, scale / 4.0)
                        .right_of(tummy_rect)
                        .align_top_of(tummy_rect),
                }),
                Box::new(Legs {
                    scale: scale,
                    color: color,
                    outline: outline,
                    bounding_rect: tummy_rect.below(tummy_rect),
                }),
                Box::new(mbody),
                Box::new(Head {
                    scale: scale,
                    color: color,
                    outline: outline,
                    bounding_rect: tummy_rect.above(tummy_rect),
                }),
            ],
        }
    }
    pub fn make(&self, d: &Draw) {
        for part in self.parts.iter() {
            part.rawr(d);
        }
    }
}
