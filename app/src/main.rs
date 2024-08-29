use std::io::stdin;

fn scan(){
    println!("Scanning the file");
    let mut my_string = String::new();
    let _ = stdin().read_line(&mut my_string).unwrap();
    println!("{}", my_string);
}

fn finding_elements_in_list(target_position: usize){
    let numbers = vec![1, 2, 3, 4];
    println!("{}", numbers[target_position]);
}

fn main() {
    println!("1: Scan file");
    println!("2: Find given element in list");
    println!("\n\n");


    let mut input = String::new();
    let _ = stdin().read_line(&mut input).unwrap();
    let number: i32 = input.trim().parse().unwrap();

    match number{
        1 => scan(),
        2 => finding_elements_in_list(3),
        _ => println!("Other Thing"),
    }   
    
}