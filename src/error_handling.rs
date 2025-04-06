// Error handling in Rust [2 approaches]

// Approach 1
enum Option<T> {
    // Define the generic option type
    Some(T), // Represents a value
    None,    // Represents no value
}

// Approach 2
enum Result<T, E> {
    // Define the generic result type
    Ok(T),  // Represents success
    Err(E), // Represents an error
}

fn devide_option(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        return Option::None;
    } else {
        return Option::Some(numerator / denominator);
    }
}

fn devide_result(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        return Result::Err("Cann't devide by zero!".to_string());
    } else {
        return Result::Ok(numerator / denominator);
    }
}

fn main() {
    let result_option = devide_option(10.0, 2.0);
    match result_option {
        Option::Some(value) => println!("Result: {}", value),
        Option::None => println!("Error: Cann't devide by zero!"),
    }

    let result_result = devide_result(10.0, 0.0);
    match result_result {
        Result::Ok(value) => println!("Result: {}", value),
        Result::Err(error) => println!("Error: {}", error),
    }
    {}
}

pub fn run() {
    main();
}
