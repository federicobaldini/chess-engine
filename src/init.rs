use crate::definitions::*;
use crate::file_rank_to_square_120;
use rand::Rng;

/**
 * The board with 120 squares is the board reference for the search engine.
 *
 * We need to have the 120 squares board and the 64 squares board as follow for these reasons:
 *
 * When we have a 64 squares bitboard we can have for example a bit on square B2.
 *
 * The B2 square has position at index 9 in the array board with 64 squares in 64 squares notation.
 * (reference: "board_64_squares_in_64_squares_notation" in definitions.rs file).
 *
 * Because we need to have that position on the 120 squares board,
 *
 * the index 9 not correspond to B2 square, it stays at index 32 in the 120 squares notation.
 * (reference: "board_120_squares_in_120_squares_notation" in definitions.rs file).
 *
 * So if we have a board of 64 squares in 120 squares notation, at index 9 we got 32, then
 * the 32 as index on the 120 squares board in 64 squares notation it's the B2 square and contains the 9 value,
 * the square that originally we wanted arrive to on the 120 squares board.
 *
 * The value that we got can help us to reverse the process and get the B2 square on 64 squares board,
 * that is the square at index 9.
 *
 * Final structure of definitions.board_64_squares_in_120_squares_notation (as we play with black):
 *
 *     A  B  C  D  E  F  G  H
 * 1  21 22 23 24 25 26 27 28
 * 2  31 32 33 34 35 36 37 38
 * 3  41 42 43 44 45 46 47 48
 * 4  51 52 53 54 55 56 57 58
 * 5  61 62 63 64 65 66 67 68
 * 6  71 72 73 74 75 76 77 78
 * 7  81 82 83 84 85 86 87 88
 * 8  91 92 93 94 95 96 97 98
 *
 * Final structure of definitions.board_120_squares_in_64_squares_notation (as we play with black):
 *        A  B  C  D  E  F  G  H
 *    65 65 65 65 65 65 65 65 65 65
 *    65 65 65 65 65 65 65 65 65 65
 * 1  65 00 01 02 03 04 05 06 07 65
 * 2  65 08 09 10 11 12 13 14 15 65
 * 3  65 16 17 18 19 20 21 22 23 65
 * 4  65 24 25 26 27 28 29 30 31 65
 * 5  65 32 33 34 35 36 37 38 39 65
 * 6  65 40 41 42 43 44 45 46 47 65
 * 7  65 48 49 50 51 52 53 54 55 65
 * 8  65 56 57 58 59 60 61 62 63 65
 *    65 65 65 65 65 65 65 65 65 65
 *    65 65 65 65 65 65 65 65 65 65
 */
fn init_squares(definitions: &mut Definitions) {
  let mut square_120: i32;
  let mut square_64: i32 = 0;

  definitions.board_120_squares_in_64_squares_notation()[0..BOARD_SQUARE_NUMBER].fill(65);
  definitions.board_64_squares_in_120_squares_notation()[0..64].fill(120);

  for rank in ChessboardRanks::R1 as i32..=ChessboardRanks::R8 as i32 {
    for file in ChessboardFiles::A as i32..=ChessboardFiles::H as i32 {
      square_120 = file_rank_to_square_120!(file, rank);
      definitions.board_64_squares_in_120_squares_notation()[square_64 as usize] = square_120;
      definitions.board_120_squares_in_64_squares_notation()[square_120 as usize] = square_64;
      square_64 += 1;
    }
  }
}

fn init_masks(definitions: &mut Definitions) {
  for index in 0..64 {
    definitions.bit_mask_to_set_bit_inside_bitboard()[index as usize] |= 1u64 << index;
    // It is the bitwise complement of the "bit_mask_to_set_bit_inside_bitboard"
    definitions.bit_mask_to_clear_bit_inside_bitboard()[index as usize] =
      !definitions.bit_mask_to_set_bit_inside_bitboard()[index as usize];
  }
}

fn generate_random_chess_piece_hash() -> u64 {
  let mut rng = rand::thread_rng();
  rng.gen::<u64>()
}

fn init_hash_keys(definitions: &mut Definitions) {
  for index_1 in 0..13 {
    for index_2 in 0..120 {
      definitions.piece_keys()[index_1][index_2] = generate_random_chess_piece_hash();
    }
  }
  *definitions.side_key() = generate_random_chess_piece_hash();
  for index in 0..16 {
    definitions.castle_keys()[index] = generate_random_chess_piece_hash();
  }
}

pub fn init(definitions: &mut Definitions) {
  init_squares(definitions);
  init_masks(definitions);
  init_hash_keys(definitions)
}
