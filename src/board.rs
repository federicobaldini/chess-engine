use crate::definitions::*;

pub fn reset_board(definitions: &mut Definitions, board: &mut Board) {
  *board.pieces() = [Squares::OffBoard as i32; BOARD_SQUARE_NUMBER];
  for index in 0..64 {
    board.pieces()[definitions.board_64_squares_in_120_squares_notation()[index] as usize] =
      Pieces::Empty as i32;
  }
  *board.big_pieces_number() = [0; 3];
  *board.major_pieces_number() = [0; 3];
  *board.minor_pieces_number() = [0; 3];
  *board.pawns() = [0u64; 3];
  *board.actual_pieces_number() = [0; 13];
  *board.king_square() = [Squares::NoSquare as i32, 2];
  *board.side() = Colors::Both;
  *board.en_passant_square() = Squares::NoSquare;
  *board.fifty_full_moves() = 0;
  *board.actual_half_moves() = 0;
  *board.total_half_moves() = 0;
  *board.castel_permission() = 0;
  *board.position_key() = 0u64;
}

// fen = Forsyth–Edwards Notation
pub fn parse_fen(definitions: &mut Definitions, board: &mut Board, fen: &str) {
  let mut rank: ChessboardRanks = ChessboardRanks::R8;
  let mut file: ChessboardFiles = ChessboardFiles::A;
  let mut piece: Pieces = Pieces::Empty;
  let mut count: i32;
  let mut char_index: usize = 0;
  let mut square_120: i32;
  let mut square_64: i32;
  let mut fen_char: char = fen.as_bytes()[0] as char;

  reset_board(definitions, board);

  while (rank as i32 >= ChessboardRanks::R1 as i32) && char_index < fen.len() /* fen_char != '\0' */ {
    count = 1;
    match fen_char {
      'c' => piece = Pieces::Bp,
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
        count = fen_char as i32 - '0' as i32
      }
      '/' | ' ' => {
        rank = ChessboardRanks::from_u32(rank as u32 - 1);
        file = ChessboardFiles::A;
        char_index += 1;
        // fen_char = fen.as_bytes()[char_index] as char;
      }
      _ => println!("FEN error"),
    }

    for _ in 0..count {
      square_64 = rank as i32 * 8 + file as i32;
      square_120 = definitions.board_64_squares_in_120_squares_notation()[square_64 as usize];
      if piece as i32 != Pieces::Empty as i32 {
        board.pieces()[square_120 as usize] = piece as i32;
      }
      file = ChessboardFiles::from_u32(file as u32 + 1);
    }
    char_index += 1;
    fen_char = fen.as_bytes()[char_index] as char;
  }
}
