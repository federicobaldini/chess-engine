use crate::definitions::*;
use crate::file_rank_to_square_120;

/**
 * References:
 * https://www.chessprogramming.org/Looking_for_Magics;
 * https://stackoverflow.com/questions/30680559/how-to-find-magic-bitboards
 * I will explain this constant if I will use it later.
 */
const BIT_TABLE: [i32; 64] = [
  63, 30, 3, 32, 25, 41, 22, 33, 15, 50, 42, 13, 11, 53, 19, 34, 61, 29, 2, 51, 21, 43, 45, 10, 18,
  47, 1, 54, 9, 57, 0, 35, 62, 31, 40, 4, 49, 5, 52, 26, 60, 6, 23, 44, 46, 27, 56, 16, 7, 39, 48,
  24, 59, 14, 12, 55, 38, 28, 58, 20, 37, 17, 36, 8,
];

/**
 * The bitboard printed disposition follow this structure (as we play with white):
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
pub fn print_bitboard(definitions: &Definitions, bitboard: u64) {
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
      // if bitboard has a bit in that position (& operator), an X is printed, instead a dash (-) is printed
      if ((shift_me << square_64) & bitboard) != 0 {
        print!(" X ");
      } else {
        print!(" - ");
      }
    }
    println!();
  }
  println!();
}

/**
 * We get a 64 bit unsigned integer as it overflowed the 32 bit size.
 * For this scope it's necessary just invert and add one from the 64 bit type.
 * This is necessary because Rust do not allow to overflow a type size as C does.
 */
fn get_overflow_as_32_bit(number: u64) -> u32 {
  (number % (u32::max_value() as u64 + 1)) as u32
}

/**
 * It takes the first bit starting at the least significant bit in a bitboard and
 * return the index that this bit was set at, so we know which square that bit was set on.
 * Then that bit is set to zero.
 * References:
 * https://www.chessprogramming.org/Looking_for_Magics;
 * https://stackoverflow.com/questions/30680559/how-to-find-magic-bitboards
 * I will explain this function line by line if I will use it later.
 */
pub fn pop_first_bit(bitboard: &mut u64) -> i32 {
  let b: u64 = *bitboard ^ (*bitboard - 1);
  let fold: u64 = ((b & 0xffffffff) ^ (b >> 32)) as u64;
  *bitboard &= *bitboard - 1;
  BIT_TABLE[(get_overflow_as_32_bit(fold * 0x783a9b23) >> 26) as usize]
}

/**
 * I will explain this function line by line if I will use it later.
 */
pub fn count_bits(mut bitboard: u64) -> u64 {
  let mut counter: u64 = 0;
  while counter < bitboard {
    bitboard &= bitboard - 1;
    counter += 1;
  }
  counter
}

pub fn set_bit_to_bitboard(definitions: &Definitions, bitboard: &mut u64, square_64: i32) {
  *bitboard |= definitions.bit_mask_to_set_bit_inside_bitboard()[square_64 as usize];
}

pub fn clear_bit_to_bitboard(definitions: &Definitions, bitboard: &mut u64, square_64: i32) {
  *bitboard &= definitions.bit_mask_to_clear_bit_inside_bitboard()[square_64 as usize];
}
