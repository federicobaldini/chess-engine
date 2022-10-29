mod bitboards;
mod definitions;
mod init;

fn main() {
  let mut definitions = definitions::Definitions::new();
  let mut test_bit_board: u64 = 0u64;

  init::init(&mut definitions);

  // bitwise operations
  test_bit_board |= 1u64
    << definitions.board_120_squares()[file_rank_to_square_120!(
      definitions::ChessboardFiles::D as i32,
      definitions::ChessboardRanks::R2 as i32
    ) as usize];
  println!("D2 pawn added to the bitboard!\n");
  bitboards::print_bit_board(&mut definitions, test_bit_board);

  // bitwise operations
  test_bit_board |= 1u64
    << definitions.board_120_squares()[file_rank_to_square_120!(
      definitions::ChessboardFiles::G as i32,
      definitions::ChessboardRanks::R3 as i32
    ) as usize];
  println!("G3 pawn added to the bitboard!\n");
  bitboards::print_bit_board(&mut definitions, test_bit_board);
}
