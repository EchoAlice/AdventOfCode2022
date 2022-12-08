#![allow(unused)]

/*
    *Random Note*  
        When rustaceans say "crate", the mean library crate
*/

use crate::helper::input_to_strings;

// Rucksacks contain two compartments.
// Each type within a rucksack should stay wtihin ONE of the two compartments
fn find_repeated_type(rucksack: &String) -> i32{
    // Split rucksack in half.  Compare each value within first half to each value in 2nd half. 

    // Convert repeated type to asci
    
    0
}

fn organize_rucksacks() -> i32 {
    let input = input_to_strings("src/day3/test_input.txt");
    
    let mut priority_sum = 0;
    for line in input.iter() {
        priority_sum += find_repeated_type(line);
        
        println!("{:?}", line);
    } 
    priority_sum
}

pub fn run_day3() -> i32 {
    let answer = crate::day3::main::organize_rucksacks();
    answer
}