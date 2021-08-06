/// Functions on numbers
/// 
/// Fibonacci sequence
/// 
/// Author: siavava <amittaijoel@outlook.com>

fn main() {
  let x = five();
  let f: i64 = fib(x);
  println!("x = {}, fib({}) = {}", x, x, f);

  println!("validity: {}", check_number(f, 5));

  foo(&["hello", "world", "I", "am", "arguments"]);
}

fn five() -> i32 {
  5
}

fn fib(n: i32) -> i64 {
  // const n: i64 = n;
  let mut i: i64 = 0;
  let mut j: i64 = 1;

  for iter in 1..=n {
    j = i + j;
    i = j - i;
    println!("iter = {}, fib = {}", iter, i);
  }
  return i;
}

fn check_number(num: i64, target: i64) -> bool {
  num == target
}

fn foo(args: &[&str]) {
  for arg in args {
    println!("{}", arg);
  }
}
