use crate::definitions::*;
use crate::file_rank_to_square_120;

pub fn print_bit_board(definitions: &mut Definitions, bit_board: u64) {
  let shift_me: u64 = 1u64;

  let mut square_120: i32;
  let mut square_64: i32;

  println!();
  for rank in (ChessboardRanks::R1 as i32..=ChessboardRanks::R8 as i32).rev() {
    for file in ChessboardFiles::A as i32..=ChessboardFiles::H as i32 {
      square_120 = file_rank_to_square_120!(file, rank);
      square_64 = definitions.board_120_squares()[square_120 as usize];
      // bitwise operations
      if ((shift_me << square_64) & bit_board) != 0 {
        print!(" X ");
      } else {
        print!(" - ");
      }
    }
    println!();
  }
  println!();
}
