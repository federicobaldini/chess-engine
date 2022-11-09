mod bitboards;
mod board;
mod definitions;
mod hashkeys;

const FEN1: &str = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
const FEN2: &str = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2";
const FEN3: &str = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";

fn main() {
  let mut definitions = definitions::Definitions::new();
  let mut board = board::Board::new();

  definitions.init();
  //board.parse_fen(definitions, definitions::START_FEN);
  //board.print_board(definitions);
  board.parse_fen(definitions, FEN1);
  board.print_board(definitions);
  board.parse_fen(definitions, FEN2);
  board.print_board(definitions);
  board.parse_fen(definitions, FEN3);
  board.print_board(definitions);
}
