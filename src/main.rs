mod lib;
use nannou::prelude::*;
use the_monster::{Body, Monster};

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(GRAY);

    let monster = Monster {
        parts: vec![Box::new(Body {
            centroid: (0.0, 0.0),
            scale: 200.0,
        })],
    };

    monster.make(&draw);

    draw.to_frame(app, &frame).unwrap();
}
