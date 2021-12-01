use itertools::Itertools;

#[aoc_generator(day1)]
fn convert(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|x| x.parse::<i32>())
        .filter_map(Result::ok)
        .collect()
}

#[aoc(day1, part1)]
fn day1_part1_2021(input: &Vec<i32>) -> usize {
    input
        .iter()
        .tuple_windows()
        .filter(|(previous, current)| previous < current)
        .count()
}

#[aoc(day1, part2)]
fn day1_part2_2021(input: &Vec<i32>) -> usize {
    input
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(previous, current)| previous < current)
        .count()
}
