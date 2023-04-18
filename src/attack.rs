use crate::board::*;
use crate::definitions::*;
use crate::file_rank_to_square_120;

/**
 * Given an attacking piece position in this notation:
 *
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
 *
 * For example a Knight on E4 (55) attacks 8 board squares:
 * G3 (55 - 8 = 47), F2 (55 - 19 = 36), D2 (55 - 21 = 34), C3 (55 - 12 = 43),
 * C5 (55 + 8 = 63), D6 (55 + 19 = 74), F6 (55 + 21 = 76), G5 (55 + 12 = 67).
 */
const KNIGHT_DIRECTIONS: [i32; 8] = [-8, -19, -21, -12, 8, 19, 21, 12];
const ROOK_DIRECTIONS: [i32; 4] = [-1, -10, 1, 10];
const BISHOP_DIRECTIONS: [i32; 4] = [-9, -11, 9, 11];
const KING_DIRECTION: [i32; 8] = [-1, -10, -1, -10, -9, -11, 9, 11];

pub fn show_squares_attacked_by_side(side: Colors, board: &Board) {
  let mut square_120: i32;

  for rank in (ChessboardRanks::R1 as i32..=ChessboardRanks::R8 as i32).rev() {
    for file in ChessboardFiles::A as i32..=ChessboardFiles::H as i32 {
      square_120 = file_rank_to_square_120!(file, rank);
      if square_attacked(square_120, side, board) {
        print!("X");
      } else {
        print!("-");
      }
    }
    println!();
  }
  println!("\n");
}
