// Arrays - Fixed list where elements are the same data types

pub fn run() {
    
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Get a single value
    println!("Single Value: {}", numbers[0]);

    // Get array length
    println!("Array Length: {}",numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes.", std::mem::size_of_val(&numbers));

    // Get Slice
    let slice:  &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);
}