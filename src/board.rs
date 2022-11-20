use crate::definitions::*;
use crate::file_rank_to_square_120;
use crate::hashkeys::*;

#[derive(Copy, Clone)]
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
  king_square: [Squares; 2],
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
  castel_permission: u8,
  /**
   * It's a unique key which is generated for each game position.
   */
  position_key: u64,
  /**
   * The number of pieces that are on the board. Indexed by piece type (Pieces enum).
   */
  actual_pieces_number: [i32; 13],
  /**
   * They are every pieces that are not a pawn. Array size is two for black and white.
   */
  big_pieces_number: [i32; 2],
  /**
   * Rooks and Queens. Array size is two for black and white.
   */
  major_pieces_number: [i32; 2],
  /**
   * Bishops and Knights. Array size is two for black and white.
   */
  minor_pieces_number: [i32; 2],
  /**
   * Is the value of the material for black and white.
   */
  material: [i32; 2],
  history: [Undo; MAX_GAME_HALF_MOVES],
  /**
   * It's an array of 13 piece types, eachone contains the list of pieces (10).
   * Why ten elements? Because for example at the start of the game you have 2 rooks, assumed
   * that you promote all the pawns at rook, you can have at maximum 10 equal pieces.
   * Use case: to set the first white knight to E3 -> pieces_list[Pieces::Wn][0] = ChessboardFiles::E + ChessboardRanks::R3
   * This structure is more usefull (in some cases) that Definitions::pieces because here we have
   * less empty squares and to get all the pieces on the board is sufficient to loop every piece type untill
   * we get NoSquare as square value. So it's increment the performance of the search move engine.
   */
  pieces_list: [[i32; 10]; 13],
}

impl Board {
  pub fn new() -> Board {
    let pieces: [i32; BOARD_SQUARE_NUMBER] = [0; BOARD_SQUARE_NUMBER];
    let pawns: [u64; 3] = [0; 3];
    let king_square: [Squares; 2] = [Squares::NoSquare; 2];
    let side: Colors = Colors::White;
    let en_passant_square: Squares = Squares::NoSquare;
    let fifty_full_moves: i32 = 0;
    let actual_half_moves: i32 = 0;
    let total_half_moves: i32 = 0;
    let castel_permission: u8 = 0;
    let position_key: u64 = 0;
    let actual_pieces_number: [i32; 13] = [0; 13];
    let big_pieces_number: [i32; 2] = [0; 2];
    let major_pieces_number: [i32; 2] = [0; 2];
    let minor_pieces_number: [i32; 2] = [0; 2];
    let material: [i32; 2] = [0; 2];
    let history: [Undo; MAX_GAME_HALF_MOVES] = [Undo::new(); MAX_GAME_HALF_MOVES];
    let pieces_list: [[i32; 10]; 13] = [
      [0; 10], [0; 10], [0; 10], [0; 10], [0; 10], [0; 10], [0; 10], [0; 10], [0; 10], [0; 10],
      [0; 10], [0; 10], [0; 10],
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
      material,
      history,
      pieces_list,
    }
  }

  pub fn pieces(&self) -> [i32; BOARD_SQUARE_NUMBER] {
    self.pieces
  }

  pub fn pawns(&self) -> [u64; 3] {
    self.pawns
  }

  pub fn side(&self) -> Colors {
    self.side
  }

  pub fn en_passant_square(&self) -> Squares {
    self.en_passant_square
  }

  pub fn castel_permission(&self) -> u8 {
    self.castel_permission
  }

  pub fn reset_board(&mut self, definitions: &Definitions) {
    self.pieces = [Squares::OffBoard as i32; BOARD_SQUARE_NUMBER];
    for index in 0..64 {
      self.pieces[definitions.board_64_squares_in_120_squares_notation()[index] as usize] =
        Pieces::Empty as i32;
    }
    self.big_pieces_number = [0; 2];
    self.major_pieces_number = [0; 2];
    self.minor_pieces_number = [0; 2];
    self.pawns = [0u64; 3];
    self.actual_pieces_number = [0; 13];
    self.king_square = [Squares::NoSquare; 2];
    self.side = Colors::Both;
    self.en_passant_square = Squares::NoSquare;
    self.fifty_full_moves = 0;
    self.actual_half_moves = 0;
    self.total_half_moves = 0;
    self.castel_permission = 0;
    self.position_key = 0u64;
  }

  // fen = Forsyth–Edwards Notation
  pub fn parse_fen(&mut self, definitions: &Definitions, fen: &str) {
    let mut rank: ChessboardRanks = ChessboardRanks::R8;
    let mut file: ChessboardFiles = ChessboardFiles::A;
    let mut piece: Pieces = Pieces::Empty;
    let mut count: i32;
    let mut char_index: usize = 0;
    let mut square_120: i32;
    let mut square_64: i32;
    let mut fen_char: char = fen.as_bytes()[0] as char;
    let mut stop: bool = false;
    let mut index: i32 = 0;

    self.reset_board(definitions);

    while (rank as i32 >= ChessboardRanks::R1 as i32) && fen_char != ' ' {
      count = 1;
      match fen_char {
        'p' => piece = Pieces::Bp,
        'r' => piece = Pieces::Br,
        'n' => piece = Pieces::Bn,
        'b' => piece = Pieces::Bb,
        'k' => piece = Pieces::Bk,
        'q' => piece = Pieces::Bq,
        'P' => piece = Pieces::Wp,
        'R' => piece = Pieces::Wr,
        'N' => piece = Pieces::Wn,
        'B' => piece = Pieces::Wb,
        'K' => piece = Pieces::Wk,
        'Q' => piece = Pieces::Wq,
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
          piece = Pieces::Empty;
          count = fen_char as i32 - '0' as i32;
        }
        '/' | ' ' => {
          rank = ChessboardRanks::from_u32(rank as u32 - 1);
          file = ChessboardFiles::A;
        }
        _ => println!("FEN error"),
      }
      if fen_char != '/' && fen_char != ' ' {
        /*
          Image to have the File A and the Rank 4, and the actual fen_char is 4.
          We loop from 0 to 3 and the File goes from A to E (0 = A->B, 1 = B->C, 2 = C->D, 3 = D->E).
          If in that File on that Rank there is a piece then it's added, if not we go on the next File.
          "count" equal to 1 will add a piece, if it's different will skip the Files as much as count.
        */
        for _ in 0..count {
          square_64 = rank as i32 * 8 + file as i32;
          square_120 = definitions.board_64_squares_in_120_squares_notation()[square_64 as usize];
          if piece != Pieces::Empty {
            self.pieces[square_120 as usize] = piece as i32;
          }
          file = ChessboardFiles::from_u32(file as u32 + 1);
        }
      }
      // read the next character.
      char_index += 1;
      fen_char = fen.as_bytes()[char_index] as char;
    }
    // "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR " string has been read (even space).
    char_index += 1;
    fen_char = fen.as_bytes()[char_index] as char;

    self.side = if fen_char == 'w' {
      Colors::White
    } else {
      Colors::Black
    };

    // "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w" string has been read,
    // we need to skip a space and position on the new character, so +2.
    char_index += 2;
    fen_char = fen.as_bytes()[char_index] as char;
    while index < 4 && !stop {
      if fen_char == ' ' {
        stop = true;
      } else {
        match fen_char {
          'K' => self.castel_permission |= Castle::WhiteKingSideCastel as u8,
          'Q' => self.castel_permission |= Castle::WhiteQueenSideCastel as u8,
          'k' => self.castel_permission |= Castle::BlackKingSideCastel as u8,
          'q' => self.castel_permission |= Castle::BlackQueenSideCastel as u8,
          _ => {}
        }
        char_index += 1;
        fen_char = fen.as_bytes()[char_index] as char;
      }
      index += 1;
    }
    // "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq " string has been read (even space).
    char_index += 1;
    fen_char = fen.as_bytes()[char_index] as char;
    if fen_char != '-' {
      file = ChessboardFiles::from_u32(fen.as_bytes()[char_index] as u32 - 'a' as u32);
      rank = ChessboardRanks::from_u32(fen.as_bytes()[char_index + 1] as u32 - '1' as u32);
      self.en_passant_square =
        Squares::from_u32(file_rank_to_square_120!(file as i32, rank as i32) as u32);
    }
    self.position_key = generate_position_key(definitions, self);
  }

    let mut square_120: i32;
  pub fn print_board(&self, definitions: &Definitions) {
    let mut piece: i32;

    println!("\nGame Board:\n");

    for rank in (ChessboardRanks::R1 as i32..=ChessboardRanks::R8 as i32).rev() {
      print!("{}   ", rank + 1);
      for file in ChessboardFiles::A as i32..=ChessboardFiles::H as i32 {
        square_120 = file_rank_to_square_120!(file, rank);
        piece = self.pieces[square_120 as usize];
        print!("{}   ", definitions.piece_characters()[piece as usize]);
      }
      println!();
    }

    print!("\n    ");
    for file in ChessboardFiles::A as i32..=ChessboardFiles::H as i32 {
      print!("{}   ", ('a' as u8 + file as u8) as char);
    }
    println!();

    println!(
      "side:{}",
      definitions.side_characters()[self.side() as usize]
    );
    println!("enPas:{}", self.en_passant_square as i32);
    println!(
      "castle:{}{}{}{}",
      if (self.castel_permission & Castle::WhiteKingSideCastel as u8) != 0 {
        'K'
      } else {
        '-'
      },
      if (self.castel_permission & Castle::WhiteQueenSideCastel as u8) != 0 {
        'Q'
      } else {
        '-'
      },
      if (self.castel_permission & Castle::BlackKingSideCastel as u8) != 0 {
        'k'
      } else {
        '-'
      },
      if (self.castel_permission & Castle::BlackQueenSideCastel as u8) != 0 {
        'q'
      } else {
        '-'
      },
    );
    println!("{:X?}", self.position_key);
  }

    let mut piece: Pieces = Pieces::Empty;
    let mut square: Squares;
  pub fn update_lists_material(&mut self, definitions: &Definitions) {
    let mut color: Colors;

    for square_120 in 0..BOARD_SQUARE_NUMBER {
      square = Squares::from_u32(self.pieces[square_120 as usize] as u32);
      if square != Squares::OffBoard {
        piece = Pieces::from_u32(self.pieces[square_120 as usize] as u32);
        if piece != Pieces::Empty {
          color = definition.piece_color()[piece as usize];

          if definition.piece_big()[piece as usize] == true {
            self.big_pieces_number[color as usize] += 1;
          }
          if definition.piece_major()[piece as usize] == true {
            self.major_pieces_number[color as usize] += 1;
          }
          if definition.piece_minor()[piece as usize] == true {
            self.minor_pieces_number[color as usize] += 1;
          }

          self.material[color as usize] += definition.piece_value()[piece as usize];
          self.pieces_list[piece as usize][self.actual_pieces_number[piece as usize] as usize] =
            square as i32;
          self.actual_pieces_number[piece as usize] += 1;
          
          if piece == Pieces::Wk {
            self.king_square[Colors::White as usize] = square;
          }
          if piece == Pieces::Bk {
            self.king_square[Colors::Black as usize] = square;
          }
        }
      }
    }
  }
}
