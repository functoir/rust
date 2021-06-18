/// Ownership scopes in Rust
/// 
/// Author: siavava <amittaijoel@outlook.com>

fn main() {
  let some_string = String::from("This is a random string.");

  println!("{}", some_string);

  let mut some_string = gives_back(some_string);
  
  println!("{}", some_string);

  does_not_give_back(&some_string);

  println!("{}", some_string);

  change(&mut some_string);

  println!("{}", some_string);





}

fn gives_back(s: String) -> String {
  s
}

fn does_not_give_back(s: &String) {
  println!("Received: {}, won't give back.", s);
}

fn change(s: &mut String) {
  s.push_str("... AND here is an eddendum!");
}
