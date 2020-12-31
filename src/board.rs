use crate::piece::{get_piece, Piece};
use crossterm::style::Color;

const WIDTH: usize = 10;
const HEIGHT: usize = 20;
const BORDER_WIDTH: usize = 1;

/// The Board contains all of the pieces that are placed into the pile of tetrominos. The Board
/// keeps track of the locations as well as the colors. If I feel like cleaning this up, I can just
/// use the color_board to keep track of all of the blocks
pub struct Board {
    width: usize,
    height: usize,
    board: [[u8; WIDTH]; HEIGHT],
    color_board: [[Color; WIDTH]; HEIGHT],
}

impl Board {
    /// Generated a new board with all of the default values. The board starts of all black.
    pub fn new() -> Self {
        Self {
            width: WIDTH,
            height: HEIGHT,
            board: [[0u8; WIDTH]; HEIGHT],
            color_board: [[Color::Black; WIDTH]; HEIGHT],
        }
    }

    /// Returns whether or not a collision is detected between the given piece and any of the
    /// elements on the board or if the piece will exceed the boundaries of the board. wrapping_add
    /// is used to prevent the case of i16 conversions of -1 turning into 65535. Ideally, I
    /// would fix this.
    pub fn detect_collision(&self, piece: Piece, row: i16, column: i16) -> bool {
        let matched_piece = get_piece(piece);
        for r in 0..matched_piece.len() {
            for c in 0..matched_piece[0].len() {
                if matched_piece[r][c] == 1 {
                    if (row + r as i16 - BORDER_WIDTH as i16) < 0
                        || (row + r as i16 - BORDER_WIDTH as i16) >= (self.height as i16)
                    {
                        return true;
                    } else if (column + c as i16 - BORDER_WIDTH as i16) < 0
                        || (column.wrapping_add(c as i16) - BORDER_WIDTH as i16)
                            >= (self.width as i16)
                    {
                        return true;
                    } else if self.board[row as usize + r as usize - BORDER_WIDTH]
                        [column.wrapping_add(c as i16) as usize - BORDER_WIDTH]
                        == 1
                    {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Returns whether or not the game has ended
    pub fn detect_endgame(&self, piece: Piece, row: i16, column: i16) -> bool {
        false
    }

    /// Saves the given piece to the board and color_board
    pub fn save(&mut self, piece: Piece, row: i16, column: i16, color: Color) {
        let matched_piece = get_piece(piece);
        for r in 0..matched_piece.len() {
            for c in 0..matched_piece[0].len() {
                if matched_piece[r][c] == 1 {
                    self.board[row as usize + r as usize - BORDER_WIDTH]
                        [column.wrapping_add(c as i16) as usize - BORDER_WIDTH] = 1;
                    self.color_board[row as usize + r as usize - BORDER_WIDTH]
                        [column.wrapping_add(c as i16) as usize - BORDER_WIDTH] = color;
                }
            }
        }
    }

    /// Returns the total number of lines that have been completed.
    ///
    /// Deletes the completed lines and shifts everything down
    pub fn handle_completed_lines(&mut self, row: i16) -> i32 {
        // CONSIDER: row number is opposite of what we want and it has a border added to it
        return 0;
    }
}
