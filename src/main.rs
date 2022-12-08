#![allow(unused)]

mod helper;
mod day1;
mod day2;
mod day3;
use crate::day1::main::run_day1;
use crate::day2::main::run_day2;
use crate::day3::main::run_day3;

fn main() {
    let answer = run_day2();
    println!("Answer: {:?}", answer);
}