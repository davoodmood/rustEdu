// Vectors are resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i8> = vec![1,2,3,4,5,6,7];

    println!("{:?}", numbers);
    
    // Re-assign value
    numbers[1] = 30;

    // Add on to vector
    numbers.push(12);
    numbers.push(55);


    // Pop off the last value
    numbers.pop();
    
    // Get a single value
    println!("Single Value: {}", numbers[0]);

    

    // Get Vector length
    println!("Vector Length: {}",numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes.", std::mem::size_of_val(&numbers));
    // println!("Address in memory is: {}", &numbers);

    // Get Slice
    let slice:  &[i8] = &numbers[1..3];
    println!("Slice: {:?}", slice);
    
    // loop through vector values
    // .iter() method is used when you want to read values in iteration
    for i in numbers.iter() {
        println!("Number: {}", i);
    }

    // loop & mutate values (similar to mapping in JS)
    // .iter_mut is used when you want to change values
    for i in numbers.iter_mut() {
        *i *= 2;
    }

}