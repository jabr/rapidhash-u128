use rapidhash::*;
use std::env;

fn main() {
  let arg = env::args().nth(1).unwrap_or("".to_string());
  println!("arg = {arg}");
  let value = u128::from_str_radix(&arg, 16).unwrap_or(1);
  println!("RapidHash({value}) = {}", hash(value));
}
