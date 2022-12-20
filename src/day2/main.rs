use std::str;
use crate::helper::{input_to_strings, read_to_bytes, print_type_of, read_lines};
use crate::day2::shapes;


// Uses enumerator to allow for chars and &strs to be used together within match 
enum Input<'a> {
    Character(char),
    String(&'a str),
}

fn assign_shape<'a> (move_input: &'a Input) -> shapes::Shape {
    match move_input {
        Input::String("rock") | Input::Character('A') | Input::Character('X') => shapes::rock(),
        Input::String("paper") | Input::Character('B') | Input::Character('Y') => shapes::paper(),
        _ => shapes::scissors(),
    }
}

// X, Y, and Z are rock, paper and scissors
fn part_1_round_logic(input: String) -> i32{
    let mut round_score = 0; 
    let opp_shape = assign_shape(&Input::Character(input.chars().nth(0).unwrap()));
    let my_shape = assign_shape(&Input::Character(input.chars().nth(2).unwrap()));
    round_score += my_shape.points;
    
    // Game logic.  Could match be use here with data structures?  
    if my_shape.beats == opp_shape.name {
        round_score += 6; 
    } else if my_shape.ties == opp_shape.name {
        round_score += 3; 
    } else {
        round_score += 0;
    } 
    round_score
}

// X, Y, and Z are the needed outcomes: wins, ties, and losses
fn part_2_round_logic(input: String) -> i32{
    let mut round_score = 0; 
    let opp_shape = assign_shape(&Input::Character(input.chars().nth(0).unwrap()));
    let outcome = input.chars().nth(2).unwrap();
    
    if outcome == 'X' {
        // I must lose
        let my_shape = assign_shape(&Input::String(&opp_shape.beats));      
        round_score += my_shape.points;  
    } else if outcome == 'Y' {
        // I must tie 
        let my_shape = assign_shape(&Input::String(&opp_shape.ties));      
        round_score += my_shape.points + 3;
    } else {
        // I must win 
        let my_shape = assign_shape(&Input::String(&opp_shape.loses_to));      
        round_score += my_shape.points + 6;
    } 
    round_score
}

fn rock_paper_scissors() -> i32 {
    let input = input_to_strings("src/day2/input.txt"); 
    let mut total_score = 0; 
    for line in input {
        total_score += part_2_round_logic(line); 
    } 
    total_score 
}



pub fn run_day2() -> i32 {
    let answer = crate::day2::main::rock_paper_scissors();
    answer
}