use crossterm::event::{poll, read, Event};
use crossterm::style::{self, Colorize};
use crossterm::terminal::{ScrollUp, SetSize};
use crossterm::{cursor, execute, QueueableCommand};
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

//struct Piece {}

//impl Piece {
//fn rotate() {}
//}

//pub trait Rotate {
//fn rotate_cw(&self) -> Self;
//fn rotate_ccw(&self) -> Self;
//}

enum Command {
    LEFT,
    RIGHT,
    UP,
    DOWN,
    SPACE,
}

static O: &'static [[u8; 4]; 3] = &[[0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]];

static I0: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]];
static J0: &'static [[u8; 3]; 3] = &[[1, 0, 0], [1, 1, 1], [0, 0, 0]];
static L0: &'static [[u8; 3]; 3] = &[[0, 0, 1], [1, 1, 1], [0, 0, 0]];
static S0: &'static [[u8; 3]; 3] = &[[0, 1, 1], [1, 1, 0], [0, 0, 0]];
static T0: &'static [[u8; 3]; 3] = &[[0, 1, 0], [1, 1, 1], [0, 0, 0]];
static Z0: &'static [[u8; 3]; 3] = &[[1, 1, 0], [0, 1, 1], [0, 0, 0]];

static IR: &'static [[u8; 4]; 4] = &[[0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0]];
static JR: &'static [[u8; 3]; 3] = &[[0, 1, 1], [0, 1, 0], [0, 1, 0]];
static LR: &'static [[u8; 3]; 3] = &[[0, 1, 0], [0, 1, 0], [0, 1, 1]];
static SR: &'static [[u8; 3]; 3] = &[[0, 1, 0], [0, 1, 1], [0, 0, 1]];
static TR: &'static [[u8; 3]; 3] = &[[0, 1, 0], [0, 1, 1], [0, 1, 0]];
static ZR: &'static [[u8; 3]; 3] = &[[0, 0, 1], [0, 1, 1], [0, 1, 0]];

static I2: &'static [[u8; 4]; 4] = &[[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]];
static J2: &'static [[u8; 3]; 3] = &[[0, 0, 0], [1, 1, 1], [0, 0, 1]];
static L2: &'static [[u8; 3]; 3] = &[[0, 0, 0], [1, 1, 1], [1, 0, 0]];
static S2: &'static [[u8; 3]; 3] = &[[0, 0, 0], [0, 1, 1], [1, 1, 0]];
static T2: &'static [[u8; 3]; 3] = &[[0, 0, 0], [1, 1, 1], [0, 1, 0]];
static Z2: &'static [[u8; 3]; 3] = &[[0, 0, 0], [1, 1, 0], [0, 1, 1]];

static IL: &'static [[u8; 4]; 4] = &[[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]];
static JL: &'static [[u8; 3]; 3] = &[[0, 1, 0], [0, 1, 0], [1, 1, 0]];
static LL: &'static [[u8; 3]; 3] = &[[1, 1, 0], [0, 1, 0], [0, 1, 0]];
static SL: &'static [[u8; 3]; 3] = &[[1, 0, 0], [1, 1, 0], [0, 1, 0]];
static TL: &'static [[u8; 3]; 3] = &[[0, 1, 0], [1, 1, 0], [0, 1, 0]];
static ZL: &'static [[u8; 3]; 3] = &[[0, 1, 0], [1, 1, 0], [1, 0, 0]];

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

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
    piece: &'a [[u8; 3]; 3],
}

impl LPiece<'_> {
    fn new(color: u8) -> Self {
        Self { color, piece: &L0 }
    }
}

struct LPiece<'a> {
    color: u8,
    piece: &'a [[u8; 3]; 3],
}

impl OPiece<'_> {
    fn new(color: u8) -> Self {
        Self { color, piece: &O }
    }
}

struct OPiece<'a> {
    color: u8,
    piece: &'a [[u8; 4]; 3],
}

struct SPiece<'a> {
    color: u8,
    piece: &'a [[u8; 3]; 3],
}

impl SPiece<'_> {
    fn new(color: u8) -> Self {
        Self { color, piece: &S0 }
    }
}

struct TPiece<'a> {
    color: u8,
    piece: &'a [[u8; 3]; 3],
}

impl TPiece<'_> {
    fn new(color: u8) -> Self {
        Self { color, piece: &T0 }
    }
}

struct ZPiece<'a> {
    color: u8,
    piece: &'a [[u8; 3]; 3],
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
    pieces: [Piece; 3],
    width: usize,
    height: usize,
    score: i32,
    lines: i32,
    level: i32,
    stdout: Stdout,
}

impl App {
    fn updateFrame() {}
    fn calculateCollision() {}
    fn nextPiece() {}
    fn completedRow() {}

    fn gravityTick() {}

    fn run(&mut self) -> crossterm::Result<()> {
        // Use gravityTick game loop here
        let mut stdout = stdout();
        crossterm::terminal::enable_raw_mode()?;
        execute!(stdout, SetSize(self.height as u16, self.width as u16))?;
        loop {
            // First test without gravity
            if poll(Duration::from_millis(100))? {
                match read()? {
                    Event::Key(event) => {
                        stdout
                            .queue(cursor::MoveTo(0, 5))?
                            .queue(style::PrintStyledContent("█".magenta()))?;
                        stdout.flush()?;
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
        }
    }

    fn new(height: usize, width: usize) -> Self {
        Self {
            height,
            width,
            board: Board::new(),
            level: 0,
            score: 0,
            lines: 0,
            stdout: stdout(),
            pieces: [Piece::I, Piece::J, Piece::T],
        }
    }

    fn clear(&self) -> crossterm::Result<()> {
        let (height, width) = crossterm::terminal::size().expect("Tetris crashed");
        let mut stdout = stdout();
        //let multiplier = std::cmp::min(height / 42, width / 20);
        for r in 0..height {
            for c in 0..width {
                stdout
                    .queue(cursor::MoveTo(r, c))?
                    .queue(style::PrintStyledContent("█".black()))?;
            }
        }

        Ok(())
    }

    fn paintPiece(
        &mut self,
        piece: Piece,
        row: u16,
        column: u16,
        _color: u16,
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
                    self.paint(row + r as u16, column + c as u16, 0)?;
                }
            }
        }
        //match piece {
        //Piece::I => {
        //for r in 0..I0.len() {
        //for c in 0..I0[0].len() {
        //if I0[r][c] == 1 {
        //self.paint(row + r as u16, column + c as u16, 0)?;
        //}
        //}
        //}
        //}
        //Piece::J => {
        //for r in 0..J0.len() {
        //for c in 0..J0[0].len() {
        //if J0[r][c] == 1 {
        //self.paint(row + r as u16, column + c as u16, 0)?;
        //}
        //}
        //}
        //}
        //Piece::L => {
        //for r in 0..L0.len() {
        //for c in 0..L0[0].len() {
        //if L0[r][c] == 1 {
        //self.paint(row + r as u16, column + c as u16, 0)?;
        //}
        //}
        //}
        //}
        //Piece::O => {
        //for r in 0..O.len() {
        //for c in 0..O[0].len() {
        //if O[r][c] == 1 {
        //self.paint(row + r as u16, column + c as u16, 0)?;
        //}
        //}
        //}
        //}
        //Piece::S => {
        //for r in 0..S0.len() {
        //for c in 0..S0[0].len() {
        //if S0[r][c] == 1 {
        //self.paint(row + r as u16, column + c as u16, 0)?;
        //}
        //}
        //}
        //}
        //Piece::T => {
        //for r in 0..T0.len() {
        //for c in 0..T0[0].len() {
        //if T0[r][c] == 1 {
        //self.paint(row + r as u16, column + c as u16, 0)?;
        //}
        //}
        //}
        //}
        //Piece::Z => {
        //for r in 0..Z0.len() {
        //for c in 0..Z0[0].len() {
        //if Z0[r][c] == 1 {
        //self.paint(row + r as u16, column + c as u16, 0)?;
        //}
        //}
        //}
        //}
        //}
        Ok(())
    }

    fn paintNextPiece(&mut self) -> crossterm::Result<()> {
        for i in 0..self.pieces.len() {
            self.paintPiece(self.pieces[i].clone(), 2 + (i * 5) as u16, 14, 0);
        }
        Ok(())
    }

    fn paint(&mut self, row: u16, column: u16, _color: u16) -> crossterm::Result<()> {
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
                        .queue(style::PrintStyledContent("█".grey()))?;
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
            app.paint(r, c, 9);
        }
        for c in 11..12 {
            app.paint(r, c, 0);
        }
    }
    for r in 21..=21 {
        for c in 0..12 {
            app.paint(r, c, 0);
        }
    }
    app.paintNextPiece();
    app.stdout.flush();

    app.run();
}
