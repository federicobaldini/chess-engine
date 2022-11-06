use crate::definitions::*;

fn generate_position_key(definitions: &mut Definitions, board: &mut Board) -> u64 {
  let mut final_key: u64 = 0;
  let mut piece: Pieces;

  for square_120 in 0..BOARD_SQUARE_NUMBER {
    piece = Pieces::from_u32(board.pieces()[square_120 as usize] as u32);
    if piece as i32 != Squares::NoSquare as i32 && piece as i32 != Pieces::Empty as i32 {
      final_key ^= definitions.piece_keys()[piece as usize][square_120 as usize];
    }
  }

  if *board.side() as i32 == Colors::White as i32 {
    final_key ^= *definitions.side_key();
  }

  if *board.en_passant_square() as i32 != Squares::NoSquare as i32 {
    final_key ^=
      definitions.piece_keys()[Pieces::Empty as usize][*board.en_passant_square() as usize];
  }

  final_key ^= definitions.castle_keys()[*board.castel_permission() as usize];

  final_key
}
