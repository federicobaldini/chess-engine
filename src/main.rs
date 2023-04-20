use attack::show_squares_attacked_by_side;
use board::Board;
use definitions::{Colors, Definitions};

mod attack;
mod bitboards;
mod board;
mod definitions;
mod hashkeys;

const TEST_FEN_1: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
const TEST_FEN_2: &str = "8/3q1p2/8/5P2/4Q3/8/8/8 w - - 0 2";

fn main() {
  let mut definitions: Definitions = definitions::Definitions::new();
  definitions.init();
  let mut board: Board = board::Board::new(&definitions);

  board.parse_fen(TEST_FEN_2);
  // board.check_board();
  board.print_board();

  println!("\n\nWhite attacking:");
  show_squares_attacked_by_side(Colors::White, &board);

  println!("\n\nBlack attacking:");
  show_squares_attacked_by_side(Colors::Black, &board);
}
