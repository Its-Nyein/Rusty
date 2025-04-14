// collect command line arguments into a vector and print them
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}

// when we run cargo run -- sample sample.txt
// [src/main.rs:5:5] args = ["target/debug/minigrep","sample","sample.txt",] "target/debug/minigrep" is the name of our binary
