#![allow(unused)]

mod helper;
mod day1;
mod day2;
use crate::day1::part1and2::run_day1;
use crate::day2::parts1and2::run_day2;

fn main() {
    let answer = run_day2();
    println!("Answer: {:?}", answer);
}
