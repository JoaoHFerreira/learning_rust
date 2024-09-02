use std::io::stdin;
use std::collections::HashMap;

fn scan() {
    println!("Scanning the file");
    let mut my_string = String::new();
    let _ = stdin().read_line(&mut my_string).unwrap();
    println!("{}", my_string);
}

fn finding_elements_in_list(target_number: usize) {
    let numbers = vec![1, 2, 3, 4];
    println!("The number at vector position {} is {}", target_number, numbers[target_number]);

    for n in &numbers { 
        if *n == target_number {
            println!("Your chosen number is {}", n);
            return;
        }
    }

    println!("The value {} was not found", target_number);
}

fn list_iteration_example() {
    let fruits = vec!["apple", "banana", "orange"];
    for fruit in &fruits {
        println!("{}", fruit);
    }
}

fn map_reduce_filter_example() {
    let words = vec!["rust", "is", "awesome"];
    let lengths: Vec<usize> = words.iter().map(|word| word.len()).collect();
    let total_length: usize = lengths.iter().sum();
    let long_words: Vec<&str> = words.iter().filter(|word| word.len() > 3).map(|word| *word).collect();

    println!("Lengths: {:?}", lengths);
    println!("Total length: {}", total_length);
    println!("Long words: {:?}", long_words);
}

fn hash_map_example() {
    let mut scores = HashMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 82);

    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
}

fn main() {
    println!("1: Scan file");
    println!("2: Find given element in list");
    println!("3: List iteration example");
    println!("4: Map, reduce, filter on a string list");
    println!("5: HashMap example");
    println!("\n\n");

    let mut input = String::new();
    let _ = stdin().read_line(&mut input).unwrap();
    let number: usize = input.trim().parse().unwrap();

    match number {
        1 => scan(),
        2 => finding_elements_in_list(0), // You might want to take input for target_number here
        3 => list_iteration_example(),
        4 => map_reduce_filter_example(),
        5 => hash_map_example(),
        _ => println!("Other Thing"),
    }
}