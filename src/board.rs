use crate::definitions::*;

pub fn reset_board(board: &mut Board) {
  for index 0..BOARD_SQUARE_NUMBER {
    board.pieces()[index as usize] = Squares::OffBoard;
  }
  for index 0..64 {
    board.pieces()[definitions.board_120_squares_in_64_squares_notation()[index]] = Pieces::Empty;
  }
}
