use crossterm::event::KeyCode;

pub enum Command {
    Empty,
    Left,
    Right,
    Up,
    Down,
    Space,
}

pub fn match_key(code: KeyCode) -> Command {
    match code {
        KeyCode::Left => Command::Left,
        KeyCode::Right => Command::Right,
        KeyCode::Down => Command::Down,
        KeyCode::Up => Command::Up,
        KeyCode::Char(c) => match c {
            'a' => Command::Left,
            's' => Command::Down,
            'd' => Command::Right,
            'w' => Command::Up,
            'A' => Command::Left,
            'S' => Command::Down,
            'D' => Command::Right,
            'W' => Command::Up,
            'h' => Command::Left,
            'j' => Command::Down,
            'l' => Command::Right,
            'k' => Command::Up,
            ' ' => Command::Space,
            _ => Command::Empty,
        },
        _ => Command::Empty,
    }
}
