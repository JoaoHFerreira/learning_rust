use std::io::stdin;

fn scan(){
    println!("Scanning the file");
    let mut my_string = String::new();
    let _ = stdin().read_line(&mut my_string).unwrap();
    println!("{}", my_string);
}

fn finding_elements_in_list(target_number: usize){
    let numbers = vec![1, 2, 3, 4];
    println!("the number in the vector postion is {}", numbers[target_number]);

    for n in numbers{
        if n == target_number{
            println!("Your choosen number is {}", n);
            return;
        }
    }

    println!("The value {} was not found", target_number);

}

fn main() {
    println!("1: Scan file");
    println!("2: Find given element in list");
    println!("\n\n");


    let mut input = String::new();
    let _ = stdin().read_line(&mut input).unwrap();
    let number: usize = input.trim().parse().unwrap();

    match number{
        1 => scan(),
        2 => finding_elements_in_list(0),
        _ => println!("Other Thing"),
    }   
    
}