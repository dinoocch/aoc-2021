use std::collections::HashSet;

#[derive(Default)]
pub struct Day11 {}

fn step(mut input: Vec<Vec<u32>>) -> (u64, Vec<Vec<u32>>) {
    input
        .iter_mut()
        .for_each(|v| v.iter_mut().for_each(|x| *x += 1));
    let mut flashed_already = HashSet::new();
    loop {
        let mut flashed = false;
        for row in 0..input.len() {
            for column in 0..input[0].len() {
                if input[row][column] > 9 && !flashed_already.contains(&(row, column)) {
                    if row < input.len() - 1 {
                        input[row + 1][column] += 1;
                    }
                    if row > 0 {
                        input[row - 1][column] += 1;
                    }
                    if column < input.len() - 1 {
                        input[row][column + 1] += 1;
                    }
                    if column > 0 {
                        input[row][column - 1] += 1;
                    }
                    if row > 0 && column > 0 {
                        input[row - 1][column - 1] += 1;
                    }
                    if row < input.len() - 1 && column < input.len() - 1 {
                        input[row + 1][column + 1] += 1;
                    }
                    if row > 0 && column < input.len() - 1 {
                        input[row - 1][column + 1] += 1;
                    }
                    if column > 0 && row < input.len() - 1 {
                        input[row + 1][column - 1] += 1;
                    }
                    flashed = true;
                    flashed_already.insert((row, column));
                }
            }
        }
        if !flashed {
            break;
        }
    }
    input.iter_mut().for_each(|v| {
        v.iter_mut().for_each(|x| {
            if *x > 9 {
                *x = 0
            }
        })
    });
    (flashed_already.len() as u64, input)
}

impl crate::aoc::AoCSolution for Day11 {
    type ConvertedType = Vec<Vec<u32>>;
    type ReturnType = u64;

    const DAY: usize = 11;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        input
            .lines()
            .map(|x| x.chars().map(|v| v.to_digit(10)).flatten().collect())
            .collect()
    }

    fn part1(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let mut input = input.to_owned();
        let mut flashes = 0;
        for _ in 0..100 {
            let (step_flashes, step_result) = step(input);
            input = step_result;
            flashes += step_flashes;
        }
        flashes
    }

    fn part2(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let mut input = input.to_owned();
        let mut counter = 0;
        loop {
            counter += 1;
            let (step_flashes, step_result) = step(input);
            input = step_result;
            if step_flashes == 100 {
                return counter;
            }
        }
    }
}
