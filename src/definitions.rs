pub const PROGRAM_NAME: &str = "Rust Chess Engine";
pub const BOARD_SQUARE_NUMBER: usize = 120;

// A square can be empty or contain a Wn (White kNight) chess piece for example
pub enum SquareStatus {
  Empty, Wp, Wn, Wb, Wr, Wq, Wk, Bp, Bn, Bb, Br, Bq, Bk,
}

pub enum ChessboardFiles {
  A, B, C, D, E, F, G, H, None,
}

// R stands for "Rank"
pub enum ChessboardRanks {
  R1, R2, R3, R4, R5, R6, R7, R8, None
}

pub enum Colors {
  White, Black, Both
}

pub enum Squares {
  A1 = 21, B1, C1, D1, E1, F1, G1, H1,
  A2 = 31, B2, C2, D2, E2, F2, G2, H2,
  A3 = 41, B3, C3, D3, E3, F3, G3, H3,
  A4 = 51, B4, C4, D4, E4, F4, G4, H4,
  A5 = 61, B5, C5, D5, E5, F5, G5, H5,
  A6 = 71, B6, C6, D6, E6, F6, G6, H6,
  A7 = 81, B7, C7, D7, E7, F7, G7, H7,
  A8 = 91, B8, C8, D8, E8, F8, G8, H8, NoSquare
}
