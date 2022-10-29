use crate::definitions::*;
use crate::file_rank_to_square_120;

/**
 * The bit board printed disposition follow this structure (as we play with white):
 *
 *    A B C D E F G H
 * 8  - - - - - - - -
 * 7  - - - - - - - -
 * 6  - - - - - - - -
 * 5  - - - - - - - -
 * 4  - - - - - - - -
 * 3  - - - - - - - -
 * 2  - - - - - - - -
 * 1  - - - - - - - -
 *
 */
pub fn print_bit_board(definitions: &mut Definitions, bit_board: u64) {
  let shift_me: u64 = 1u64;

  let mut square_120: i32;
  let mut square_64: i32;

  println!();
  for rank in (ChessboardRanks::R1 as i32..=ChessboardRanks::R8 as i32).rev() {
    for file in ChessboardFiles::A as i32..=ChessboardFiles::H as i32 {
      square_120 = file_rank_to_square_120!(file, rank);
      square_64 = definitions.board_120_squares_in_64_squares_notation()[square_120 as usize];
      // bitwise operations explanation:
      // shift_me = (H8 last bit) 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000001 (A1 first bit)
      // square_64 = (for example) 28 (E4)
      // shift_me << square_64 = (H8 last bit) 00000000 00000000 00000000 00000000 00001000 00000000 00000000 00000000 (A1 first bit)
      // if bit_board has a bit in that position (& operator), an X is printed, instead a dash (-) is printed
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
