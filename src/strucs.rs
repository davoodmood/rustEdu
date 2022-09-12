// Structs - Used to create custom data types (somehow similar to classes in other langs)


// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8 
 }
// Tuple Struct
struct RGB(u8,u8,u8);

// creating a Person struct 
struct Person {
    first_name: String,
    last_name: String
}
// implementing methods into the Person struct and a constructor
impl Person {
    // Construct Person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Modify Last Name
    fn change_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // traditional struct
    let mut color = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    color.red = 200;

    println!("Color: {} {} {}", color.red, color.green, color.blue);

    // tuple struct
    let mut c = RGB(255,0,0);

    c.0 = 200;

    println!("Color w/ tuple struct: {} {} {}", c.0, c.1, c.2);


    // creating a new person by its constructor

    let mut dave = Person::new("David", "Mood");
    println!("Person: {} {}", dave.first_name, dave.last_name);
    println!("Person Fullname : {}", dave.full_name());
    
    dave.change_last_name("H. Mood");
    println!("Person Fullname : {}", dave.full_name());

    println!("Person Tuple: {:?}", dave.to_tuple());
}