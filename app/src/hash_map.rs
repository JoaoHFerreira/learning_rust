
pub mod hashing{
    use std::collections::HashMap;
    pub fn h(){
        let mut scores = HashMap::new();
        scores.insert("Alice", 95);
        scores.insert("Bob", 82);
    
        for (name, score) in &scores {
            println!("{}: {}", name, score);
        }
    }
}