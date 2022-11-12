use crate::board::*;
use crate::definitions::*;

pub fn generate_position_key(definitions: Definitions, board: Board) -> u64 {
  let mut final_key: u64 = 0;
  let mut piece: Pieces;

  for square_120 in 0..BOARD_SQUARE_NUMBER {
    if board.pieces()[square_120 as usize] as u32 != Squares::OffBoard as u32 {
      piece = Pieces::from_u32(board.pieces()[square_120 as usize] as u32);
      if piece as u32 != Squares::NoSquare as u32 && piece != Pieces::Empty {
        final_key ^= definitions.piece_keys()[piece as usize][square_120 as usize];
      }
    }
  }

  if board.side() == Colors::White {
    final_key ^= definitions.side_key();
  }

  if board.en_passant_square() != Squares::NoSquare {
    final_key ^=
      definitions.piece_keys()[Pieces::Empty as usize][board.en_passant_square() as usize];
  }

  final_key ^= definitions.castle_keys()[board.castel_permission() as usize];

  final_key
}
