pub fn run() {
  /* 
   comment? 
   This is okay
  */
  println!("Hello, from print.rs!");

  let (name, age) = ("Amittai", 22);
  let place = "Kenya";
  println!("{} is {} years old.", name, age); 

  println!("{0} is {1} years old and {0} is from {2}.", name, age, place);

  let randnum = 10;

  println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", randnum, randnum, randnum);

}