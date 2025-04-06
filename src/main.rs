mod conditions;
mod datatypes;
mod error_handling;
mod functions;
mod ownership;
mod variables;

fn main() {
    // fn is function main is entry point
    println!("Hello, Rust!"); // print to console and (!) means it is macro not function

    variables::run();
    datatypes::run();
    functions::run();
    conditions::run();
    ownership::run();
    error_handling::run();
}
