pub mod mapping{
    pub fn m(){
        let words = vec!["rust", "is", "awesome"];
        let lengths: Vec<usize> = words.iter().map(|word| word.len()).collect();
        let total_length: usize = lengths.iter().sum();
        let long_words: Vec<&str> = words.iter().filter(|word| word.len() > 3).map(|word| *word).collect();
    
        println!("Lengths: {:?}", lengths);
        println!("Total length: {}", total_length);
        println!("Long words: {:?}", long_words);

    }
}