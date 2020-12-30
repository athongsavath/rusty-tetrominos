use crossterm::style::Color;
use rand::Rng;

pub enum PaintType {
    Permanent,
    Temporary,
}

pub fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..=11) {
        0 => Color::Red,
        1 => Color::Green,
        2 => Color::Yellow,
        3 => Color::Blue,
        4 => Color::Magenta,
        5 => Color::Cyan,
        6 => Color::DarkRed,
        7 => Color::DarkGreen,
        8 => Color::DarkYellow,
        9 => Color::DarkBlue,
        10 => Color::DarkMagenta,
        11 => Color::DarkCyan,
        _ => panic!("RNG went out of range for random color."),
    }
}
