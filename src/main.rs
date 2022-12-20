#![allow(unused)]

mod helper;
mod day1;
mod day2;
mod day3;
mod notes;
use crate::day1::main::run_day1;
use crate::day2::main::run_day2;
use crate::day3::part1::run_day3_part1;
use crate::day3::part2::run_day3_part2;
use crate::notes::threads::run_notes_threads;
use crate::notes::ownership::run_notes_ownership;
use crate::notes::asyncronous::run_notes_async;


fn main() {
    // let answer = run_notes_threads();
    // println!("Threads: {:?}", answer);
    // println!("\n");   

    let answer = run_notes_async();
    println!("Async: {:?}", answer);
}