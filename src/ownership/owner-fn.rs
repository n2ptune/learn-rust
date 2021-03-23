fn main() {
  // 텍스트의 소유권이 여기있음
  let text = String::from("Hello");

  // 함수 호출시 소유권이 넘어감
  trans_hello_to_world(text);
  // 함수 종료시 hello가 메모리에서 해제되어 참조 불가
  // println!("{}", text);

  // 새로 변수를 만들고 소유권을 얻음
  let text = String::from("Hello");
  // 소유권을 넘기고 text는 사라지지만 새로운 문자열 반환
  let world = get_trans_hello_to_world(text);

  // world 문자열의 소유권이 이 문맥에서 존재하므로 참조 가능
  println!("{}", world);
}

fn trans_hello_to_world(hello: String) {
  // 함수 호출시 소유권이 이 함수에 있음
  let world = hello.replace("Hello", "World");
  // 스코프를 벗어나면 hello의 메모리가 해제됨
}

fn get_trans_hello_to_world(hello: String) -> String {
  // 소유권을 받고 다른 String 타입을 만들어 반환함
  let world = hello.replace("Hello", "World");
  world
}
