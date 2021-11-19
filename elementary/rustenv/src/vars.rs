/// Author: siavava <amittaijoel@outlook.com>
/// Created: 2021-08-06
pub fn run() {
  let native_str = "Here is a native string";
  println!("{}", native_str);
  let mut other_native_str = "Here is a mutable native string";
  println!("{}", other_native_str);
  other_native_str = "we";
  println!("{}", other_native_str);
}

