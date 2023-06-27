mod Memory;

use Memory::memory_parser;

fn main() {
  match memory_parser("00 10 10 0F", 30, "javaw", None, Some("10 10 0F 00")) {
      Ok(q) => println!("scanned {} and changed {}", q.0, q.1),
      Err(e) => println!("an error occurred! {}",e)
  }
  match memory_parser("00 10 10 0F", 30 "javaw", Some(3.63), None) {
      Ok(q) => println!("scanned {} and changed {}", q.0,q.1),
      Err(e) => println!("an error occurred! {}",e),
  }
}
