// Variables in Rust

pub fn run () {
    let name = "N0taiL"; // Immutable bu default
    let mut age = 24; // Mutable variable (mut) allows to change values

    // Updating variables
    println!("{} is {} years old.", name, age);
    age = 25;
    println!("Now, {} is {} years old.", name, age);

    // Constants must have a type and cannot be changed
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Shadowing
    let age = "Twenty-Five"; // Shadowing allows re-declaring a variable with the same name
    println!("Age as a string: {}", age);
    let age = age.len();
    println!("Legnth of 'age' string: {}", age);

    // Multiple variables
    let (x, y, z) = (1, 2, 3); // Declaring multiple variables at once
    println!("x = {}, y = {}, z = {}", x, y, z);
}