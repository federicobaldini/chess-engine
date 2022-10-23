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

pub struct Board {
  pieces: [u8; BOARD_SQUARE_NUMBER],
  /**
   * The pawns are stored in a bitboard where each square is a bit - hence 64 bits
   * The reason for using bitboards for the pawns was twofold. 
   * 1. To show bitboards, so setting moving and clearing bits. 
   * 2. It makes evaluation of pawn structures easier as you can use bit masks.
   * So we'll have three bitboards, once with the white pawns, another with the black pawns
   * and a third with both color pawns (intersection)
   */
  pawns: [u64; 3],
  /**
   * Black or white.
   */
  king_square: [u8; 2],
  side: u8,
  en_passant_square: u8,
  /**
   * Fifty moves counter for draw, in our case will be hundred moves because we'll
   * using half moves and not full moves.
   */
  fifty_full_moves: u32,
  /**
   * The counter of how many half moves are into the current search.
   */
  actual_half_moves: u32,
  /**
   * The counter of the total half moves played. It's needed for
   * looking back and determining repetitions when we'll come to storing our history.
   */
  total_half_moves: u32,
  /**
   * It's a unique key which is generated for each game position.
   */
  position_key: u64,
  /**
   * The number of pieces that are on the board. Indexed by piece type (SquareStatus enum).
   */
  actual_pieces_number: [u8; 13],
  /**
   * Thery are every pieces that are not a pawn. Array size is three for black, white or both.
   */
  big_pieces_number: [u8; 3],
  /**
   * Rooks and Queens. Array size is three for black, white or both.
   */
  major_pieces_number: [u8; 3],
  /**
   * Bishops and Knights. Array size is three for black, white or both.
   */
  minor_pieces_number: [u8; 3]
}
