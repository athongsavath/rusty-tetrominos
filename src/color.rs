use crossterm::style::Color;
use rand::Rng;

/// Painting to the screen can be either Permanent or Temporary
///
/// In the case of Temporary painting, the painted character positions will be stored in the app's
/// temp vector, which allows them to be removed between renderings. This is useful for when the
/// piece is moving across the screen. Also, whenever you want a rendered character to be removed
/// upon the next render.
///
/// For Permanent painting, the painted characters positions will not be stored in the temp vector.
pub enum PaintType {
    Permanent,
    Temporary,
}

/// Returns a random color
///
/// There are a total of 16 colors, which are compatible amongst terminals, but I have only
/// selected the colors, which are colorful. e.g. not white, or dark white, or grey since the
/// border is grey
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
