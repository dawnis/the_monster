mod monster;
mod parts;
use crate::monster::Monster;
use nannou::color::encoding::Srgb;
use nannou::color::rgb::Rgb;
use nannou::prelude::*;
use palette::named;
use structopt::StructOpt;

///A monster drawing program
#[derive(StructOpt, Debug)]
#[structopt(name = "the_monster")]
pub struct Opt {
    /// Set the color of the monster
    #[structopt(short, long, default_value = "blue")]
    color: String,
}

///Type alias for nannou color type
type Mrgb = Rgb<Srgb, u8>;

#[derive(Debug, Clone, Copy)]
enum Color {
    Yellow,
    Blue,
    HotPink,
    LawnGreen,
    Gold,
    Chocolate,
    Gray,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

impl From<String> for Color {
    fn from(s: String) -> Self {
        let s_lower = s.to_lowercase();
        match s_lower.as_str() {
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "pink" => Color::HotPink,
            "green" => Color::LawnGreen,
            "gold" => Color::Gold,
            "brown" => Color::Chocolate,
            _ => Color::Gray,
        }
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
    let opt = Opt::from_args();
    let color_param = Color::from(opt.color);

    Monster::new(
        model.monster_location,
        50.0,
        Mrgb::from(color_param),
        outline_color,
    )
    .make(&draw);

    draw.to_frame(app, &frame).unwrap();
}
