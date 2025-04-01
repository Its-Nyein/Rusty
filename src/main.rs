mod variables;
mod datatypes;
mod functions;
mod conditions;

fn main() {
    // fn is function main is entry point
    println!("Hello, Rust!"); // print to console and (!) means it is macro not function

    variables::run();
    datatypes::run();
    functions::run();
    conditions::run();
}
