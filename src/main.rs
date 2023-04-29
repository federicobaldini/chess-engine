use board::Board;
use definitions::{Definitions, Move, Pieces};

mod attack;
mod bitboards;
mod board;
mod definitions;
mod hashkeys;

const TEST_FEN_1: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

fn print_binary(mov_e: i32) {
  println!("As binary:");
  for index in (0..=27).rev() {
    if ((1 << index) & mov_e) != 0 {
      print!("1");
    } else {
      print!("0");
    }
    if index != 28 && (index % 4) == 0 {
      print!(" ");
    }
  }
  println!();
}

fn main() {
  let mut definitions: Definitions = definitions::Definitions::new();
  definitions.init();
  let mut board: Board = board::Board::new(&definitions);

  board.parse_fen(TEST_FEN_1);
  // board.check_board();
  board.print_board();

  let from: i32 = 6;
  let to: i32 = 12;
  let captured: Pieces = Pieces::Wr;
  let promoted: Pieces = Pieces::Br;
  let mut temp_move: Move = Move::new(
    (from) | (to << 7) | ((captured as i32) << 14) | ((promoted as i32) << 20),
    0,
  );

  println!("\ndec: {} hex: {:x}", temp_move.mov_e(), temp_move.mov_e());
  print_binary(temp_move.mov_e());

  println!(
    "\nfrom: {} to: {} captured: {} promoted: {}",
    temp_move.from_square(),
    temp_move.to_square(),
    temp_move.captured_piece(),
    temp_move.promoted()
  );

  // temp_move.set_mov_e(temp_move.mov_e() | 0x80000);

  println!(
    "\nIs a pawn start: {}",
    if temp_move.pawn_start() != 0 {
      "YES"
    } else {
      "NO"
    }
  );
}
