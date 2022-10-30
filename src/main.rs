mod monster;
mod parts;
use crate::monster::Monster;
use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let outline_color = BLACK;

    draw.background().color(MEDIUMTURQUOISE);

    Monster::new((0.0, 0.0), 20.0, MEDIUMORCHID, outline_color).make(&draw);

    Monster::new((150.0, 150.0), 30.0, GOLD, outline_color).make(&draw);

    Monster::new((-150.0, -150.0), 30.0, CHARTREUSE, outline_color).make(&draw);
    Monster::new((150.0, -150.0), 30.0, HOTPINK, outline_color).make(&draw);
    Monster::new((-150.0, 150.0), 30.0, INDIANRED, outline_color).make(&draw);
    draw.to_frame(app, &frame).unwrap();
}
