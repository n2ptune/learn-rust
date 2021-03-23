## 소유권

러스트를 정말 빛내게 하는 기능 중 하나인 소유권(Ownership) 기능은 이해하기 그렇게 쉬운 건 아닌 것 같다. 다만 이 개념을 왜 만들어내서 러스트에 적용했는지는 알기 쉽다. C 혹은 C++은 개발자가 운영체제로부터 메모리를 할당받고 필요없어졌을 때 해제해야한다. 할당 후 해제하지 않으면 메모리 누수가 일어나므로 프로그램 전체 성능이 저하될 수 있다. 그래서 C와 C++로부터 파생된 많은 언어는 이 문제를 해결하기 위해 가비지 컬렉션이라는 기능을 가지고 있다. 더이상 사용하지 않는 메모리를 언어 차원에서 해제시킨다. 하지만 러스트에서는 이 가비지 컬렉션이 런타임에서 계속 돌기 때문에 오너쉽 모델이란 개념을 고안해낸 것 같다.

### 스택과 힙

스택은 고정된 크기를 가지고 먼저 푸쉬된 데이터가 나중에 나가는 구조를 갖는 자료 구조이다. 스택이 빠른 이유는 데이터 흐름이 한 곳으로만 흐르기 때문이다. 복잡한 배열에서 인덱스 값으로 참조해서 해당 메모리 주소의 원소를 꺼내줄 필요 없이 항상 꼭대기에 있는 원소를 꺼내기 때문이다.

힙은 운영체제에게 장소를 임대하는 개념이다. 빌려달라고 하면 운영체제는 공간을 보고 빌려줄 만한 곳을 골라 그 곳을 참조할 수 있는 포인터를 반환한다.

### 소유권 규칙

- 러스트에서 값은 해당 값의 오너(owner)라고 불리는 변수를 갖고 있다.
- 한번에 딱 하나의 오너만 존재할 수 있다. (한 변수당 오직 오너가 한 개라는 소리인듯)
- 오너가 스코프 밖으로 벗어날 때 값은 버려진다.

```rust
fn main() {
  // main 함수 안 스코프
  let p = 1;
  {
    // 블록 레벨 스코프
    let x = 2;
    // 블록을 벗어나면 값이 버려짐
  }
  println!("{}, {}", p, x);
}
```

위 코드는 컴파일 오류가 난다. 블록 레벨에서 `x`라는 변수는 존재하지만 블록을 벗어났기 때문에 `x`는 버려졌다. (자바스크립트로 생각하면 이해하기 쉬운 것 같다.)

### 문자열 리터럴과 String 타입

```rust
let text = "Hello";
```

`text`는 컴파일되면 바이너리에 직접 Hello 라는 문구가 찍혀 들어간다. 즉 하드코딩된 텍스트가 컴파일시에 Hello라는 문자 그대로 바이너리에 찍힌다는 소리다. 반면 사용자로부터 입력을 받거나 런타임시 달라질 수 있는 문자열에 대해서는 다르게 처리한다.

러스트는 컴파일시 크기를 알 수 없는 텍스트나 런타임시 크기가 변할 수도 있는 텍스트를 바이너리에 찍을 수 없다. 그렇기 때문에 `String` 타입을 제공한다. 이 타입은 문자열이 런타임시에 변경될 수 있고 컴파일시 크기를 알 수 없음에도 컴파일된다. 그렇게 설계되었다.

```rust
let text = String::from("Hello");
```

마찬가지로, `text`는 스코프를 벗어나면 자동으로 해제된다. (더이상 접근할 수 없음) 그리고 문자열 리터럴은 런타임시에 값을 변경하거나 문자 일부분을 삭제하거나 추가하는 것이 안된다.

### 이동

자바스크립트에서 한 객체를 다른 변수에 대입할 경우 얕은 복사가 되는데, 즉 두 변수 모두 같은 객체를 바라보기 때문에 두 변수 중 하나의 변수에서라도 객체를 수정하게 되면 두 변수가 바라보는 객체가 수정되기 때문에 두 변수가 표현하는 값이 모두 달라진다.

자바스크립트의 경우 가비지 컬렉터가 있기 때문에, 만약 이 객체가 더이상 닿지 않는 곳에 존재한다면 삭제시켜버린다. 러스트에서 문제로 삼는 건 바로 두 번 메모리가 해제되는 경우이다. 러스트는 이동 이라는 개념으로 이 문제를 해결한다.

```rust
let a = String::from("Hello World");
let b = a;
```

변수 `a`는 더이상 유효하지 않다. `b`로 참조하는 포인터가 이동되었기 때문에 `a`를 참조할 수 없고, `b`가 스코프를 벗어나게 되면 바라보던 문자열 포인터는 메모리에서 해제된다.

`clone`이라는 메서드로 힙 데이터까지 복사시키는 메서드를 러스트에서 지원한다. 여타 다른 언어에서 깊은 복사라는 개념이다.

### 복사

컴파일시 크기가 정해져있는 타입은 값을 복사시켜도 이전의 변수가 유효하다. 스택에 저장되기 때문이다. 깊은 복사와 얕은 복사를 이용했을 때의 차이점이 전혀 없기 때문이다.

아래와 같은 타입은 스택에 저장시킬 수 있는 타입이다.

- `u8`, `u16`, ...
- `f32`, `f64`
- `boolean`
- 위 타입들로만 구성된 튜플 e.g) `(u32, u64, f32, boolean)`

### 소유권과 함수

함수의 인자로 들어오는 변수도 소유권의 대상이다.

```rust

```