
pub fn run() {
  let value = 0b1111_0101u8;
  println!("The value is {}", value);
  println!("The value is {:08b}", value);
  println!("The NOT value is {:08b}", !value);
}