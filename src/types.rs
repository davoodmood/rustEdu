/*
Primitive Types --
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (num of bits in taken by memory)
Floats: f32, f64
Boolean (bool)
Characters (char): 1 character, chars are different with String in Rust
Tuples
Arrays: in Rust are fixed length 
*/

// Rust is a statically typed language, which means that it must know the types of all variables 
// at compile time, however, the compiler can usually infer what type we want to use based on 
// the value and how we use it.


pub fn run() {
    // Default is "i32"
    let _x = 1;

    // Default is "f64"
    let _y = 2.5;

    // Add explicit type
    let _z: i64 = 4565468765;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;

    // Get boolean from expression
    let is_greater = 10 > 5;

    // 
    let a1 = 'a'; // single quotes are used to represent char type in Rust. char types are UNICODE format.
    let face = '\u{1F600}'; // Unicode so you can also print an emoji.

    println!("{:?}", (_x, _y, _z, is_active, is_greater, face));

}