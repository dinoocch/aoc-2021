#![feature(hash_drain_filter)]

#[macro_use]
extern crate lazy_static;

mod aoc;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

use aoc::AoCSolution;

fn main() {
    // day1::Day1 {}.run();
    // day2::Day2::run();
    // day3::Day3::run();
    // day4::Day4::run();
    // day5::Day5::run();
    // day6::Day6::run();
    // day7::Day7::run();
    day8::Day8::run();
}
