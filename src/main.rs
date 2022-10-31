mod bitboards;
mod definitions;
mod init;

fn main() {
  let mut definitions = definitions::Definitions::new();
  let mut test_bit_board: u64 = 0u64;

  init::init(&mut definitions);

  test_bit_board |= 1u64
    << definitions.board_120_squares_in_64_squares_notation()[(definitions::Squares::D2) as usize];
  test_bit_board |= 1u64
    << definitions.board_120_squares_in_64_squares_notation()[(definitions::Squares::D3) as usize];
  test_bit_board |= 1u64
    << definitions.board_120_squares_in_64_squares_notation()[(definitions::Squares::D4) as usize];

  bitboards::print_bit_board(&mut definitions, test_bit_board);

  println!("Count: {}", bitboards::count_bits(test_bit_board));
  println!("Index: {}", bitboards::pop_first_bit(&mut test_bit_board));

  bitboards::print_bit_board(&mut definitions, test_bit_board);
}
