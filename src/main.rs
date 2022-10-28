mod monster;
mod parts;
use crate::monster::Monster;
use crate::monster::{head::Head, body::Body, arms::Arms, legs::Legs};
use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let scale = 100.0;
    let outline_color = BLACK;
    let monster_color = MEDIUMORCHID;

    draw.background().color(MEDIUMTURQUOISE);

    let monster = Monster {
        parts: vec![
            Box::new(Head {
                scale: scale,
                color: monster_color,
                outline: outline_color,
            }),
            Box::new(Arms {
                scale: scale,
                color: monster_color,
                outline: outline_color,
            }),
            Box::new(Legs {
                scale: scale,
                color: monster_color,
                outline: outline_color,
            }),
            Box::new(Body {
                centroid: (0.0, 0.0),
                color: monster_color,
                outline: outline_color,
                scale: scale,
            }),
        ],
    };

    monster.make(&draw);

    draw.to_frame(app, &frame).unwrap();
}
