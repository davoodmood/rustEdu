// 

use std::env;

pub fn run () {
    let args: Vec<String> = env::args().collect();
    let command = if args.len() > 1 {args[1].clone()} else {"no_command".to_string()};
    let rank: String = "1st".to_string();

    // println!("Args: {}", command);

    if command == "help" {
        println!("Hi, how can i help you?");
    } else if command == "rank" {
        println!("Your Rank is: {}", rank);
    } else if command == "no_command" {} else {
        println!("That is not a valid command");
    }
}