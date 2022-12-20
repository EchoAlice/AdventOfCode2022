use crate::helper::input_to_strings;

fn compare_types(c: char, comp2: &Vec<char>) -> i32 {
    let mut priority = 0;
    
    for item in comp2 {
        if &c == item {
            if c.is_lowercase() { 
                priority = (c.clone() as i32) - 96; 
                break 
            } else if c.is_uppercase() { 
                priority = (c.clone() as i32) - 38; 
                break 
            }
        }
    }    
    priority
}

// Rucksacks contain two compartments.
// Each type within a rucksack should stay wtihin ONE of the two compartments
fn find_repeated_type(rucksack: &String) -> i32{
    let mut repeated_sum = 0; 
    let mut compartment1 = Vec::new();  
    let mut compartment2 = Vec::new();  
    let length = rucksack.chars().count();
    
    // Split rucksack in half.   
    let mut char_index = 0; 
    for i in rucksack.chars() {
        if char_index < length/2{ 
            compartment1.push(i);
        } else { 
            compartment2.push(i); 
        }; 
        char_index += 1;
    }
    // Compare value within first half to each value in 2nd half (until you find a match).
    for c in compartment1 {
        let repeated_value = compare_types(c, &compartment2); 
        if repeated_value > 0{
            repeated_sum = repeated_value;
            break
        }
    } 
    repeated_sum
}



fn organize_rucksack() -> i32 {
    let mut priority_sum = 0;
    let input = input_to_strings("src/day3/input.txt");
    
    for line in input.iter() {
        priority_sum += find_repeated_type(line);
    } 
    priority_sum
}



pub fn run_day3_part1() -> i32 {
    let answer = crate::day3::part1::organize_rucksack();
    answer
}