use std::io::stdin;


mod scan_printer;
use scan_printer::scanner as scanner;

mod find_elements;
use find_elements::finding_elements_in_list as finder;

mod list_iteration;
use list_iteration::lister::iteration;

mod hash_map;
use hash_map::hashing;

mod map_reduce;
use map_reduce::mapping;


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
        1 => scanner::scan(),
        2 => finder::find_n(0),
        3 => iteration(),
        4 => mapping::m(),
        5 => hashing::h(),
        _ => println!("Other Thing"),
    }
}