use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::style::{self, Color, Colorize};
use crossterm::terminal::{ScrollUp, SetSize};
use crossterm::{cursor, execute, QueueableCommand};
use rand::Rng;
use std::collections::VecDeque;
use std::io::{stdout, Stdout, Write};
use std::time::Duration;

#[derive(Clone, Copy)]
enum Piece {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

enum PaintType {
    Permanent,
    Temporary,
}

//struct Piece {}

//impl Piece {
//fn rotate() {}
//}

//pub trait Rotate {
//fn rotate_cw(&self) -> Self;
//fn rotate_ccw(&self) -> Self;
//}

enum Command {
    Empty,
    Left,
    Right,
    Up,
    Down,
    Space,
}

static O: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]];

static I0: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]];
static J0: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [1, 0, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0]];
static L0: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0]];
static S0: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0]];
static T0: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0]];
static Z0: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]];

static IR: &'static [[u8; 4]; 4] = &[[0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0]];
static JR: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 0, 0], [0, 1, 0, 0]];
static LR: &'static [[u8; 4]; 4] = &[[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]];
static SR: &'static [[u8; 4]; 4] = &[[0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]];
static TR: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0], [0, 1, 0, 0]];
static ZR: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 0, 1, 0], [0, 1, 1, 0], [0, 1, 0, 0]];

static I2: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]];
static J2: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 0], [0, 0, 1, 0]];
static L2: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 0], [1, 0, 0, 0]];
static S2: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 0, 0, 0], [0, 1, 1, 0], [1, 1, 0, 0]];
static T2: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0]];
static Z2: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 1, 0]];

static IL: &'static [[u8; 4]; 4] = &[[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]];
static JL: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0]];
static LL: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]];
static SL: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [1, 0, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0]];
static TL: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0]];
static ZL: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0], [1, 0, 0, 0]];

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

#[derive(Clone)]
struct Point(u16, u16);

struct IPiece<'a> {
    color: u8,
    piece: &'a [[u8; 4]; 4],
}

impl IPiece<'_> {
    fn new(color: u8) -> Self {
        Self { color, piece: &I0 }
    }
}

impl JPiece<'_> {
    fn new(color: u8) -> Self {
        Self { color, piece: &J0 }
    }
}

struct JPiece<'a> {
    color: u8,
    piece: &'a [[u8; 4]; 4],
}

impl LPiece<'_> {
    fn new(color: u8) -> Self {
        Self { color, piece: &L0 }
    }
}

struct LPiece<'a> {
    color: u8,
    piece: &'a [[u8; 4]; 4],
}

impl OPiece<'_> {
    fn new(color: u8) -> Self {
        Self { color, piece: &O }
    }
}

struct OPiece<'a> {
    color: u8,
    piece: &'a [[u8; 4]; 4],
}

struct SPiece<'a> {
    color: u8,
    piece: &'a [[u8; 4]; 4],
}

impl SPiece<'_> {
    fn new(color: u8) -> Self {
        Self { color, piece: &S0 }
    }
}

struct TPiece<'a> {
    color: u8,
    piece: &'a [[u8; 4]; 4],
}

impl TPiece<'_> {
    fn new(color: u8) -> Self {
        Self { color, piece: &T0 }
    }
}

struct ZPiece<'a> {
    color: u8,
    piece: &'a [[u8; 4]; 4],
}

impl ZPiece<'_> {
    fn new(color: u8) -> Self {
        Self { color, piece: &Z0 }
    }
}

struct Board {
    width: usize,
    height: usize,
    board: [[u8; WIDTH]; HEIGHT],
}

impl Board {
    fn new() -> Self {
        Self {
            width: WIDTH,
            height: HEIGHT,
            board: [[0u8; WIDTH]; HEIGHT],
        }
    }
}

struct App {
    board: Board,
    pieces: VecDeque<Piece>,
    temp: Vec<Point>,
    //pieces: vec![randomPiece(), randomPiece(), randomPiece()]
    //pieces: [Piece; 3],
    width: usize,
    height: usize,
    score: i32,
    lines: i32,
    level: i32,
    stdout: Stdout,
}

fn random_piece() -> Piece {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..7) {
        0 => Piece::I,
        1 => Piece::J,
        2 => Piece::L,
        3 => Piece::O,
        4 => Piece::S,
        5 => Piece::T,
        6 => Piece::Z,
        _ => panic!("This should not be possible"),
    }
}

fn match_key(code: KeyCode) -> Command {
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

impl App {
    fn updateFrame() {}
    fn calculateCollision() {}
    fn nextPiece() {}
    fn completedRow() {}

    fn gravityTick() {}

    fn run(&mut self) -> crossterm::Result<()> {
        // Use gravityTick game loop here
        crossterm::terminal::enable_raw_mode()?;

        let mut piece = random_piece();
        let mut now = std::time::Instant::now();
        let mut r = 0;
        let mut c = 5;

        loop {
            // First test without gravity
            if poll(Duration::from_millis(100))? {
                match read()? {
                    Event::Key(event) => {
                        self.stdout.flush()?;
                        match match_key(event.code) {
                            // Have to verify nothing intersects the walls
                            Command::Left => c -= 1,
                            Command::Right => c += 1,
                            Command::Down => r += 1,
                            Command::Up => piece = piece.rotate(),
                            _ => {}
                        }
                        self.queue_clear_piece();
                        self.paintPiece(piece, r, c, 0, PaintType::Temporary)?;
                        //execute!(stdout, SetSize(20, 10));
                        //println!("{:?}", crossterm::terminal::size());
                    }
                    Event::Resize(width, height) => {
                        // Clear everything and write everything else back
                        println!("New size {}x{}", width, height);

                        self.clear();
                        // Write everything back to the screen
                    }
                    //Event::Key(event) => execute!(stdout, ScrollUp(5))?,
                    //Event::Key(event) => println!("{:?}", event),
                    _ => {}
                }
            }
            if now.elapsed().as_millis() > 500 {
                now = std::time::Instant::now();
                r += 1;
                // TODO: If intersection, this hsould be permanent
                self.queue_clear_piece();
                self.temp.clear();
                self.paintPiece(piece, r, c, 0, PaintType::Temporary)?;
            }
            // TODO: Check game tick here
        }
    }

    fn new(height: usize, width: usize) -> Self {
        let mut pieces = VecDeque::with_capacity(3);
        for _ in 0..3 {
            pieces.push_back(random_piece());
        }

        Self {
            height,
            width,
            board: Board::new(),
            level: 0,
            score: 0,
            lines: 0,
            stdout: stdout(),
            pieces,
            temp: vec![],
        }
    }

    fn clear(&mut self) -> crossterm::Result<()> {
        let (height, width) = crossterm::terminal::size().expect("Tetris crashed");
        //let multiplier = std::cmp::min(height / 42, width / 20);
        for r in 0..height {
            for c in 0..width {
                self.stdout
                    .queue(cursor::MoveTo(r, c))?
                    .queue(style::PrintStyledContent("█".black()))?;
            }
        }

        Ok(())
    }

    fn queue_clear_piece(&mut self) -> crossterm::Result<()> {
        for Point(row, column) in self.temp.clone().iter() {
            self.paint(row.clone(), column.clone(), Color::Black)?;
        }
        Ok(())
    }

    fn paintPiece(
        &mut self,
        piece: Piece,
        row: u16,
        column: u16,
        _color: u16,
        paint_type: PaintType,
    ) -> crossterm::Result<()> {
        let next_piece = match piece {
            Piece::I => I0,
            Piece::J => J0,
            Piece::L => L0,
            Piece::O => O,
            Piece::S => S0,
            Piece::T => T0,
            Piece::Z => Z0,
        };

        for r in 0..next_piece.len() {
            for c in 0..next_piece[0].len() {
                if next_piece[r][c] == 1 {
                    self.paint(row + r as u16, column + c as u16, Color::Magenta)?;
                    match paint_type {
                        PaintType::Temporary => {
                            self.temp.push(Point(row + r as u16, column + c as u16));
                        }
                        _ => {}
                    };
                }
            }
        }
        self.stdout.flush();
        Ok(())
    }

    fn paintNextPiece(&mut self) -> crossterm::Result<()> {
        for i in 0..self.pieces.len() {
            self.paintPiece(
                self.pieces[i].clone(),
                2 + (i * 4) as u16,
                14,
                0,
                PaintType::Permanent,
            );
        }
        Ok(())
    }

    fn paint(&mut self, row: u16, column: u16, color: Color) -> crossterm::Result<()> {
        let (width, height) = crossterm::terminal::size().expect("Tetris crashed");
        //let factor = std::cmp::min(height / 21, width / 36);
        let mut width_multiplier = 1;
        while width_multiplier * 2 + width_multiplier <= width / 36 {
            width_multiplier += 1;
        }
        let mut height_multiplier = height / 21;

        let game_multiplier = std::cmp::min(width_multiplier, height_multiplier);
        let info_multiplier = std::cmp::max(game_multiplier / 2, 1);

        const COLUMN_MULTIPLIER: u16 = 2;
        const GAME_WIDTH: u16 = 12;

        if column < GAME_WIDTH {
            for y in (row * game_multiplier)..((row + 1) * game_multiplier) {
                for x in (column * game_multiplier * COLUMN_MULTIPLIER)
                    ..((column + 1) * game_multiplier * COLUMN_MULTIPLIER)
                {
                    self.stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::PrintStyledContent(
                            crossterm::style::style("█").with(color),
                        ));
                    //crossterm::queue!(
                    //stdout,
                    //style::PrintStyledContent(crossterm::style("█").with(color))
                    //);
                    //self.stdout
                    //.queue(cursor::MoveTo(x, y))?
                    //.queue(Print(crossterm::style("█").with(color).on(Color::Black)));
                }
            }
        } else {
            for y in (row * info_multiplier)..((row + 1) * info_multiplier) {
                for x in (column * info_multiplier * COLUMN_MULTIPLIER)
                    ..((column + 1) * info_multiplier * COLUMN_MULTIPLIER)
                {
                    self.stdout
                        .queue(cursor::MoveTo(x + GAME_WIDTH * game_multiplier, y))?
                        .queue(style::PrintStyledContent("█".magenta()))?;
                }
            }
        }

        Ok(())
    }
}

fn main() {
    println!("Starting Tetris!");
    // TODO: Whenever there is a resize, i need to repaint everything

    // I should be getting height and width from the current terminal
    //let height = 1280;
    //let width = 640;

    // First thing to do is to make the whole screen black
    let (height, width) = crossterm::terminal::size().expect("Tetris crashed");
    let mut app = App::new(height as usize, width as usize);
    app.clear();
    crossterm::style::SetBackgroundColor(crossterm::style::Color::Black);
    for r in 0..21 {
        for c in 0..1 {
            app.paint(r, c, Color::Grey);
        }
        for c in 11..12 {
            app.paint(r, c, Color::Grey);
        }
    }
    for r in 21..=21 {
        for c in 0..12 {
            app.paint(r, c, Color::Grey);
        }
    }
    app.paintNextPiece();
    app.stdout.flush();

    app.run();
}
