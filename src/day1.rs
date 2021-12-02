use itertools::Itertools;

#[derive(Default)]
pub struct Day1 {}

impl crate::aoc::AoCSolution for Day1 {
    type ConvertedType = Vec<i32>;
    type ReturnType = usize;

    fn day(&self) -> usize {
        1
    }

    fn convert(&self, input: &str) -> Vec<i32> {
        input
            .lines()
            .map(|x| x.parse::<i32>())
            .filter_map(Result::ok)
            .collect()
    }

    fn part1(&self, input: &Vec<i32>) -> usize {
        input
            .iter()
            .tuple_windows()
            .filter(|(previous, current)| previous < current)
            .count()
    }

    fn part2(&self, input: &Vec<i32>) -> usize {
        input
            .iter()
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .tuple_windows()
            .filter(|(previous, current)| previous < current)
            .count()
    }
}
