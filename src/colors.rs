use nannou::color::encoding::Srgb;
use nannou::color::rgb::Rgb;
use palette::named;

///Type alias for nannou color type
pub type Mrgb = Rgb<Srgb, u8>;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Yellow,
    Aqua,
    HotPink,
    LawnGreen,
    Gold,
    Chocolate,
    Gray,
    HoneyDew,
    Black,
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
            "blue" => Color::Aqua,
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
