pub fn run() {
  let a = 13;
  let b = 2.3;
  let c: f32 = 120.0;
  let expected: f64 = 45.1;

  let average: f64 = avg(a as f64, b as f64, c as f64);
  assert_eq!(average, expected);
  println!("Test passed!");
}

fn avg(a: f64, b: f64, c: f64) -> f64 {
  let sum = a + b + c;
  return sum / 3.0;
}