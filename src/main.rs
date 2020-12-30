use board::Board;
use color::{random_color, PaintType};
use command::{match_key, Command};
use crossterm::event::{poll, read, Event};
use crossterm::style::{self, Color, Colorize};
use crossterm::terminal::{ScrollUp, SetSize};
use crossterm::{cursor, execute, QueueableCommand};
use piece::{get_piece, random_piece, rotate, Piece};
use std::collections::VecDeque;
use std::convert::TryInto;
use std::io::{stdout, Stdout, Write};
use std::time::Duration;

mod board;
mod color;
mod command;
mod piece;

#[derive(Clone)]
struct Point(u16, u16);

struct App {
    board: Board,
    pieces: VecDeque<(Piece, Color)>,
    temp: Vec<Point>,
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
    fn nextPiece(&mut self) -> (Piece, Color) {
        let sol = self.pieces.pop_front();
        self.pieces.push_back((random_piece(), random_color()));
        sol.unwrap()
    }

    fn completedRow() {}

    fn gravityTick() {}

    fn run(&mut self) -> crossterm::Result<()> {
        // Use gravityTick game loop here

        let (mut piece, mut color) = self.nextPiece();

        let mut now = std::time::Instant::now();
        let mut r: i16 = 0;
        let mut c: i16 = 4;

        self.paint_piece(piece, r as u16, c as u16, color, PaintType::Temporary)?;
        loop {
            if poll(Duration::from_millis(100))? {
                match read()? {
                    Event::Key(event) => {
                        match match_key(event.code) {
                            // Have to verify nothing intersects the walls
                            Command::Left => {
                                if !self.board.detect_collision(piece, r, c - 1) {
                                    c -= 1;
                                }
                            }
                            Command::Right => {
                                if !self.board.detect_collision(piece, r, c + 1) {
                                    c += 1;
                                }
                            }
                            Command::Down => {
                                if !self.board.detect_collision(piece, r + 1, c) {
                                    r += 1;
                                }
                            }
                            Command::Up => loop {
                                piece = rotate(piece);
                                if !self.board.detect_collision(piece, r, c) {
                                    break;
                                }
                            },
                            Command::Space => {
                                loop {
                                    if self.board.detect_collision(piece, r + 1, c) {
                                        break;
                                    }
                                    r += 1;
                                }
                                // Ensure that enough time elaphsed to make this piece permanent
                                now -= std::time::Duration::new(5, 0);
                            }
                            _ => {}
                        }
                        self.queue_clear_piece();
                        self.paint_piece(piece, r as u16, c as u16, color, PaintType::Temporary)?;
                    }
                    Event::Resize(width, height) => {
                        // Clear everything and write everything else back
                        println!("New size {}x{}", width, height);

                        self.clear_screen();
                        // TODO: Repaint everything back to the screen
                    }
                    _ => {}
                }
            }
            if now.elapsed().as_millis() > 500 {
                now = std::time::Instant::now();
                if self.board.detect_collision(piece, r + 1, c) {
                    // TODO: If intersection, this hsould be permanent
                    self.temp.clear();
                    // TODO: Pop the next piece!
                    // TODO: Check for completed lines
                    //      Increment lines by completed lines
                    self.paint_piece(piece, r as u16, c as u16, color, PaintType::Permanent)?;
                    self.board.save(piece, r, c, 0);
                    r = 0;
                    c = 4;
                    let (new_piece, new_color) = self.nextPiece();
                    self.clear_next_piece()?;
                    self.paint_next_piece()?;
                    piece = new_piece;
                    color = new_color;
                    self.paint_piece(piece, r as u16, c as u16, color, PaintType::Temporary)?;
                    self.stdout.flush()?;
                } else {
                    r += 1;
                    self.queue_clear_piece()?;
                    self.temp.clear();
                    self.paint_piece(piece, r as u16, c as u16, color, PaintType::Temporary)?;
                }
            }
            // TODO: Check game tick here
        }
    }

    fn new(height: usize, width: usize) -> Self {
        let mut pieces = VecDeque::with_capacity(3);
        for _ in 0..3 {
            pieces.push_back((random_piece(), random_color()));
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

    fn clear_screen(&mut self) -> crossterm::Result<()> {
        let (height, width) = crossterm::terminal::size().expect("Could not get terminal size.");
        for r in 0..height {
            for c in 0..width {
                self.paint(r, c, Color::Black)?;
            }
        }
        Ok(())
    }

    fn queue_clear_piece(&mut self) -> crossterm::Result<()> {
        for Point(row, column) in self.temp.clone().iter() {
            self.paint(
                row.clone().try_into().unwrap(),
                column.clone().try_into().unwrap(),
                Color::Black,
            )?;
        }
        Ok(())
    }

    fn paint_piece(
        &mut self,
        piece: Piece,
        row: u16,
        column: u16,
        color: Color,
        paint_type: PaintType,
    ) -> crossterm::Result<()> {
        let next_piece = get_piece(piece);

        for r in 0..next_piece.len() {
            for c in 0..next_piece[0].len() {
                if next_piece[r][c] == 1 && (row + r as u16) != 0 {
                    self.paint(row + r as u16, column + c as u16, color)?;
                    match paint_type {
                        PaintType::Temporary => {
                            self.temp.push(Point(row + r as u16, column + c as u16));
                        }
                        _ => {}
                    };
                }
            }
        }
        self.stdout.flush()?;
        Ok(())
    }

    fn paint_next_piece(&mut self) -> crossterm::Result<()> {
        for i in 0..self.pieces.len() {
            self.paint_piece(
                self.pieces[i].0.clone(),
                2 + (i * 4) as u16,
                14,
                self.pieces[i].1,
                PaintType::Permanent,
            )?;
        }
        Ok(())
    }

    fn clear_next_piece(&mut self) -> crossterm::Result<()> {
        const GAME_WIDTH: usize = 12;
        const INFO_PADDING: usize = 1;
        const EMPTY_PIECE_COLUMN: usize = 1;
        const INFO_WIDTH: usize = 4;
        const EMPTY_TOP_INFO_ROWS: usize = 2;
        const INFO_HEIGHT: usize = 15; // TODO: Determine if this is the right number
        let r_start = EMPTY_TOP_INFO_ROWS;
        let r_end = r_start + INFO_HEIGHT;
        let c_start = GAME_WIDTH + INFO_PADDING + EMPTY_PIECE_COLUMN;
        let c_end = c_start + INFO_WIDTH;
        for r in r_start..r_end {
            for c in c_start..c_end {
                self.paint(r as u16, c as u16, Color::Black)?;
            }
        }
        self.stdout.flush()?;
        Ok(())
    }

    fn paint(&mut self, row: u16, column: u16, color: Color) -> crossterm::Result<()> {
        let (width, height) =
            crossterm::terminal::size().expect("Could not get terminal dimensions.");
        const TOTAL_WIDTH: u16 = 36;
        const TOTAL_HEIGHT: u16 = 22;

        let mut width_multiplier = 1;
        while width_multiplier * 2 + width_multiplier <= width / TOTAL_WIDTH {
            width_multiplier += 1;
        }
        let height_multiplier = height / TOTAL_HEIGHT;

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
                        ))?;
                }
            }
        } else {
            for y in (row * info_multiplier)..((row + 1) * info_multiplier) {
                for x in (column * info_multiplier * COLUMN_MULTIPLIER)
                    ..((column + 1) * info_multiplier * COLUMN_MULTIPLIER)
                {
                    self.stdout
                        .queue(cursor::MoveTo(x + GAME_WIDTH * game_multiplier, y))?
                        .queue(style::PrintStyledContent(
                            crossterm::style::style("█").with(color),
                        ))?;
                }
            }
        }

        Ok(())
    }

    fn paint_game_border(&mut self) -> crossterm::Result<()> {
        // TODO: Use constants for all these magic numbers
        for r in 0..21 {
            for c in 0..1 {
                self.paint(r, c, Color::Grey)?;
            }
            for c in 11..12 {
                self.paint(r, c, Color::Grey)?;
            }
        }
        for r in 21..=21 {
            for c in 0..12 {
                self.paint(r, c, Color::Grey)?;
            }
        }
        for r in 0..=0 {
            for c in 0..12 {
                self.paint(r, c, Color::Grey)?;
            }
        }

        Ok(())
    }

    fn init(&mut self) -> crossterm::Result<()> {
        crossterm::style::SetBackgroundColor(crossterm::style::Color::Black);
        crossterm::terminal::enable_raw_mode()?;
        self.stdout.queue(cursor::Hide)?;
        self.clear_screen()?;
        self.paint_game_border()?;
        self.paint_next_piece()?;
        self.stdout.flush()?;

        Ok(())
    }
}

fn main() {
    println!("Starting Tetrominos!");
    let (height, width) = crossterm::terminal::size().expect("Tetrominos crashed");
    // TODO: Make a height and width check and crash if terminal isnt large enough?

    let mut app = App::new(height as usize, width as usize);
    app.init();

    app.run();
}
