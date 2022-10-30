use crate::monster::Rawr;
use crate::parts::{polygon::Polygon, weapon::Weapon};
use nannou::color::encoding::Srgb;
use nannou::color::rgb::Rgb;
use nannou::prelude::*;

pub struct Arms {
    pub color: Rgb<Srgb, u8>,
    pub outline: Rgb<Srgb, u8>,
    pub scale: f32,
    pub left_bounding_rect: Rect,
    pub right_bounding_rect: Rect
}

impl Rawr for Arms {
    fn rawr(&self, d: &Draw) {
        d.ellipse()
            .xy(self.left_bounding_rect.xy())
            .wh(self.left_bounding_rect.wh())
            .z_degrees(-5.0)
            .color(self.outline);

        d.ellipse()
            .xy(self.left_bounding_rect.xy())
            .wh(self.left_bounding_rect.wh() - vec2(3.0, 3.0))
            .z_degrees(-5.0)
            .color(self.color);

        let left_hand = Rect::from_w_h(self.scale/2.5, self.scale/2.5).top_left_of(self.left_bounding_rect).shift_y(self.scale/4.0);

        Polygon::new(
            3,
            (left_hand.x(), left_hand.y()),
            self.scale / 2.5,
            self.color,
            self.outline,
        )
        .show(d);

        Weapon::new(
            String::from("circle"),
            self.scale/3.0,
            (left_hand.x(), left_hand.y()),
            SILVER,
            BLACK,
        )
        .show(d);

        d.ellipse()
            .xy(self.right_bounding_rect.xy())
            .wh(self.right_bounding_rect.wh())
            .z_degrees(5.0)
            .color(self.outline);

        d.ellipse()
            .xy(self.right_bounding_rect.xy())
            .wh(self.right_bounding_rect.wh() - vec2(3.0, 3.0))
            .z_degrees(5.0)
            .color(self.color);

        let right_hand = Rect::from_w_h(self.scale/2.5, self.scale/2.5).top_right_of(self.right_bounding_rect).shift_y(self.scale/4.0);

        Polygon::new(
            3,
            (right_hand.x(), right_hand.y()),
            self.scale / 2.5,
            self.color,
            self.outline,
        )
        .show(d);
    }
}
