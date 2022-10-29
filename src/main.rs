mod bitboards;
mod definitions;
mod init;

fn main() {
  let mut definitions = definitions::Definitions::new();
  let mut test_bit_board: u64 = 0u64;

  init::init(&mut definitions);

  // bitwise operations ("<<" bit shift, "|=" is like "+=" )
  test_bit_board |= 1u64
    << definitions.board_120_squares_in_64_squares_notation()[(definitions::Squares::D2) as usize];
  println!("D2 pawn added to the bitboard!\n");
  bitboards::print_bit_board(&mut definitions, test_bit_board);

  // bitwise operations ("<<" bit shift, "|=" is like "+=" )
  test_bit_board |= 1u64
    << definitions.board_64_squares_in_120_squares_notation()[(definitions::Squares::G3) as usize];
  println!("G3 pawn added to the bitboard!\n");
  bitboards::print_bit_board(&mut definitions, test_bit_board);
}
