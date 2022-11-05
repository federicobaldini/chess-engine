use crate::definitions::*;

pub fn reset_board(definitions: &mut Definitions, board: &mut Board) {
  for index in 0..BOARD_SQUARE_NUMBER {
    board.pieces()[index as usize] = Squares::OffBoard as i32;
  }
  for index in 0..64 {
    board.pieces()[definitions.board_120_squares_in_64_squares_notation()[index] as usize] =
      Pieces::Empty as i32;
  }
  for index in 0..3 {
    board.big_pieces_number()[index] = 0;
    board.major_pieces_number()[index] = 0;
    board.minor_pieces_number()[index] = 0;
    board.pawns()[index] = 0u64;
  }
}
