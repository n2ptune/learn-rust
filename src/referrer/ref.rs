fn main() {
  let st = String::from("llllalalala");
  let st_len = len_str(&st);
  // 문자열에 대한 소유권이 없으므로 아래 라인은 컴파일되지 않는다.
  println!("{}", st);
}

fn len_str(s: &String) -> usize {
  // 문자열의 길이를 반환한다.
  s.len()
}
