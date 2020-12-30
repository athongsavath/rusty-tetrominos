use crate::piece::{get_piece, Piece};

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

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
                    // TODO: Fix these hardcoded offsets to account for row and column offset from
                    // gray outline
                    if (row + r as i16 - 1) < 0 || (row + r as i16 - 1) >= (self.height as i16) {
                        return true;
                    } else if (column + c as i16 - 1) < 0
                        || (column + c as i16 - 1) >= (self.width as i16)
                    {
                        return true;
                    } else if self.board[row as usize + r as usize - 1]
                        [column as usize + c as usize - 1]
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
}
