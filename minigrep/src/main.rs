// collect command line arguments into a vector and print them
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];

    println!("Query: {}", query);
    println!("File path: {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should be able to read the file");

    println!("With text:\n{}", contents);
}
