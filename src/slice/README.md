## 슬라이스

형태는 자바스크립트에서 배열의 인덱스 혹은 객체의 키 값으로 값을 읽어들이는 것과 비슷한 형태로 사용한다.

```rust
fn main() {
  // 슬라이스 대상
  let mut target_str = String::from("Hello World");
  // 슬라이스
  let first = &target_str[0..5]; // Hello를 가진다.
  let second = &target_str[6..11]; // World를 가진다.

  println!("{0}, {1}", first, second);
  // Hello, World
}
```

슬라이싱이란 개념이 무언가 파이썬에서 반복 가능한 객체에 대해서 집합에서 일부분만 가져오는 개념으로 이해했다. 여기서 사용된 문자열을 슬라이싱 하는건 `String`의 일부분을 참조하는 것과 비슷하다. 예를 들어 `target_str`에 대한 포인터가 생성되면, `first`라는 변수는 `target_str`의 포인터 일부분을 참조하는 것이 된다.

슬라이싱은 몇 가지 규칙이 있다.

- 인덱스를 0에서 시작하면 시작 인덱스를 생략할 수 있다. `&target_str[..2]`
- 끝나는 인덱스를 생략하면 끝까지 슬라이싱한다. `&target_str[2..]`
- 시작 인덱스와 끝나는 인덱스를 생략하면 처음부터 끝까지 슬라이싱한다. `&target[..]`

