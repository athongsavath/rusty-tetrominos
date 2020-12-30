use crate::piece::{get_piece, Piece};

const WIDTH: usize = 10;
const HEIGHT: usize = 20;
const BORDER_WIDTH: usize = 1;

pub struct Board {
    width: usize,
    height: usize,
    board: [[u8; WIDTH]; HEIGHT],
}

impl Board {
    pub fn new() -> Self {
        Self {
            width: WIDTH,
            height: HEIGHT,
            board: [[0u8; WIDTH]; HEIGHT],
        }
    }

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
                        || (column + c as i16 - BORDER_WIDTH as i16) >= (self.width as i16)
                    {
                        return true;
                    } else if self.board[row as usize + r as usize - BORDER_WIDTH]
                        [column as usize + c as usize - BORDER_WIDTH]
                        == 1
                    {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn detect_endgame(&self, piece: Piece, row: i16, column: i16) -> bool {
        false
    }

    pub fn save(&mut self, piece: Piece, row: i16, column: i16, color: u8) {
        let matched_piece = get_piece(piece);
        for r in 0..matched_piece.len() {
            for c in 0..matched_piece[0].len() {
                if matched_piece[r][c] == 1 {
                    self.board[row as usize + r as usize - BORDER_WIDTH]
                        [column as usize + c as usize - BORDER_WIDTH] = 1;
                }
            }
        }
    }
}
