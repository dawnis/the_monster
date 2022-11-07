pub mod eye;
pub mod polygon;
pub mod weapon;

/// A coordinate pair - the (0,0) default is the center of the frame
#[derive(Debug, Default, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
