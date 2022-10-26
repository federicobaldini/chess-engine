pub const PROGRAM_NAME: &str = "Rust Chess Engine";
pub const BOARD_SQUARE_NUMBER: usize = 120;
pub const MAX_GAME_HALF_MOVES: usize = 2048;

// A square can be empty or contain a Wn (White kNight) chess piece for example
pub enum Pieces {
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

/**
 * They are rappresented by 4 bits. Ex. bit 1 -> [1 0 0 0]; bit 4 -> [0 0 1 0].
 * The case [1 0 0 1] tell us that white castel on king side and black castel on queen side.
 */
pub enum Castle {
  WhiteKingSideCastel = 1,
  WhiteQueenSideCastel = 2,
  BlackKingSideCastel = 4,
  BlackQueenSideCastel = 8,
}

pub struct Undo {
  /**
   * The move made.
   */
  half_move: i32,
  /**
   * Before the move has made.
   */
  castle_permission: i32,
  /**
   * Before the move has made.
   */
  en_passant_square: i32,
  /**
   * Fifty moves counter for draw, in our case will be hundred moves because we'll
   * using half moves and not full moves. Before the move has made.
   */
  fifty_full_moves: i32,
  /**
   * It's a unique key which is generated for each game position. Before the move has made.
   */
  position_key: i64,
}

pub struct Board {
  /**
  * It contains the whole chess board squares, and for each square contains the chess piece on it (Empty if none). 
  */
  pieces: [i32; BOARD_SQUARE_NUMBER],
  /**
   * The pawns are stored in a bitboard where each square is a bit - hence 64 bits
   * The reason for using bitboards for the pawns was twofold. 
   * 1. To show bitboards, so setting moving and clearing bits. 
   * 2. It makes evaluation of pawn structures easier as you can use bit masks.
   * So we'll have three bitboards, once with the white pawns, another with the black pawns
   * and a third with both color pawns (intersection)
   */
  pawns: [i64; 3],
  /**
   * Black or white.
   */
  king_square: [i32; 2],
  side: i32,
  en_passant_square: i32,
  /**
   * Fifty moves counter for draw, in our case will be hundred moves because we'll
   * using half moves and not full moves.
   */
  fifty_full_moves: i32,
  /**
   * The counter of how many half moves are into the current search.
   */
  actual_half_moves: i32,
  /**
   * The counter of the total half moves played. It's needed for
   * looking back and determining repetitions when we'll come to storing our history.
   */
  total_half_moves: i32,
  castel_permission: i32,
  /**
   * It's a unique key which is generated for each game position.
   */
  position_key: i64,
  /**
   * The number of pieces that are on the board. Indexed by piece type (Pieces enum).
   */
  actual_pieces_number: [i32; 13],
  /**
   * They are every pieces that are not a pawn. Array size is three for black, white or both.
   */
  big_pieces_number: [i32; 3],
  /**
   * Rooks and Queens. Array size is three for black, white or both.
   */
  major_pieces_number: [i32; 3],
  /**
   * Bishops and Knights. Array size is three for black, white or both.
   */
  minor_pieces_number: [i32; 3],
  history: [Undo; MAX_GAME_HALF_MOVES],
  /**
   * It's an array of 10 elements, eachone contains the list of piece types (13).
   * Why ten elements? Because for example at the start of the game you have 2 rooks, assumed
   * that you promote all the pawns at rook, you can have at maximum 10 equal pieces.
   * Use case: to set the first white knight to E3 -> pieces_list[Pieces::Wn][0] = ChessboardFiles::E + ChessboardRanks::R3
   * This structure is more usefull (in some cases) that Definitions::pieces because here we have
   * less empty squares and to get all the pieces on the board is sufficient to loop every piece type untill
   * we get NoSquare as square value. So it's increment the performance of the search move engine.
   */
  pieces_list: [[i32; 13]; 10],
}

/* MACROS */

/**
 * f -> board file
 * r -> board rank
 */
#[macro_export]
macro_rules! file_rank_to_square_120 {
  ( $f:expr, $r:expr ) => {
    (21 + $f) + ($r * 10)
  };
}

/* GLOBALS */

pub struct Definitions {
  board_120_squares: [i32; BOARD_SQUARE_NUMBER],
  board_64_squares: [i32; 64],
}

impl Definitions {
  pub fn new() -> Definitions {
    let board_120_squares: [i32; BOARD_SQUARE_NUMBER] = [0; BOARD_SQUARE_NUMBER];
    let board_64_squares: [i32; 64] = [0; 64];
    Definitions {board_120_squares, board_64_squares}
  }

  pub fn board_120_squares(&mut self) -> &mut[i32; BOARD_SQUARE_NUMBER] {
    &mut self.board_120_squares
  }

  pub fn board_64_squares(&mut self) -> &mut[i32; 64] {
    &mut self.board_64_squares
  }
}

/* FUNCTIONS */