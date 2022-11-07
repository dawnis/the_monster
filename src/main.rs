mod colors;
mod logging;
mod monster;
mod parts;
use crate::colors::{Color, Mrgb};
use crate::logging::init_logging;
use crate::monster::Monster;
use crate::parts::Point;
use lazy_static::lazy_static;
use log::*;
use nannou::prelude::*;
use structopt::StructOpt;

///A monster drawing program
#[derive(StructOpt, Debug)]
#[structopt(name = "the_monster")]
pub struct Opt {
    /// Set the color of the monster
    #[structopt(short, long, default_value = "blue")]
    color: String,
    /// Verbose mode (-v: warn, -vv: info, -vvv: debug, , -vvvv or more: trace)
    #[structopt(short, long, parse(from_occurrences))]
    verbosity: u8,
}

lazy_static! {
    pub static ref OPT: Opt = Opt::from_args();
}

fn main() {
    init_logging(OPT.verbosity);
    nannou::app(model).update(update).run()
}

struct Model {
    _window: window::Id,
    pub monster_location: Point,
    pub monster_moving: bool,
    pub flag_updated_time: f64
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model {
        _window,
        monster_location: Point::new(0.0, 0.0),
        monster_moving: true,
        flag_updated_time: 0.0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    use nannou::state::mouse::ButtonPosition;

    if let ButtonPosition::Down(_) = app.mouse.buttons.left() {
        if app.duration.since_start.secs() - model.flag_updated_time > 0.5 {
            model.monster_moving = !model.monster_moving;
            model.flag_updated_time = app.duration.since_start.secs();
            debug!("Monster moving set to {:?}", model.monster_moving);
        }
    }

    if model.monster_moving {
        model.monster_location = Point::new(app.mouse.x, app.mouse.y);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let outline_color = Color::Black;
    let bg_color = Color::HoneyDew;

    draw.background().color(Mrgb::from(bg_color));
    let color_param = Color::from(OPT.color.clone());

    Monster::new(
        model.monster_location,
        50.0,
        Mrgb::from(color_param),
        Mrgb::from(outline_color),
    )
    .make(&draw);

    draw.to_frame(app, &frame).unwrap();
}
