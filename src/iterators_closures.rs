use std::thread;
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Investory {
    shirts: Vec<ShirtColor>,
}

impl Investory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Investory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "User preference: {:?}, Giveaway: {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "User preference: {:?}, Giveaway: {:?}",
        user_pref2, giveaway2
    );

    // Defining and calling a closure that captures an immutable reference
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    // Defining and calling a closure that captures a mutable reference
    let mut mut_list = vec![1, 2, 3];
    println!("Before defining closure: {mut_list:?}");

    let mut borrows_mutably = || mut_list.push(7);

    borrows_mutably();
    println!("After calling closure: {mut_list:?}");

    // Using move to force the closure for the thread to take ownership of list
    let move_list = vec![1, 2, 3];
    println!("Before defining closure: {move_list:?}");

    thread::spawn(move || println!("From thread: {move_list:?}"))
        .join()
        .unwrap();
}

pub fn run() {
    main();
}
