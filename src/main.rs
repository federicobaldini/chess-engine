mod bitboards;
mod definitions;
mod init;

fn main() {
  let mut definitions = definitions::Definitions::new();
  let mut test_bitboard: u64 = 0u64;

  init::init(&mut definitions);

  /*
  let set_mask = definitions.bit_mask_to_set_bit_inside_bitboard().clone();
  let clear_mask = definitions.bit_mask_to_clear_bits_inside_bitboard().clone();

  for index in 0..64 {
    println!("Index: {}", index);
    bitboards::print_bitboard(&mut definitions, set_mask[index as usize]);
    println!();
  }

  for index in 0..64 {
    println!("Index: {}", index);
    bitboards::print_bitboard(&mut definitions, clear_mask[index as usize]);
    println!();
  }
  */

  bitboards::set_bit_to_bitboard(&mut definitions, &mut test_bitboard, 61);
  bitboards::print_bitboard(&mut definitions, test_bitboard);

  bitboards::clear_bit_to_bitboard(&mut definitions, &mut test_bitboard, 61);
  bitboards::print_bitboard(&mut definitions, test_bitboard);
}
