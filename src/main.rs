use board::Board;
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

enum PaintType {
    Permanent,
    Temporary,
}

#[derive(Clone)]
struct Point(u16, u16);

struct App {
    board: Board,
    pieces: VecDeque<Piece>,
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
    fn nextPiece() {}
    fn completedRow() {}

    fn gravityTick() {}

    fn run(&mut self) -> crossterm::Result<()> {
        // Use gravityTick game loop here
        crossterm::terminal::enable_raw_mode()?;

        let mut piece = random_piece();
        let mut now = std::time::Instant::now();
        let mut r: i16 = 0;
        let mut c: i16 = 4;

        self.paintPiece(piece, r as u16, c as u16, 0, PaintType::Temporary)?;
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

                            _ => {}
                        }
                        self.queue_clear_piece();
                        self.paintPiece(piece, r as u16, c as u16, 0, PaintType::Temporary)?;
                    }
                    Event::Resize(width, height) => {
                        // Clear everything and write everything else back
                        println!("New size {}x{}", width, height);

                        self.clear();
                        // TODO: Repaint everything back to the screen
                    }
                    _ => {}
                }
            }
            if now.elapsed().as_millis() > 500 {
                now = std::time::Instant::now();
                if self.board.detect_collision(piece, r + 1, c) {
                    println!("collisioin  detected!");
                    // TODO: If intersection, this hsould be permanent
                    self.temp.clear();
                    // TODO: Pop the next piece!
                    // TODO: Check for completed lines
                    //      Increment lines by completed lines
                    r = 0;
                    c = 4;
                    self.paintPiece(piece, r as u16, c as u16, 0, PaintType::Temporary)?;
                } else {
                    r += 1;
                    self.queue_clear_piece();
                    self.temp.clear();
                    self.paintPiece(piece, r as u16, c as u16, 0, PaintType::Temporary)?;
                }
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
        let (height, width) = crossterm::terminal::size().expect("Tetrominos crashed");
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
            self.paint(
                row.clone().try_into().unwrap(),
                column.clone().try_into().unwrap(),
                Color::Black,
            )?;
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
        let next_piece = get_piece(piece);

        for r in 0..next_piece.len() {
            for c in 0..next_piece[0].len() {
                if next_piece[r][c] == 1 && (row + r as u16) != 0 {
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
        let (width, height) = crossterm::terminal::size().expect("Tetrominos crashed");
        //let factor = std::cmp::min(height / 21, width / 36);
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
                        ));
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
    println!("Starting Tetrominos!");
    // TODO: Whenever there is a resize, i need to repaint everything

    // I should be getting height and width from the current terminal
    //let height = 1280;
    //let width = 640;

    // First thing to do is to make the whole screen black
    let (height, width) = crossterm::terminal::size().expect("Tetrominos crashed");
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
    for r in 0..=0 {
        for c in 0..12 {
            app.paint(r, c, Color::Grey);
        }
    }
    app.paintNextPiece();
    app.stdout.flush();

    app.run();
}
