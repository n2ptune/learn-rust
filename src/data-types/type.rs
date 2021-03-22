fn main() {
  let guess = "42";
  println!("{}", guess);

  let guess: u32 = 132_000;
  println!("{}", guess);

  let guess_hex: u32 = 0xff;
  let guess_oct: u32 = 0o77;
  println!("{}, {}", guess_hex, guess_oct);

  let union_type: (u32, f64, char) = (32, 64.4, 'Z');
  println!("{}, {}, {}", union_type.0, union_type.1, union_type.2);
}
