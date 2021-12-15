#![feature(hash_drain_filter)]

#[macro_use]
extern crate lazy_static;

mod aoc;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use aoc::AoCSolution;

fn main() {
    day15::Day15::run();
}
