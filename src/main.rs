mod bitboards;
mod board;
mod definitions;
mod hashkeys;
mod init;
use rand::Rng;

fn main() {
  let mut definitions = definitions::Definitions::new();

  /*
  let mut rng = rand::thread_rng();

  let piece_one: i32 = rng.gen();
  let piece_two: i32 = rng.gen();
  let piece_three: i32 = rng.gen();
  let piece_four: i32 = rng.gen();

  println!("Piece one: {:X?}", piece_one);
  println!("Piece two: {:X?}", piece_two);
  println!("Piece three: {:X?}", piece_three);
  println!("Piece four: {:X?}", piece_four);

  let key = piece_one ^ piece_two ^ piece_four;
  let mut temp_key = piece_one;
  temp_key ^= piece_three;
  temp_key ^= piece_four;
  temp_key ^= piece_two;

  println!("Key: {:X?}", key);
  println!("Temp key: {:X?}", temp_key);

  temp_key ^= piece_three;
  println!("(three out) temp key: {:X?}", temp_key);

  temp_key ^= piece_three;
  println!("(three in) temp key: {:X?}", temp_key);
  */

  init::init(&mut definitions);
}
