// Loops in Rust

// There are different types of loops like other languages
// 1. infinite loop
// 2. while loop
// 3. for range loop

pub fn run() {
    
    let mut count = 0;

    // Infinite loop
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 20 {
            break;
        }
    }

    // While Loop (FizzBuzz);
    count = 0;
    while count <= 100 {
        if count % 5 == 0 && count % 3 == 0 {
            println!("FizzBuzz!");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count);
        }
        count += 1;
    }

    // for range loop
    for i in 0..100 {
        if i % 5 == 0 && i % 3 == 0 {
            println!("FizzBuzz!");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}