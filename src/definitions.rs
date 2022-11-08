pub const PROGRAM_NAME: &str = "Rust Chess Engine";
pub const BOARD_SQUARE_NUMBER: usize = 120;
pub const MAX_GAME_HALF_MOVES: usize = 2048;
// https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
pub const START_FEN: &str =  "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

// A square can be empty or contain a Wn (White kNight) chess piece for example
#[derive(Copy, Clone)]
pub enum Pieces {
  Empty, Wp, Wn, Wb, Wr, Wq, Wk, Bp, Bn, Bb, Br, Bq, Bk,
}

impl Pieces {
  pub fn from_u32(value: u32) -> Pieces {
    match value {
      0 => Pieces::Empty,
      1 => Pieces::Wp,
      2 => Pieces::Wn,
      3 => Pieces::Wb,
      4 => Pieces::Wr,
      5 => Pieces::Wq,
      6 => Pieces::Wk,
      7 => Pieces::Bp,
      8 => Pieces::Bn,
      9 => Pieces::Bb,
      10 => Pieces::Br,
      11 => Pieces::Bq,
      12 => Pieces::Bk,
      _ => panic!("Unknown value: {}", value),
    }
  }
}

#[derive(Copy, Clone)]
pub enum ChessboardFiles {
  A, B, C, D, E, F, G, H, None,
}

impl ChessboardFiles {
  pub fn from_u32(value: u32) -> ChessboardFiles {
    match value {
      0 => ChessboardFiles::A,
      1 => ChessboardFiles::B,
      2 => ChessboardFiles::C,
      3 => ChessboardFiles::D,
      4 => ChessboardFiles::E,
      5 => ChessboardFiles::F,
      6 => ChessboardFiles::G,
      7 => ChessboardFiles::H,
      8 => ChessboardFiles::None,
      _ => panic!("Unknown value: {}", value),
    }
  }
}

// R stands for "Rank"
#[derive(Copy, Clone)]
pub enum ChessboardRanks {
  R1, R2, R3, R4, R5, R6, R7, R8, None
}

impl ChessboardRanks {
  pub fn from_u32(value: u32) -> ChessboardRanks {
    match value {
      0 => ChessboardRanks::R1,
      1 => ChessboardRanks::R2,
      2 => ChessboardRanks::R3,
      3 => ChessboardRanks::R4,
      4 => ChessboardRanks::R5,
      5 => ChessboardRanks::R6,
      6 => ChessboardRanks::R7,
      7 => ChessboardRanks::R8,
      8 => ChessboardRanks::None,
      _ => panic!("Unknown value: {}", value),
    }
  }
}

#[derive(Copy, Clone)]
pub enum Colors {
  White, Black, Both
}

#[derive(Copy, Clone)]
pub enum Squares {
  A1 = 21, B1, C1, D1, E1, F1, G1, H1,
  A2 = 31, B2, C2, D2, E2, F2, G2, H2,
  A3 = 41, B3, C3, D3, E3, F3, G3, H3,
  A4 = 51, B4, C4, D4, E4, F4, G4, H4,
  A5 = 61, B5, C5, D5, E5, F5, G5, H5,
  A6 = 71, B6, C6, D6, E6, F6, G6, H6,
  A7 = 81, B7, C7, D7, E7, F7, G7, H7,
  A8 = 91, B8, C8, D8, E8, F8, G8, H8, NoSquare, OffBoard
}

/**
 * They are rapresented by 4 bits. Ex. bit 1 -> [1 0 0 0]; bit 4 -> [0 0 1 0].
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
  position_key: u64,
}

impl Undo {
  pub fn new() -> Undo {
    let half_move: i32 = 0;
    let castle_permission: i32 = 0;
    let en_passant_square: i32 = 0;
    let fifty_full_moves: i32 = 0;
    let position_key: u64 = 0;

    Undo {
      half_move,
      castle_permission,
      en_passant_square,
      fifty_full_moves,
      position_key,
    }
  }
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
   * and a third with both color pawns (intersection).
   * We could rapresent every chess piece on a bitboard (maybe in the future will be done), but
   * the pawns give the most performance improvement.
   */
  pawns: [u64; 3],
  /**
   * Black or white.
   */
  king_square: [i32; 2],
  side: Colors,
  en_passant_square: Squares,
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
  position_key: u64,
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
  history: Vec<Undo>,
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

impl Board {
  pub fn new() -> Board {
    let pieces: [i32; BOARD_SQUARE_NUMBER] = [0; BOARD_SQUARE_NUMBER];
    let pawns: [u64; 3] = [0; 3];
    let king_square: [i32; 2] = [0; 2];
    let side: Colors = Colors::White;
    let en_passant_square: Squares = Squares::NoSquare;
    let fifty_full_moves: i32 = 0;
    let actual_half_moves: i32 = 0;
    let total_half_moves: i32 = 0;
    let castel_permission: i32 = 0;
    let position_key: u64 = 0;
    let actual_pieces_number: [i32; 13] = [0; 13];
    let big_pieces_number: [i32; 3] = [0; 3];
    let major_pieces_number: [i32; 3] = [0; 3];
    let minor_pieces_number: [i32; 3] = [0; 3];
    let history: Vec<Undo> = Vec::with_capacity(MAX_GAME_HALF_MOVES); // to initialize
    let pieces_list: [[i32; 13]; 10] = [
      [0; 13], [0; 13], [0; 13], [0; 13], [0; 13], [0; 13], [0; 13], [0; 13], [0; 13], [0; 13],
    ];

    Board {
      pieces,
      pawns,
      king_square,
      side,
      en_passant_square,
      fifty_full_moves,
      actual_half_moves,
      total_half_moves,
      castel_permission,
      position_key,
      actual_pieces_number,
      big_pieces_number,
      major_pieces_number,
      minor_pieces_number,
      history,
      pieces_list,
    }
  }

  pub fn pieces(&mut self) -> &mut [i32; BOARD_SQUARE_NUMBER] {
    &mut self.pieces
  }

  pub fn pawns(&mut self) -> &mut [u64; 3] {
    &mut self.pawns
  }

  pub fn king_square(&mut self) -> &mut [i32; 2] {
    &mut self.king_square
  }

  pub fn side(&mut self) -> &mut Colors {
    &mut self.side
  }

  pub fn en_passant_square(&mut self) -> &mut Squares {
    &mut self.en_passant_square
  }
  pub fn fifty_full_moves(&mut self) -> &mut i32 {
    &mut self.fifty_full_moves
  }
  pub fn actual_half_moves(&mut self) -> &mut i32 {
    &mut self.actual_half_moves
  }
  pub fn total_half_moves(&mut self) -> &mut i32 {
    &mut self.total_half_moves
  }

  pub fn actual_pieces_number(&mut self) -> &mut [i32; 13] {
    &mut self.actual_pieces_number
  }

  pub fn castel_permission(&mut self) -> &mut i32 {
    &mut self.castel_permission
  }

  pub fn position_key(&mut self) -> &mut u64 {
    &mut self.position_key
  }

  pub fn big_pieces_number(&mut self) -> &mut [i32; 3] {
    &mut self.big_pieces_number
  }

  pub fn major_pieces_number(&mut self) -> &mut [i32; 3] {
    &mut self.major_pieces_number
  }

  pub fn minor_pieces_number(&mut self) -> &mut [i32; 3] {
    &mut self.minor_pieces_number
  }
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
  /**
   * The board with 120 squares is the board reference for the search engine.
   * Check out why we need to change the boards squares notation on the init_squares() function
   * inside the init.rs file.
   *
   * Original structure of definitions.board_120_squares_in_64_squares_notation,
   * more like "board_120_squares_in_120_squares_notation" (as we play with black):
   *      A    B    C    D    E    F    G    H
   *    000  001  002  003  004  005  006  007  008  009
   *    010  011  012  013  014  015  016  017  018  019
   * 1  020  021  022  023  024  025  026  027  028  029
   * 2  030  031  032  033  034  035  036  037  038  039
   * 3  040  041  042  043  044  045  046  047  048  049
   * 4  050  051  052  053  054  055  056  057  058  059
   * 5  060  061  062  063  064  065  066  067  068  069
   * 6  070  071  072  073  074  075  076  077  078  079
   * 7  080  081  082  083  084  085  086  087  088  089
   * 8  090  091  092  093  094  095  096  097  098  099
   *    100  101  102  103  104  105  106  107  108  109
   *    110  111  112  113  114  115  116  117  118  119
   */
  board_120_squares_in_64_squares_notation: [i32; BOARD_SQUARE_NUMBER],
  /**
   * Original structure of definitions.board_64_squares_in_120_squares_notation,
   * more like "board_64_squares_in_64_squares_notation" (as we play with black):
   *
   *     A  B  C  D  E  F  G  H
   * 1  00 01 02 03 04 05 06 07
   * 2  08 09 10 11 12 13 14 15
   * 3  16 17 18 19 20 21 22 23
   * 4  24 25 26 27 28 29 30 31
   * 5  32 33 34 35 36 37 38 39
   * 6  40 41 42 43 44 45 46 47
   * 7  48 49 50 51 52 53 54 55
   * 8  56 57 58 59 60 61 62 63
   */
  board_64_squares_in_120_squares_notation: [i32; 64],
  bit_mask_to_set_bit_inside_bitboard: [u64; 64],
  bit_mask_to_clear_bit_inside_bitboard: [u64; 64],
  piece_keys: [[u64; 120]; 13],
  side_key: u64,
  castle_keys: [u64; 16],
}

impl Definitions {
  pub fn new() -> Definitions {
    let board_120_squares_in_64_squares_notation: [i32; BOARD_SQUARE_NUMBER] =
      [0; BOARD_SQUARE_NUMBER];
    let board_64_squares_in_120_squares_notation: [i32; 64] = [0; 64];
    let bit_mask_to_set_bit_inside_bitboard: [u64; 64] = [0; 64];
    let bit_mask_to_clear_bit_inside_bitboard: [u64; 64] = [0; 64];
    let piece_keys: [[u64; 120]; 13] = [
      [0; 120], [0; 120], [0; 120], [0; 120], [0; 120], [0; 120], [0; 120], [0; 120], [0; 120],
      [0; 120], [0; 120], [0; 120], [0; 120],
    ];
    let side_key: u64 = 0;
    let castle_keys: [u64; 16] = [0; 16];

    Definitions {
      board_120_squares_in_64_squares_notation,
      board_64_squares_in_120_squares_notation,
      bit_mask_to_set_bit_inside_bitboard,
      bit_mask_to_clear_bit_inside_bitboard,
      piece_keys,
      side_key,
      castle_keys,
    }
  }

  pub fn board_120_squares_in_64_squares_notation(&mut self) -> &mut [i32; BOARD_SQUARE_NUMBER] {
    &mut self.board_120_squares_in_64_squares_notation
  }

  pub fn board_64_squares_in_120_squares_notation(&mut self) -> &mut [i32; 64] {
    &mut self.board_64_squares_in_120_squares_notation
  }

  pub fn bit_mask_to_set_bit_inside_bitboard(&mut self) -> &mut [u64; 64] {
    &mut self.bit_mask_to_set_bit_inside_bitboard
  }

  pub fn bit_mask_to_clear_bit_inside_bitboard(&mut self) -> &mut [u64; 64] {
    &mut self.bit_mask_to_clear_bit_inside_bitboard
  }

  pub fn piece_keys(&mut self) -> &mut [[u64; 120]; 13] {
    &mut self.piece_keys
  }

  pub fn side_key(&mut self) -> &mut u64 {
    &mut self.side_key
  }

  pub fn castle_keys(&mut self) -> &mut [u64; 16] {
    &mut self.castle_keys
  }
}