use board::Board;
use definitions::Definitions;

mod bitboards;
mod board;
mod definitions;
mod hashkeys;

const TEST_FEN: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

fn main() {
  let mut definitions: Definitions = definitions::Definitions::new();
  definitions.init();
  let mut board: Board = board::Board::new(&definitions);

  board.parse_fen(TEST_FEN);
  board.print_board();
}
