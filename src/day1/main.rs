// A file can be a module. 
// Super allows you to access functionality from a parent module
use std::str;
use crate::helper::read_to_bytes;

fn gather_input(path: &str) -> Vec<i32> {
    let mut input: Vec<i32> = Vec::new(); 
    let data = read_to_bytes(path);
    let my_string = str::from_utf8(&data).unwrap(); 
    // Hacky way to gather input.  I didn't use errors and substituted empty space with zeros...  Ask Leo what he did  
    for s in my_string.split_terminator("\r\n") {
        if s != ""{
            let my_int = s.parse::<i32>().unwrap(); 
            input.push(my_int);
        } else {
            input.push(0);
        } 
    }
    input   
}

// Which elf is carrying the most calories?
fn most_calories() -> (i32, i32) {
    let path = "src/day1/input.txt";
    let input = gather_input(path); 
    
    // Should I be using match?
    let mut elves: Vec<i32> =  Vec::new();
    let mut current_elf = 0; 
    let mut biggest_elf = 0;
    for i in input {
        if i != 0{
            current_elf += i;  
            if current_elf >= biggest_elf{
                biggest_elf = current_elf
            } 
        } else {
            elves.push(current_elf); 
            current_elf = 0;  
        } 
    }
    elves.sort();
    let top3 = elves[elves.len() - 1] +  elves[elves.len() - 2] + elves[elves.len() - 3];
    return (biggest_elf, top3);
}


/* 
Provide pub fn for other files to be able to come in and execute these functions
*/
pub fn run_day1() -> (i32, i32) {
    let (most_calories, top3) = crate::day1::main::most_calories();
    return (most_calories, top3);
}