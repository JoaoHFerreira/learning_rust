pub mod lister{
    pub fn iteration(){
        let fruits = vec!["apple", "banana", "orange"];
        for fruit in &fruits {
            println!("{}", fruit);
        }
    }
}