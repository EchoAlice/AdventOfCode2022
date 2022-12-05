#![allow(unused)]

mod helper;
mod day1;
mod day2;
use crate::day1::part1and2::run_day1;
use crate::day2::part1::run_day2;

// Crates:  Modules that produce a library or executable
// Modules: Organize and handle privacy
// Packages: Build, test and share crates
// Paths: A way of naming an item such as a struct, function

// Rewatch "Rust Demystified": ownership
//  String vs &str?

fn main() {
    let answer = run_day2();
    println!("Answer: {:?}", answer)
}
