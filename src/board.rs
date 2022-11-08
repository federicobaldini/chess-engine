use crate::definitions::*;

pub fn reset_board(definitions: &mut Definitions, board: &mut Board) {
  *board.pieces() = [Squares::OffBoard as i32; BOARD_SQUARE_NUMBER];
  for index in 0..64 {
    board.pieces()[definitions.board_64_squares_in_120_squares_notation()[index] as usize] =
      Pieces::Empty as i32;
  }
  *board.big_pieces_number() = [0; 3];
  *board.major_pieces_number() = [0; 3];
  *board.minor_pieces_number() = [0; 3];
  *board.pawns() = [0u64; 3];
  *board.actual_pieces_number() = [0; 13];
  *board.king_square() = [Squares::NoSquare as i32, 2];
  *board.side() = Colors::Both;
  *board.en_passant_square() = Squares::NoSquare;
  *board.fifty_full_moves() = 0;
  *board.actual_half_moves() = 0;
  *board.total_half_moves() = 0;
  *board.castel_permission() = 0;
  *board.position_key() = 0u64;
}
