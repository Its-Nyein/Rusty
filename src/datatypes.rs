// Datatypes in Rust

// Primitive Datatypes
// int, float, bool, char

fn primitive_types() {
    // Rust has signed(+ and -) and unsigned integer(only +)
    // i8, i16, i32, i64, i128: Signed integers.
    // u8, u16, u32, u64, u128: Unsigned integers.
    let integer: i32 = -25;
    let unsigned_int: u32 = 30;
    let float: f64 = 9.99;
    let is_rust_fun: bool = true;
    let message: char = 'N'; // Single character

    println!("Integer: {}", integer);
    println!("Unsigned Integer: {}", unsigned_int);
    println!("Float: {}", float);
    println!("Boolean: {}", is_rust_fun);
    println!("Character: {}", message);
}

// Compound Datatypes
// arrays, tuples, slices, and strings
fn compound_types() {
    // Arrays
    let number: [i32; 3] = [1, 2, 3];
    println!("Array: {:?}", number);

    // Tuples
    let human = ("Alice", 20, false);
    println!("Tuple: {:?}", human);

    // Slices
    let slice: &[i32] = &number[0..2];
    println!("Slice: {:?}", slice);

    // Strings
    let string: String = String::from("Hello, Rust!");
    println!("String: {}", string);
}

pub fn run() {
    // Primitive Types
    primitive_types();

    // Compound Types
    compound_types();

    // Integer Limits
    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;

    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", i);

    // Floats
    let pi: f64 = 3.14159;
    println!("Value of PI: {}", pi);

    // Arrays
    let number: [i32; 3] = [1, 2, 3];
    println!("Number array: {:?}", number);

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits: {:?}", fruits);
    println!("First fruit: {}", fruits[0]);

    // Tuples
    let human: (String, i32, bool) = ("Alice".to_string(), 20, false);
    println!("Human tuple: {:?}", human);

    let mix_tuples = ("Bob", 22, true, [1, 2, 3]);
    println!("Mixed tuple: {:?}", mix_tuples);

    // Slices
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number slices: {:?}", number_slices);

    let name_slices: &[&str] = &["BoBo", "Dave", "Nyein"];
    println!("Name slices: {:?}", name_slices);

    let animal_slices: &[String] = &[
        "Lion".to_string(),
        "Elephant".to_string(),
        "Crocodile".to_string(),
    ];
    println!("Animal slices: {:?}", animal_slices);

    // String -> mutable, stored in Heap (Dynamic)
    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Cold.");
    println!("Stone cold says: {}", stone_cold);

    // &str (String slice) -> immutable, stored in Stack (Static)
    let string: String = String::from("Hello, Rust");
    let slice: &str = &string[0..5];
    println!("Slice value: {}", slice);
}