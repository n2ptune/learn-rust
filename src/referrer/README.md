## 참조자

함수의 인자로 넘긴 변수의 소유권은 그 함수에게 있어 스코프를 벗어나면 소유권이 없어진다. 소유권을 유지한 채로 사용하기 위해 함수에서 인자로 받은 데이터로 어떤 행위를 하다가 해당 인자의 소유권을 반환하는 형태의 트릭을 사용해서 소유권이 없어지는 문제를 우회할 수 있지만 러스트에서는 더 우아한 솔루션을 제공한다.

```rust
fn main() {
  let st = String::from("llllalalala");
  let st_len = len_str(st);
  // 문자열에 대한 소유권이 없으므로 아래 라인은 컴파일되지 않는다.
  println!("{}", st);
}

fn len_str(s: String) -> usize {
  // 문자열의 길이를 반환한다.
  s.len()
}
```

참조자를 이용하면 조금만 수정해서 소유권을 유지시킬 수 있다.

```rust
len_str(&st);

fn len_str(s: &String) ...
```

함수 호출부와 시그니처를 조금만 수정한다. 앰퍼샌드(&)를 넣으면 참조자라는 뜻이다. 하지만 참조자로는 해당하는 변수를 수정할 수 없다. 소유권이 없기 때문이다.

```rust
fn len_str(s: &String) {
  // 해당 변수의 소유권이 없기 때문에 수정할 수 없다.
  s.push_str("blah blah");
}
```

인자를 가변 참조자로 바꾸면 된다.

```rust
let mut st = String::from("lalala");
len_str(&mut st);

fn len_str(st: mut &String) {
  st.push_str("blah blah");
}
```

몇 가지 규칙을 지키면 참조자를 자유롭게 사용할 수 있다.

- 가변 참조자는 여러 개를 못가진다.
- 불변 참조자와 같이 사용이 불가능하다.

### dangle ref

러스트는 [댕글링 참조자](https://rinthel.github.io/rust-lang-book-ko/ch04-02-references-and-borrowing.html#%EB%8C%95%EA%B8%80%EB%A7%81-%EC%B0%B8%EC%A1%B0%EC%9E%90dangling-references)가 존재하지 않도록 보장한다.

```rust
fn d() -> &String {
  let s = String::from("blah");

  &s
} // 스코프를 벗어나면 s가 사라짐 근데 s의 참조자를 반환한다!?
```

위 코드는 정상적으로 컴파일되지 않는다. 참조자가 아닌 값 그대로를 반환해야 유지가 가능하다.
