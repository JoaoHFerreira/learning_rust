pub mod finding_elements_in_list{
    pub fn find_n(target_number: usize){
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
}