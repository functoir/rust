/// Loops in rust
/// 
/// Author: siavava <amittaijoel@outlook.com>

extern crate rand;

use rand::Rng;
fn main() {

  let mut num = 10;

  while num > 0 {
    println!("num: {}", num);
    num = num - 1;
  }

  println!("The End.");

  let arr = ["what", "else", "is", "going", "on", "here"];

  for word in arr.iter() {
    print!("{} ", word);
  }
  println!();


  for _iter in 1..10 {
    let num = rand::thread_rng().gen_range(-100..=100);

    let cels = f_to_c(num as f64);

    let fahr = c_to_f(cels);
  
    println!("fahrenheit = {}, celsius = {}", num, cels);
    println!("celsius = {}, fahrenheit = {}", cels, fahr);
  }


}

/// convert fahrenheit to celsius
fn f_to_c(temp: f64) -> f64 {

  // convert fahrenheit to celcius
  let temp: f64 = ((temp - 32.0) * 5.0 / 9.0) as f64;

  return temp;
}

/// convert celsius to fahrenheit
fn c_to_f(temp: f64) -> f64 {

  // convert celcius to fahrenheit

  let temp: f64 = temp * 1.8 + 32.0;

  return temp;
}
