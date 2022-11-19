mod bitboards;
mod board;
mod definitions;
mod hashkeys;

fn main() {
  let mut definitions = definitions::Definitions::new();

  definitions.init();
}
