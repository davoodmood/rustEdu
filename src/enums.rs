//  Enums - types which have a few definite values

// starting enum with a Capital letter
enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar (m: Movement) {
    // Perform action depending on movemnet info
    // match is similar to Switch statement in other langs
    match m {
        Movement::Up => println!("Avatar Moving Up!"),
        Movement::Down => println!("Avatar Moving Down!"),
        Movement::Left => println!("Avatar Moving Left!"),
        Movement::Right => println!("Avatar Moving Right!")
    }
}

pub fn run() {
    let avatar1 = Movement::Up;
    let avatar2 = Movement::Left;
    let avatar3 = Movement::Down;
    let avatar4 = Movement::Right;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}