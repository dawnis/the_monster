mod monster;
mod parts;
use crate::monster::Monster;
use nannou::color::encoding::Srgb;
use palette::named;
use nannou::color::rgb::Rgb;
use nannou::prelude::*;
use rand::Rng;

///Type alias for nannou color type
type Mrgb = Rgb<Srgb, u8>;

#[derive(Debug, Clone, Copy)]
enum Color {
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
}

impl ToString for Color {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

impl From<Color> for Mrgb {
    fn from(c: Color) -> Self {
        named::from_str(&c.to_string()).unwrap()
    }
}

/// A coordinate pair - the (0,0) default is the center of the frame
#[derive(Debug, Default, Clone, Copy)]
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

fn main() {
    nannou::app(model).update(update).run()
}

struct Model {
    _window: window::Id,
    pub monster_location: Point,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model {
        _window,
        monster_location: Point::new(0.0, 0.0),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.monster_location = Point::new(app.mouse.x, app.mouse.y);
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
