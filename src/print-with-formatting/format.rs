fn main() {
  // 중괄호 안은 주어진 인자로 대치된다.
  println!("{} World!", "Hello");
  // -> Hello World!

  // 인자의 위치를 번호로 매겨 출력할 수 있다.
  println!("{0} {1}! {1} {0}!", "Hello", "World");
  // -> Hello World! World Hello!

  // 인자에 이름을 줄 수 있다.
  println!("{first} {second}!", first="Hello", second="World");
  // -> Hello World!

  // 콜론 뒤에 형식을 지정해서 출력할 수 있다.
  println!("{:b} 이 수와 {} 이 수는 같은 수 입니다.", 2, 2);
  // -> 10 이 수와 2 이 수는 같은 수 입니다.

  // 폭을 지정해서 출력할 수 있다.
  println!("{n:>width$}", n=3, width=10);
  // 9개의 공백 뒤 3이 출력된다.
  // 공백 대신 0을 대치시킬 수 있다.
  println!("{n:>0width$}", n=3, width=10);
  // -> 0000000003 (공백 대신 0이 채워진다.)

  let pi = 3.141592;

  // 소수점 이하 출력 (3 자리 수 까지만)
  println!("PI: ${p:.3}", p=pi);
}