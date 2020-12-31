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

const TOTAL_WIDTH: u16 = 36;

const GAME_WIDTH: u16 = 12;

const GAME_BORDER_WIDTH: u16 = 1;
const INFO_PADDING: u16 = 1;
const INFO_WIDTH: u16 = 4;

const TOTAL_HEIGHT: u16 = 22;

const EMPTY_PIECE_COLUMN: u16 = 1;
const EMPTY_TOP_INFO_ROWS: u16 = 2;
const INFO_HEIGHT: u16 = 15; // TODO: Determine if this is the right number

const PIECE_HEIGHT: u16 = 4;

/*
 * Width consists of
 * 1 border = GAME_BORDER_WIDTH
 * 10 game
 * 1 border = GAME_BORDER_WIDTH
 * 1 padding = INFO_PADDING
 * 4 info box = INFO_WIDTH (piece width)
 * 1 padding = INFO_PADDING
 */

/*
 * Game Height = TOTAL_HEIGHT
 * 1 border = GAME_BORDER_WIDTH
 * 20 game
 * 1 border = GAME_BORDER_WIDTH
 */

/*
 * Info Height
 * 2 padding = EMPTY_TOP_INFO_ROWS
 * 4 piece = PIECE_HEIGHT
 * 4 piece = PIECE_HEIGHT
 * 4 piece = PIECE_HEIGHT
 * 1 padding = INFO_PADDING
 * REST: TODO: RANDOM INFO STUFF?
 */
const STARTING_ROW: i16 = 0;
const STARTING_COLUMN: i16 = 4;

#[derive(Clone)]
struct Point(u16, u16);

struct App {
    board: Board,
    pieces: VecDeque<(Piece, Color)>,
    temp: Vec<Point>,
    score: i32,
    lines: i32,
    level: i32,
    stdout: Stdout,

    now: std::time::Instant,
    piece: Piece,
    r: i16,
    c: i16,
    color: Color,
}

impl App {
    /// Returns the next piece out of the deque and replaces it with a new piece
    fn next_piece(&mut self) -> (Piece, Color) {
        let sol = self.pieces.pop_front();
        self.pieces.push_back((random_piece(), random_color()));
        sol.unwrap()
    }

    /// Gets the players move and updates the board accordingly
    ///
    /// For rotations, the piece moves to the next valid rotation, This means that if no valid
    /// rotations are found, the piece will just rotate back into it's original orientation.
    fn update_player_move(&mut self) -> crossterm::Result<()> {
        if poll(Duration::from_millis(25))? {
            match read()? {
                Event::Key(event) => {
                    match match_key(event.code) {
                        Command::Left => {
                            if !self.board.detect_collision(self.piece, self.r, self.c - 1) {
                                self.c -= 1;
                            }
                        }
                        Command::Right => {
                            if !self.board.detect_collision(self.piece, self.r, self.c + 1) {
                                self.c += 1;
                            }
                        }
                        Command::Down => {
                            if !self.board.detect_collision(self.piece, self.r + 1, self.c) {
                                self.r += 1;
                            }
                        }
                        Command::Up => loop {
                            self.piece = rotate(self.piece);
                            if !self.board.detect_collision(self.piece, self.r, self.c) {
                                break;
                            }
                        },
                        Command::Space => {
                            // Places the piece onto the board
                            loop {
                                if self.board.detect_collision(self.piece, self.r + 1, self.c) {
                                    break;
                                }
                                self.r += 1;
                            }
                            // Ensure that enough time elapsed to make this piece permanent
                            self.now -= std::time::Duration::new(5, 0);
                        }
                        Command::Escape => {
                            self.clear_screen()?;
                            println!("Aborting");
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                    self.queue_clear_piece()?;
                    self.paint_piece(
                        self.piece,
                        self.r as u16,
                        self.c as u16,
                        self.color,
                        PaintType::Temporary,
                    )?;
                }
                Event::Resize(_, _) => {
                    self.init()?;
                    self.paint_piece(
                        self.piece,
                        self.r as u16,
                        self.c as u16,
                        self.color,
                        PaintType::Temporary,
                    )?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Either moves pieces down due to gravity or affixes piece to the board since, it can no
    /// longer move downwards. In the latter case, the next piece is setup for the next game loop.
    fn gravity_tick(&mut self) -> crossterm::Result<()> {
        self.now = std::time::Instant::now();

        if self.board.detect_collision(self.piece, self.r + 1, self.c) {
            // Fix piece to board
            self.temp.clear();

            let new_lines = self.board.handle_completed_lines(self.r);
            if new_lines > 0 {
                self.paint_board(self.r as u16)?;
                self.lines += new_lines;
            }

            self.paint_piece(
                self.piece,
                self.r as u16,
                self.c as u16,
                self.color,
                PaintType::Permanent,
            )?;

            // Check to see if game ended
            if self.board.detect_endgame(self.piece, self.r, self.c) {
                self.clear_screen()?;
                println!("GAME OVER!");
                println!("You cleared {} lines", self.lines);
                std::process::exit(0);
            }

            self.board.save(self.piece, self.r, self.c, self.color);

            // Setup for next piece
            self.r = STARTING_ROW;
            self.c = STARTING_COLUMN;
            let (new_piece, new_color) = self.next_piece();
            self.clear_next_piece()?;
            self.paint_next_piece()?;
            self.piece = new_piece;
            self.color = new_color;
            self.paint_piece(
                self.piece,
                self.r as u16,
                self.c as u16,
                self.color,
                PaintType::Temporary,
            )?;
            self.stdout.flush()?;
        } else {
            // Gravity
            self.r += 1;
            self.queue_clear_piece()?;
            self.temp.clear();
            self.paint_piece(
                self.piece,
                self.r as u16,
                self.c as u16,
                self.color,
                PaintType::Temporary,
            )?;
        }
        Ok(())
    }

    /// Repaints the board after a completed row has been deleted
    fn paint_board(&mut self, row: u16) -> crossterm::Result<()> {
        let mut color;
        for r in 0..=(row as usize) {
            for c in 0..self.board.width {
                color = self.board.color_board[r][c];
                self.paint(
                    r as u16 + GAME_BORDER_WIDTH,
                    c as u16 + GAME_BORDER_WIDTH,
                    color,
                )?;
            }
        }

        Ok(())
    }

    /// Runs the program
    ///
    /// First the initial piece is setup, then the event loop, which looks for a player move and
    /// gravity ticks
    fn run(&mut self) -> crossterm::Result<()> {
        let (piece, color) = self.next_piece();
        self.now = std::time::Instant::now();
        self.piece = piece;
        self.color = color;

        self.paint_piece(
            self.piece,
            self.r as u16,
            self.c as u16,
            self.color,
            PaintType::Temporary,
        )?;
        loop {
            self.update_player_move()?;

            if self.now.elapsed().as_millis() > 500 {
                self.gravity_tick()?;
            }
        }
    }

    /// Initializes an App struct
    fn new() -> Self {
        let mut pieces = VecDeque::with_capacity(3);
        for _ in 0..3 {
            pieces.push_back((random_piece(), random_color()));
        }

        Self {
            board: Board::new(),
            level: 0,
            score: 0,
            lines: 0,
            stdout: stdout(),
            pieces,
            temp: vec![],

            piece: random_piece(),
            now: std::time::Instant::now(),
            r: STARTING_ROW,
            c: STARTING_COLUMN,
            color: Color::Black,
        }
    }

    /// Clears the whole terminal screen
    fn clear_screen(&mut self) -> crossterm::Result<()> {
        let (height, width) = crossterm::terminal::size().expect("Could not get terminal size.");
        for r in 0..height {
            for c in 0..width {
                self.paint(r, c, Color::Black)?;
            }
        }
        Ok(())
    }

    /// Queues a piece to be cleared. These values come from the Temporary painted pieces
    fn queue_clear_piece(&mut self) -> crossterm::Result<()> {
        for Point(row, column) in self.temp.clone().iter() {
            self.paint(*row, *column, Color::Black)?;
        }
        Ok(())
    }

    /// Paints a piece to the board
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
                    self.paint(row + r as u16, column.wrapping_add(c as u16), color)?;
                    match paint_type {
                        PaintType::Temporary => {
                            self.temp
                                .push(Point(row + r as u16, column.wrapping_add(c as u16)));
                        }
                        _ => {}
                    };
                }
            }
        }
        self.stdout.flush()?;
        Ok(())
    }

    /// Paints the next pieces on the info pane
    fn paint_next_piece(&mut self) -> crossterm::Result<()> {
        let column = GAME_WIDTH + INFO_PADDING;
        let row = EMPTY_TOP_INFO_ROWS;

        for i in 0..self.pieces.len() {
            let (piece, color) = self.pieces[i];

            self.paint_piece(
                piece,
                row + (i as u16 * PIECE_HEIGHT),
                column,
                color,
                PaintType::Permanent,
            )?;
        }
        Ok(())
    }

    /// Clears all of the pieces on the info pane visually, not physically
    fn clear_next_piece(&mut self) -> crossterm::Result<()> {
        let r_start = EMPTY_TOP_INFO_ROWS;
        let r_end = r_start + INFO_HEIGHT;
        let c_start = GAME_WIDTH + INFO_PADDING;
        let c_end = c_start + INFO_WIDTH;

        for r in r_start..r_end {
            for c in c_start..c_end {
                self.paint(r as u16, c as u16, Color::Black)?;
            }
        }
        self.stdout.flush()?;
        Ok(())
    }

    /// Paints pixels to the screen.
    ///
    /// Since the screen has a ratio of 2:1, it's necessary to paint 2 characters to get a square
    /// pixel
    fn paint(&mut self, row: u16, column: u16, color: Color) -> crossterm::Result<()> {
        let (width, height) =
            crossterm::terminal::size().expect("Could not get terminal dimensions.");

        // Keep a 2:1 ratio between the game section and the info section
        let mut width_multiplier = 1;
        while width_multiplier * 2 + width_multiplier <= width / TOTAL_WIDTH {
            width_multiplier += 1;
        }
        //let width_multiplier = std::cmp::max(width / TOTAL_WIDTH / 3, 1);
        let height_multiplier = height / TOTAL_HEIGHT;

        let game_multiplier = std::cmp::min(width_multiplier, height_multiplier);
        let info_multiplier = std::cmp::max(game_multiplier / 2, 1);

        // Since terminal characters are a 2:1 height:width ratio,
        //  2 characters will be used to create a square pixel
        const COLUMN_MULTIPLIER: u16 = 2;

        if column < GAME_WIDTH as u16 {
            // Game Section
            let x_start = column * game_multiplier * COLUMN_MULTIPLIER;
            let x_end = x_start + game_multiplier * COLUMN_MULTIPLIER;
            let y_start = row * game_multiplier;
            let y_end = y_start + game_multiplier;

            for x in x_start..x_end {
                for y in y_start..y_end {
                    self.stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::PrintStyledContent(
                            crossterm::style::style("█").with(color),
                        ))?;
                }
            }
        } else {
            // Info Section
            let adjusted_column = column - GAME_WIDTH + GAME_WIDTH * game_multiplier;
            let x_start = adjusted_column * info_multiplier * COLUMN_MULTIPLIER;
            let x_end = x_start + info_multiplier * COLUMN_MULTIPLIER;
            let y_start = row * info_multiplier;
            let y_end = y_start + info_multiplier;

            for x in x_start..x_end {
                for y in y_start..y_end {
                    self.stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::PrintStyledContent(
                            crossterm::style::style("█").with(color),
                        ))?;
                }
            }
        }

        Ok(())
    }

    /// Paints the grey game border
    fn paint_game_border(&mut self) -> crossterm::Result<()> {
        // Paint left and right borders of game box
        for r in 0..TOTAL_HEIGHT {
            for c in 0..GAME_BORDER_WIDTH {
                self.paint(r, c, Color::Grey)?;
            }
            for c in (GAME_WIDTH - GAME_BORDER_WIDTH)..GAME_WIDTH {
                self.paint(r, c, Color::Grey)?;
            }
        }

        // Paint bottom border of game box
        for c in 0..GAME_WIDTH {
            self.paint(TOTAL_HEIGHT - GAME_BORDER_WIDTH, c, Color::Grey)?;
        }

        // Paint top border of game box
        for c in 0..12 {
            self.paint(0, c, Color::Grey)?;
        }

        Ok(())
    }

    /// Paints all of the things necessary for the board game to the screen
    fn init(&mut self) -> crossterm::Result<()> {
        crossterm::style::SetBackgroundColor(crossterm::style::Color::Black);
        crossterm::terminal::enable_raw_mode()?;
        self.stdout.queue(cursor::Hide)?;
        self.clear_screen()?;
        self.paint_game_border()?;
        self.paint_next_piece()?;
        self.paint_board(self.board.height as u16 - 1)?;
        self.stdout.flush()?;

        Ok(())
    }
}

/// Starts the game
fn main() {
    let mut app = App::new();
    app.init();
    app.run();
}
