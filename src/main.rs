use crate::definitions::Colors;

mod bitboards;
mod board;
mod definitions;
mod hashkeys;

const TEST_FEN: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

fn main() {
  let mut definitions = definitions::Definitions::new();
  let mut board = board::Board::new();

  definitions.init();
  board.parse_fen(&definitions, TEST_FEN);
  board.print_board(&definitions);

  println!("\nWhite Pawns:");
  bitboards::print_bitboard(&definitions, board.pawns()[Colors::White as usize]);
  println!("\nBlack Pawns:");
  bitboards::print_bitboard(&definitions, board.pawns()[Colors::Black as usize]);
  println!("\nAll Pawns:");
  bitboards::print_bitboard(&definitions, board.pawns()[Colors::Both as usize]);
}
