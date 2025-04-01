// Conditions and Loops in Rust

fn main () {
    let num = 10;

    if num < 20 {
        println!("Number is Smaller")
    } else {
        println!("Number is Greater")
    }

    // Loops in Rust
    // Rust supports for, while, and loop (infinite loop)

    for i in 1..5 { // Range: 1 to 4
        println!("{}", i)
    }

    let mut count = 0;
    while count < 5 {
        println!("{}", count);
        count += 1;
    }

    // loop {
    //     println!("This will run forever!");
    // }


    // Example1: iterating over array
    let fruits = ["Apple", "Banana", "Orange"];
    for fruit in fruits {
        println!("I love {}", fruit);
    }

    // Example2: iterating with index(enumerate())
    let names = ["Dave", "John", "Charlie"];
    for (index, name) in names.iter().enumerate() {
        println!("Index: {}, Name: {}", index, name);
    }

    // Example3: reverse loop(rev())
    for num in (1..=5).rev() {
        println!("{}", num)
    }

    // Example4: skipping loop(step_by())
    for num in (1..10).step_by(2) { // Skips every 2nd numbers
        println!("{}", num)
    }

    // Example5: break and continue an iteration
    let mut number = 0;
    loop {
        number += 1;

        if number == 3 {
            println!("Skipping number 3...");
            continue; // skipping number 3
        }

        println!("Number: {}", number);

        if number == 5 {
            break; // exit the loop when number reach 5
        }
    }

    // Example6: nested infinite loops with labels
    // Rust allow nested loops with label('label_name)

    'outer: loop {
        println!("Outer loop is running...");

        loop {
            println!("Inner loop in running...");
            break 'outer; // Exited both loops
        }
    }
    println!("Exited both loops!")
}

pub fn run () {
    main()
}