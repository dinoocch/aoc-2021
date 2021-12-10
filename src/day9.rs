use std::collections::HashSet;

#[derive(Default)]
pub struct Day9 {}

fn low_points(input: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut low_points = Vec::new();
    for row in 0..input.len() {
        for column in 0..input[row].len() {
            let mut low_point = true;
            if column + 1 < input[row].len() && input[row][column] >= input[row][column + 1] {
                low_point = false;
            }
            if column > 0 && input[row][column] >= input[row][column - 1] {
                low_point = false;
            }
            if row > 0 && input[row][column] >= input[row - 1][column] {
                low_point = false;
            }
            if row + 1 < input.len() && input[row][column] >= input[row + 1][column] {
                low_point = false;
            }

            if low_point {
                low_points.push((row, column));
            }
        }
    }
    low_points
}

impl crate::aoc::AoCSolution for Day9 {
    type ConvertedType = Vec<Vec<u32>>;
    type ReturnType = u32;

    const DAY: usize = 9;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        input
            .lines()
            .map(|x| {
                x.chars()
                    .map(|height| height.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect()
    }

    fn part1(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        low_points(input)
            .iter()
            .map(|(x, y)| input[*x][*y] + 1)
            .sum()
    }

    fn part2(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let mut basin_sizes = Vec::new();
        let low_coords = low_points(input);

        for (row, column) in low_coords.iter() {
            let (row, column) = (*row, *column);
            let mut basin_points = HashSet::new();
            let mut to_consider = HashSet::new();
            basin_points.insert((row, column));
            to_consider.insert((row, column));
            loop {
                let mut added = HashSet::new();
                for (x, y) in to_consider.drain() {
                    if y + 1 < input[x].len()
                        && input[x][y + 1] < 9
                        && !basin_points.contains(&(x, y + 1))
                    {
                        added.insert((x, y + 1));
                    }
                    if y > 0 && input[x][y - 1] < 9 && !basin_points.contains(&(x, y - 1)) {
                        added.insert((x, y - 1));
                    }
                    if x > 0 && input[x - 1][y] < 9 && !basin_points.contains(&(x - 1, y)) {
                        added.insert((x - 1, y));
                    }
                    if x + 1 < input.len()
                        && input[x + 1][y] < 9
                        && !basin_points.contains(&(x + 1, y))
                    {
                        added.insert((x + 1, y));
                    }
                }
                if added.len() == 0 {
                    break;
                }
                basin_points.extend(&added);
                to_consider.extend(&added);
            }
            basin_sizes.push(basin_points.len() as u32);
        }
        basin_sizes.sort();
        basin_sizes.iter().rev().take(3).fold(1, |acc, x| acc * x)
    }
}
