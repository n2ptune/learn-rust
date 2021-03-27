fn first_word(st: &str) -> &str {
  let bytes = st.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &st[0..i];
    }
  }

  &st[..]
}

fn main() {
  // 슬라이스 대상
  let mut target_str = String::from("Hello World");
  // 슬라이스
  let first = &target_str[0..5]; // Hello를 가진다.
  let second = &target_str[6..11]; // World를 가진다.

  println!("{0}, {1}", first, second);

  target_str = String::from("Rust is programming lanauge");

  let q = first_word(&target_str);

  println!("{}", q);
}