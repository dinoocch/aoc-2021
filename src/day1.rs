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
fn day1_part1_2021(input: &Vec<i32>) -> i32 {
    input.iter().tuple_windows().map(|(previous, current)| {
        if previous < current { 1 } else { 0 }
    }).sum()
}


#[aoc(day1, part2)]
fn day1_part2_2021(input: &Vec<i32>) -> i32 {
    input.iter().tuple_windows::<(_, _, _)>().map(|(a, b, c)| a + b + c).tuple_windows().map(|(previous, current)| {
        if previous < current { 1 } else { 0 }
    }).sum()

}
