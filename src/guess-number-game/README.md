## 숫자 추측하기 게임

1 ~ 100 사이의 난수를 생성하기 위해 외부 크레이트를 사용한다. 외부 크레이트에 의존한다는 의미로 `extern crate {name}`을 상단에 적는다.

```rust
extern crate rand;

use::std::io;
use::rand::Rng;
```

두 값을 비교하기 위해 `std::cmp::Ordering`을 이용한다.

```rust
match guess.cmp(&secret_number) {
  Ordering::Less => println!("Small"),
  Ordering::Greater => println!("Big"),
  Ordering::Equal => {
    println!("You Win!");
    break;
  }
}
```

반복문은 `loop` 키워드와 중괄호 안에 작성한다. 해당 반복문 안에서 `break`문을 만나면 반복문을 빠져나가는 것과 `continue`를 만나면 반복문의 처음 부분으로 돌아가는 것은 여타 다른 프로그래밍 언어와 비슷하다.

변수에 담긴 값의 타입을 추론해서 변수의 타입을 알아서 지정해줄 수 있다. 이후 해당 변수의 타입을 변경하고 싶거나할 때 아래와 같이 사용한다.

```rust
let guess: u32 = match guess.trim().parse() {
  Ok(n) => n,
  Err(_) => continue,
};
```

`parse` 함수의 반환 값이 `Result` 타입이므로 `Ok`, `Err`에 대한 처리를 해줄 수 있다.
