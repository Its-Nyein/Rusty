// Ownership, borrowing and referances in Rust
// Rust manages memory safely through ownership rules instead of garbage collection.

// Example1
// In Rust, variables get "moved" instead of being copied.
// After passing name to say_hello, it's no longer available in main().

fn main() {
    let name = String::from("Rust"); // Ownership of string
    say_hello(name); // Ownership moves to function
    // println!("{}", name); ❌ Error! 'name' is moved
}

fn say_hello(name: String) {
    println!("Hello, {}", name);
}

// Example2
// Borrowing insted of moving
// & means borrowing (read-only access, no transfer of ownership)
// This allows multiple functions to use the same variable without errors.

fn borrowing() {
    let name = String::from("Rusty");
    say_hola(&name); // Borrowing (doesn't move ownership)
    println!("Hello, {}", name); // Work fine
}

fn say_hola(name: &String) {
    println!("Hello, {}", name); // Work fine
}

// Example3
// There can be only one owner at a time

fn owner() {
    let s1 = String::from("Rust");
    let s2 = s1;
    // println!("s1: {}", s1); // value borrowed here after move
    println!("s2: {}", s2);
}

// Example4
// Demostration on one mutable ref or many immutable ref

fn account() {
    let mut account = BankAccount {
        owner: "John Doe".to_string(),
        balance: 1500.58,
    };

    // Immutable borrow check_balance
    account.check_balance();

    // Mutable borrow to withdraw
    account.withdraw(45.99);
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdraw {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!(
            "Account owned by {} has a balance {}",
            self.owner, self.balance
        );
    }
}

// umm forgot to demonstrate about struct enums
// Structs & Enums (Like Objects in JavaScript)

struct User {
    name: String,
    age: u8,
}

fn user() {
    let user = User {
        name: String::from("Alice"),
        age: 25,
    };
    println!("User: {} is {} years old", user.name, user.age);
}

pub fn run() {
    main();
    borrowing();
    owner();
    account();
    user();
}
