#![allow(unused)]

mod helper;
mod day1;
mod day2;
mod day3;
mod day4;
use crate::day1::main::run_day1;
use crate::day2::main::run_day2;
use crate::day3::part1::run_day3_part1;
use crate::day3::part2::run_day3_part2;

fn main() {
    let answer = run_day3_part2();
    println!("Answer: {:?}", answer);
}