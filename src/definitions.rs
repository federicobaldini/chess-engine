use rand::Rng;

/* CONSTANTS */

pub const PROGRAM_NAME: &str = "Rust Chess Engine";
pub const BOARD_SQUARE_NUMBER: usize = 120;
pub const MAX_GAME_HALF_MOVES: usize = 2048;
// <https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation>
pub const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const PIECE_CHARACTERS: [char; 13] = [
  '.', 'P', 'N', 'B', 'R', 'Q', 'K', 'p', 'n', 'b', 'r', 'q', 'k',
];
pub const SIDE_CHARACTERS: [char; 3] = ['w', 'b', '-'];
pub const RANK_CHARACTERS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];
pub const FILE_CHARACTERS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
// (index)pieces = (0) None / White: (1)pawn, (2)knight, (3)bishop (4)rook (5)queen (6)king / Black: (7)pawn, (8)knight, (9)bishop (10)rook (11)queen (12)king
pub const PIECE_BIG: [bool; 13] = [
  false, false, true, true, true, true, true, false, true, true, true, true, true,
];
pub const PIECE_MAJOR: [bool; 13] = [
  false, false, false, false, true, true, true, false, false, false, true, true, true,
];
pub const PIECE_MINOR: [bool; 13] = [
  false, false, true, true, false, false, false, false, true, true, false, false, false,
];
pub const PIECE_VALUE: [i32; 13] = [
  0, 100, 325, 325, 550, 1000, 50000, 100, 325, 325, 550, 1000, 50000,
];
pub const PIECE_COLOR: [Colors; 13] = [
  Colors::Both,
  Colors::White,
  Colors::White,
  Colors::White,
  Colors::White,
  Colors::White,
  Colors::White,
  Colors::Black,
  Colors::Black,
  Colors::Black,
  Colors::Black,
  Colors::Black,
  Colors::Black,
];
pub const PIECE_KNIGHT: [bool; 13] = [
  false, false, true, false, false, false, false, false, true, false, false, false, false,
];
pub const PIECE_KING: [bool; 13] = [
  false, false, false, false, false, false, true, false, false, false, false, false, true,
];
pub const PIECE_ROOK_QUEEN: [bool; 13] = [
  false, false, false, false, true, true, false, false, false, false, true, true, false,
];
pub const PIECE_BISHOP_QUEEN: [bool; 13] = [
  false, false, false, true, false, true, false, false, false, true, false, true, false,
];

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

/* FUNCTIONS */

fn generate_random_chess_piece_hash() -> u64 {
  let mut rng = rand::thread_rng();
  rng.gen::<u64>()
}

/* STRUCTS (and their implementations) */

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

impl PartialEq for Pieces {
  fn eq(&self, other: &Self) -> bool {
    *self as u32 == *other as u32
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

impl PartialEq for ChessboardFiles {
  fn eq(&self, other: &Self) -> bool {
    *self as u32 == *other as u32
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

impl PartialEq for ChessboardRanks {
  fn eq(&self, other: &Self) -> bool {
    *self as u32 == *other as u32
  }
}

#[derive(Copy, Clone, Debug)]
pub enum Colors {
  White, Black, Both
}

impl PartialEq for Colors {
  fn eq(&self, other: &Self) -> bool {
    *self as u32 == *other as u32
  }
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

impl Squares {
  pub fn from_u32(value: u32) -> Squares {
    match value {
      21 => Squares::A1,
      22 => Squares::B1,
      23 => Squares::C1,
      24 => Squares::D1,
      25 => Squares::E1,
      26 => Squares::F1,
      27 => Squares::G1,
      28 => Squares::H1,
      31 => Squares::A2,
      32 => Squares::B2,
      33 => Squares::C2,
      34 => Squares::D2,
      35 => Squares::E2,
      36 => Squares::F2,
      37 => Squares::G2,
      38 => Squares::H2,
      41 => Squares::A3,
      42 => Squares::B3,
      43 => Squares::C3,
      44 => Squares::D3,
      45 => Squares::E3,
      46 => Squares::F3,
      47 => Squares::G3,
      48 => Squares::H3,
      51 => Squares::A4,
      52 => Squares::B4,
      53 => Squares::C4,
      54 => Squares::D4,
      55 => Squares::E4,
      56 => Squares::F4,
      57 => Squares::G4,
      58 => Squares::H4,
      61 => Squares::A5,
      62 => Squares::B5,
      63 => Squares::C5,
      64 => Squares::D5,
      65 => Squares::E5,
      66 => Squares::F5,
      67 => Squares::G5,
      68 => Squares::H5,
      71 => Squares::A6,
      72 => Squares::B6,
      73 => Squares::C6,
      74 => Squares::D6,
      75 => Squares::E6,
      76 => Squares::F6,
      77 => Squares::G6,
      78 => Squares::H6,
      81 => Squares::A7,
      82 => Squares::B7,
      83 => Squares::C7,
      84 => Squares::D7,
      85 => Squares::E7,
      86 => Squares::F7,
      87 => Squares::G7,
      88 => Squares::H7,
      91 => Squares::A8,
      92 => Squares::B8,
      93 => Squares::C8,
      94 => Squares::D8,
      95 => Squares::E8,
      96 => Squares::F8,
      97 => Squares::G8,
      98 => Squares::H8,
      99 => Squares::NoSquare,
      100 => Squares::OffBoard,
      _ => panic!("Unknown value: {}", value),
    }
  }
}

impl PartialEq for Squares {
  fn eq(&self, other: &Self) -> bool {
    *self as u32 == *other as u32
  }
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
pub struct Move {
  /* 'move' is a reserved keyword in Rust */
  mov_e: i32,
  score: i32,
}

/**
 *     A  B  C  D  E  F  G  H
 * 1  21 22 23 24 25 26 27 28
 * 2  31 32 33 34 35 36 37 38
 * 3  41 42 43 44 45 46 47 48
 * 4  51 52 53 54 55 56 57 58
 * 5  61 62 63 64 65 66 67 68
 * 6  71 72 73 74 75 76 77 78
 * 7  81 82 83 84 85 86 87 88
 * 8  91 92 93 94 95 96 97 98
 *
 * 7 bits are sufficient to index a square ranging from 21 to 98:
 * 21 = 001 0101 / 98 = 110 0010
 *
 * The 28 bits of the "move" are used in this way:
 *
 * the first 7 bits to indicate the starting square of the move
 * 0000 0000 0000 0000 0000 0111 1111 -> From 0x7F
 *
 * the bits from 8 to 14 to indicate the ending square of the move
 * 0000 0000 0000 0011 1111 1000 0000 -> To >> 7, 0x7F
 *
 * the bits from 15 to 18 to indicate if a piece has been captured with this move
 * 0000 0000 0011 1100 0000 0000 0000 -> Captured >> 14, 0xF
 *
 * the 19 bit to indicate if it is an "en passant" move
 * 0000 0000 0100 0000 0000 0000 0000 -> EP 0x40000
 *
 * the 20 bit to indicate if it is a "pawn start" move
 * 0000 0000 1000 0000 0000 0000 0000 -> Pawn start 0x80000
 *
 * the bits from 21 to 24 to indicate in which piece I have promoted a pawn
 * 0000 1111 0000 0000 0000 0000 0000 -> Promoted >> 20, 0xF
 *
 * the 25 bit to indicate if it is a "Castle" move
 * 0001 0000 0000 0000 0000 0000 0000 -> Castle 0x1000000
 */
impl Move {
  pub fn new(mov_e: i32, score: i32) -> Move {
    Move { mov_e, score }
  }

  pub fn mov_e(&self) -> i32 {
    self.mov_e
  }

  pub fn set_mov_e(&mut self, mov_e: i32) {
    self.mov_e = mov_e;
  }

  pub fn from_square(&self) -> i32 {
    self.mov_e & 0x7F
  }

  pub fn to_square(&self) -> i32 {
    (self.mov_e >> 7) & 0x7F
  }

  pub fn captured_piece(&self) -> i32 {
    (self.mov_e >> 14) & 0xF
  }

  pub fn promoted(&self) -> i32 {
    (self.mov_e >> 20) & 0xF
  }

  pub fn captured_piece_with_en_passant(&self) -> i32 {
    self.mov_e & 0x7C000
  }

  pub fn promotion(&self) -> i32 {
    self.mov_e & 0xF00000
  }

  pub fn en_passant(&self) -> i32 {
    self.mov_e & 0x40000
  }

  pub fn pawn_start(&self) -> i32 {
    self.mov_e & 0x80000
  }

  pub fn castle(&self) -> i32 {
    self.mov_e & 0x1000000
  }
}

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub struct Definitions {
  /**
   * The board with 120 squares is the board reference for the search engine.
   * Check out why we need to change the boards squares notation on the init_squares() function
   * inside the init.rs file.
   *
   * Original structure of definitions.board_120_squares_in_64_squares_notation,
   * more like "board_120_squares_in_120_squares_notation" (as we play with black):
   *          A    B    C    D    E    F    G    H
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
  /**
   * At given square, we want to know immediatly on which file the square is, so
   * we use this structure.
   */
  files_board: [i32; BOARD_SQUARE_NUMBER],
  /**
   * At given square, we want to know immediatly on which rank the square is, so
   * we use this structure.
   */
  ranks_board: [i32; BOARD_SQUARE_NUMBER],
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

    let files_board: [i32; BOARD_SQUARE_NUMBER] = [0; BOARD_SQUARE_NUMBER];
    let ranks_board: [i32; BOARD_SQUARE_NUMBER] = [0; BOARD_SQUARE_NUMBER];

    Definitions {
      board_120_squares_in_64_squares_notation,
      board_64_squares_in_120_squares_notation,
      bit_mask_to_set_bit_inside_bitboard,
      bit_mask_to_clear_bit_inside_bitboard,
      piece_keys,
      side_key,
      castle_keys,
      files_board,
      ranks_board,
    }
  }

  pub fn board_120_squares_in_64_squares_notation(&self) -> [i32; BOARD_SQUARE_NUMBER] {
    self.board_120_squares_in_64_squares_notation
  }

  pub fn board_64_squares_in_120_squares_notation(&self) -> [i32; 64] {
    self.board_64_squares_in_120_squares_notation
  }

  pub fn bit_mask_to_set_bit_inside_bitboard(&self) -> [u64; 64] {
    self.bit_mask_to_set_bit_inside_bitboard
  }

  pub fn bit_mask_to_clear_bit_inside_bitboard(&self) -> [u64; 64] {
    self.bit_mask_to_clear_bit_inside_bitboard
  }

  pub fn piece_keys(&self) -> [[u64; 120]; 13] {
    self.piece_keys
  }

  pub fn side_key(&self) -> u64 {
    self.side_key
  }

  pub fn castle_keys(&self) -> [u64; 16] {
    self.castle_keys
  }

  pub fn files_board(&self) -> [i32; BOARD_SQUARE_NUMBER] {
    self.files_board
  }

  pub fn ranks_board(&self) -> [i32; BOARD_SQUARE_NUMBER] {
    self.ranks_board
  }

  pub fn init(&mut self) {
    self.init_squares();
    self.init_masks();
    self.init_hash_keys();
    self.init_files_ranks_board();
  }

  /**
   * The board with 120 squares is the board reference for the search engine.
   *
   * We need to have the 120 squares board and the 64 squares board as follow for these reasons:
   *
   * When we have a 64 squares bitboard we can have for example a bit on square B2.
   *
   * The B2 square has position at index 9 in the array board with 64 squares in 64 squares notation.
   * (reference: "board_64_squares_in_64_squares_notation" in definitions.rs file).
   *
   * Because we need to have that position on the 120 squares board,
   *
   * the index 9 not correspond to B2 square, it stays at index 32 in the 120 squares notation.
   * (reference: "board_120_squares_in_120_squares_notation" in definitions.rs file).
   *
   * So if we have a board of 64 squares in 120 squares notation, at index 9 we got 32, then
   * the 32 as index on the 120 squares board in 64 squares notation it's the B2 square and contains the 9 value,
   * the square that originally we wanted arrive to on the 120 squares board.
   *
   * The value that we got can help us to reverse the process and get the B2 square on 64 squares board,
   * that is the square at index 9.
   *
   * Final structure of definitions.board_64_squares_in_120_squares_notation (as we play with black):
   *
   *     A  B  C  D  E  F  G  H
   * 1  21 22 23 24 25 26 27 28
   * 2  31 32 33 34 35 36 37 38
   * 3  41 42 43 44 45 46 47 48
   * 4  51 52 53 54 55 56 57 58
   * 5  61 62 63 64 65 66 67 68
   * 6  71 72 73 74 75 76 77 78
   * 7  81 82 83 84 85 86 87 88
   * 8  91 92 93 94 95 96 97 98
   *
   * Final structure of definitions.board_120_squares_in_64_squares_notation (as we play with black):
   *        A  B  C  D  E  F  G  H
   *    65 65 65 65 65 65 65 65 65 65
   *    65 65 65 65 65 65 65 65 65 65
   * 1  65 00 01 02 03 04 05 06 07 65
   * 2  65 08 09 10 11 12 13 14 15 65
   * 3  65 16 17 18 19 20 21 22 23 65
   * 4  65 24 25 26 27 28 29 30 31 65
   * 5  65 32 33 34 35 36 37 38 39 65
   * 6  65 40 41 42 43 44 45 46 47 65
   * 7  65 48 49 50 51 52 53 54 55 65
   * 8  65 56 57 58 59 60 61 62 63 65
   *    65 65 65 65 65 65 65 65 65 65
   *    65 65 65 65 65 65 65 65 65 65
   */
  fn init_squares(&mut self) {
    let mut square_120: i32;
    let mut square_64: i32 = 0;

    self.board_120_squares_in_64_squares_notation[0..BOARD_SQUARE_NUMBER].fill(65);
    self.board_64_squares_in_120_squares_notation[0..64].fill(120);

    for rank in ChessboardRanks::R1 as i32..=ChessboardRanks::R8 as i32 {
      for file in ChessboardFiles::A as i32..=ChessboardFiles::H as i32 {
        square_120 = file_rank_to_square_120!(file, rank);
        self.board_64_squares_in_120_squares_notation[square_64 as usize] = square_120;
        self.board_120_squares_in_64_squares_notation[square_120 as usize] = square_64;
        square_64 += 1;
      }
    }
  }

  fn init_masks(&mut self) {
    for index in 0..64 {
      self.bit_mask_to_set_bit_inside_bitboard[index as usize] |= 1u64 << index;
      // It is the bitwise complement of the "bit_mask_to_set_bit_inside_bitboard"
      self.bit_mask_to_clear_bit_inside_bitboard[index as usize] =
        !self.bit_mask_to_set_bit_inside_bitboard[index as usize];
    }
  }

  fn init_hash_keys(&mut self) {
    self.piece_keys = [[generate_random_chess_piece_hash(); 120]; 13];
    self.side_key = generate_random_chess_piece_hash();
    self.castle_keys = [generate_random_chess_piece_hash(); 16];
  }

  /**
   * Final structure of files_board:
   *
   * 100 100 100 100 100 100 100 100 100 100
   * 100 100 100 100 100 100 100 100 100 100
   * 100   0   1   2   3   4   5   6   7 100
   * 100   0   1   2   3   4   5   6   7 100
   * 100   0   1   2   3   4   5   6   7 100
   * 100   0   1   2   3   4   5   6   7 100
   * 100   0   1   2   3   4   5   6   7 100
   * 100   0   1   2   3   4   5   6   7 100
   * 100   0   1   2   3   4   5   6   7 100
   * 100   0   1   2   3   4   5   6   7 100
   * 100 100 100 100 100 100 100 100 100 100
   * 100 100 100 100 100 100 100 100 100 100
   *
   * Final structure of ranks_board:
   *
   * 100 100 100 100 100 100 100 100 100 100
   * 100 100 100 100 100 100 100 100 100 100
   * 100   0   0   0   0   0   0   0   0 100
   * 100   1   1   1   1   1   1   1   1 100
   * 100   2   2   2   2   2   2   2   2 100
   * 100   3   3   3   3   3   3   3   3 100
   * 100   4   4   4   4   4   4   4   4 100
   * 100   5   5   5   5   5   5   5   5 100
   * 100   6   6   6   6   6   6   6   6 100
   * 100   7   7   7   7   7   7   7   7 100
   * 100 100 100 100 100 100 100 100 100 100
   * 100 100 100 100 100 100 100 100 100 100
   */
  fn init_files_ranks_board(&mut self) {
    let mut square_120: i32;

    self.files_board = [Squares::OffBoard as i32; BOARD_SQUARE_NUMBER];
    self.ranks_board = [Squares::OffBoard as i32; BOARD_SQUARE_NUMBER];

    for rank in ChessboardRanks::R1 as i32..=ChessboardRanks::R8 as i32 {
      for file in ChessboardFiles::A as i32..=ChessboardFiles::H as i32 {
        square_120 = file_rank_to_square_120!(file, rank);
        self.files_board[square_120 as usize] = file;
        self.ranks_board[square_120 as usize] = rank;
      }
    }
  }
}
