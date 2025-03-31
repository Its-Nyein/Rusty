// Functions in Rust.
// Rust does not have hoisting in the JavaScript sense.
// Instead, Rust performs full compilation before execution, allowing functions to be used before they are declared in the code.

fn add (x: i32, y: i32) -> i32 {
    return x + y; // otherwise just simply x + y
}

fn human_id (name: &str, age: i32, height: f32) {
    println!("{} is {} years old and {} cm in height.", name, age, height);
}

fn main () {
    let sum = add(10, 20);
    println!("The sum: {}", sum);
    human_id("JohnDoe", 23, 185.3);
    let _x: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty
    };
    println!("Result is: {}", _x);
}

pub fn run () {
    main();
}