// Variables hold primitive data or references to data
// Variables are immutable by default       !important
// Rust is a block-scoped language

pub fn run() {
  let name = "Dave";
  let mut age = 33;
  println!("My name is {} and I am {}", name, age);
  age = 34;
  println!("My name is {} and I am {}", name, age);

  // Define Constant: name of "const" variable is usually ALL UpperCase
  const ID: i32 = 001; // i32 is a interger32 type 
  println!("ID: {}", ID);

  // Assing multiple vars
  let (my_name, my_age) = ("Dave", 33);
  println!("{} is {}", my_name, my_age);
}