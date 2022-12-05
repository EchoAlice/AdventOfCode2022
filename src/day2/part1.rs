#![allow(unused)]

use std::str;
use std::collections::HashMap;
use crate::helper::{read_to_bytes, print_type_of, read_lines};


/*
    // &str  --> Read only reference to the string (works with other types as well)
    // String

    +1    A = Rock             Y = Paper
    +2    B = Paper            X = Rock
    +3    C = Scissors         Z = Scissors
*/

fn round_logic(input: String) -> i32{
    let shape  = (('X', 1), ('Y', 2), ('Z', 3)); 
    let opp_move =  input.chars().nth(0).unwrap();
    let my_move =  input.chars().nth(2).unwrap();
   
    // Shape logic 
    // How can I use an index to access "shape.i.0"
    let mut round_score = 0; 
    if my_move == shape.0.0 {
        round_score += shape.0.1;
    } else if my_move == shape.1.0 {
        round_score += shape.1.1; 
    } else {
        round_score += shape.2.1; 
    } 
    // Match logic 
    
    round_score
}


fn gather_input(path: &str) -> Vec<String> {
    let mut input: Vec<String> = Vec::new(); 
    if let Ok(lines) = read_lines(path){
        for line in lines.flatten(){
            input.push(line);
        }
    }
    input
}

fn rock_paper_scissors() -> i32 {
    let input = gather_input("src/day2/test_input.txt"); 
    let mut total_score = 0; 
    for line in input {
        total_score += round_logic(line); 
    } 
    total_score 
}




pub fn run_day2() -> i32 {
    let answer = crate::day2::part1::rock_paper_scissors();
    answer
}