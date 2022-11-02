mod monster;
mod parts;
use crate::monster::Monster;
use nannou::prelude::*;
use rand::Rng;

fn main() {
    nannou::app(model).update(update).run()
}

struct Model {
    _window: window::Id,
    pub monster_location: (f32, f32),
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model {
        _window,
        monster_location: (0.0, 0.0),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.monster_location = (
        model.monster_location.0 + 0.1,
        model.monster_location.1 + 0.1,
    );
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let outline_color = BLACK;

    draw.background().color(MEDIUMTURQUOISE);
    let color_array = vec![
        MEDIUMORCHID,
        YELLOW,
        ORANGE,
        BLUE,
        HOTPINK,
        LAWNGREEN,
        GOLD,
        DEEPPINK,
        CORAL,
        AQUAMARINE,
        CHOCOLATE,
    ];

    let mut rng = rand::thread_rng();
    let color_selection = rng.gen_range(0..color_array.len());

    Monster::new(
        model.monster_location,
        50.0,
        color_array[color_selection],
        outline_color,
    )
    .make(&draw);

    draw.to_frame(app, &frame).unwrap();
}
