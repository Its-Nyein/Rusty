// Collections in Rust [Vectors, UTF-8 Strings, HashMaps]

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main () {
    // Vector - Some Methods [push, pop, len, get(index) safe access(return option)]
    let mut _v: Vec<i32> = Vec::new();

    let mut _v: Vec<i32> = vec![1, 2, 3];
    _v.push(4);
    _v.push(5);
    let second: i32 = _v[1];

    println!("{:?}", _v);
    println!("The second element is: {second}");

    // Strings - UTF-8 Strings
    // Some Methods [push_str, len, replace, .to_uppercase, .to_lowercase]
    let mut _s: String = String::new();
    let mut _s: String = String::from("Hello");
    _s.push_str(", world!");

    println!("{:?}", _s);

    // HashMap - Key-Value Pairs
    // Like Objects and Maps in JS
    // Some Methods [insert, get(key), remove(key), contains_key(key)]

    
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores.get(&String::from("Blue")));

    // Hast<T> - Set of Unique Values
    // No duplicates allowed
    // Useful for: fast membership checks (contains())

    let mut set = HashSet::new();
    set.insert("apple");
    set.insert("banana");
    set.insert("apple"); // Ignored

    println!("{:?}", set);

    // vecDeque - Double-ended queue
    let mut queue: VecDeque<i32> = VecDeque::new();
    queue.push_back(1);
    queue.push_front(0);

    println!("{:?}", queue);

    // Vec<T>, String, HashMap, HashSet, VecDeque are Growable
    // Vec, String and VecDeque are ordered
    // Unique: HashMap -> Keys, HashSet
    // Key-Value: HashMap
}

pub fn run () {
    main();
}