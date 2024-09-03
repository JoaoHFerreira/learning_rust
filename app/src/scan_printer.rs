pub mod scanner {
    use std::io::stdin;

    pub fn scan() {
        println!("Scanning the file");
        let mut my_string = String::new();
        let _ = stdin().read_line(&mut my_string).unwrap();
        println!("{}", my_string);   
    }
}