pub fn run() {
    // print to console
    println!("Hello from the print.rs file!");

    // Basid formating
    println!("Number: {}", 1);

    // Basic formating with multiple Arguments
    println!("Bob is {} when {} is on the {}.", "happy", "he", "journey");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Dave", "Mash", "code");

    // Named Arguments
    println!("{name} likes to play {activity}", name = "John", activity = "Baseball");

    // Placeholder Traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!(" 10 + 15 = {}", 10 + 15);
}