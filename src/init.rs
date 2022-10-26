use crate::definitions::*;
use crate::file_rank_to_square_120;

fn init_squares(definitions: &mut Definitions) {
  let mut square_120: i32;
  let mut square_64: i32 = 0;

  definitions.board_120_squares()[0..BOARD_SQUARE_NUMBER].fill(65);
  definitions.board_64_squares()[0..64].fill(120);

  for rank in ChessboardRanks::R1 as i32..=ChessboardRanks::R8 as i32 {
    for file in ChessboardFiles::A as i32..=ChessboardFiles::H as i32 {
      square_120 = file_rank_to_square_120!(file, rank);
      definitions.board_64_squares()[square_64 as usize] = square_120;
      definitions.board_120_squares()[square_120 as usize] = square_64;
      square_64 += 1;
    }
  }
}

pub fn init(definitions: &mut Definitions) {
  init_squares(definitions);
}
