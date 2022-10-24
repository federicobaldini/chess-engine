mod definitions;
mod init;

fn main() {
  let mut definitions = definitions::Definitions::new();
  
  init::init(&mut definitions);

  for square in 0..definitions.square_120_to_square_64().len() {
    if square % 10 == 0 {
      println!();
    }
    print!(
      "{} ",
      definitions.square_120_to_square_64()[square as usize]
    );
  }

  println!();
  println!();

  for square in 0..definitions.square_64_to_square_120().len() {
    if square % 8 == 0 {
      println!();
    }
    print!(
      "{} ",
      definitions.square_64_to_square_120()[square as usize]
    );
  }
}
