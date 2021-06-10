/// Guessing game
/// 
/// Prompts user for number guesses
/// 
/// Prints number
/// 
/// Checks if it matches a given expected number.
/// 
/// Author: siavava <amittaijoel@outlook.com>

/// external crate
extern crate rand;

/// utilities
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

  let secret_number = rand::thread_rng().gen_range(1..101);

  // loop
  loop {
    
    // Prompt
    print!("Guess the number:\n");
    println!("Please enter your guess.");
  
    // Read in input
    let mut guess = String::new();

    // Read in line from stdin, expect exception
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    // Cast input to integer, handle exception case
    let guess: u32 = match guess.trim().parse() {

      // if okay, save num
      Ok(num) => num,

      // if not okay, print error message and continue to next iteration
      Err(_) => {
        println!("Please enter a valid number.");
        continue;
      }
    };
  
    // Print the input
    println!("You guessed: {}", guess);
  
    // match statement -- akin to a switch
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}
