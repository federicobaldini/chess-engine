mod bitboards;
mod board;
mod definitions;
mod hashkeys;
mod init;
use rand::Rng;

fn main() {
  let mut definitions = definitions::Definitions::new();
  init::init(&mut definitions);
}
