fn main() {
  let tempera = 32;
  let flt = convert_to_f(tempera);

  println!("{}", flt);
}

fn convert_to_cel(x: u32) -> f64 {
  (x as f64 - 32.0) * 5.0 / 9.0
}

fn convert_to_f(x: u32) -> f64 {
  (x as f64 * 9.0 / 5.0) + 32.0
}