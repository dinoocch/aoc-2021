use std::collections::HashMap;

#[derive(Default)]
pub struct Day7 {}

impl crate::aoc::AoCSolution for Day7 {
    type ConvertedType = Vec<usize>;
    type ReturnType = usize;

    const DAY: usize = 7;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        input
            .split(',')
            .map(|x| x.trim().parse::<usize>())
            .filter_map(Result::ok)
            .collect()
    }

    fn part1(&self, input: &Self::ConvertedType) -> usize {
        let mut counts: HashMap<usize, usize> = HashMap::new();
        let mut input = input.to_owned();
        input.sort();
        input
            .iter()
            .for_each(|x| *counts.entry(*x).or_insert(0) += 1);

        let min = input[0];
        let max = input[input.len() - 1];

        let mut min_fuel = usize::MAX;
        for position in min..max {
            let mut cost = 0;
            for (start, count) in &counts {
                cost += ((*start as i32 - position as i32).abs() as usize) * count;
            }
            if cost < min_fuel {
                min_fuel = cost;
            }
        }
        min_fuel
    }

    fn part2(&self, input: &Self::ConvertedType) -> usize {
        let mut counts: HashMap<usize, usize> = HashMap::new();
        let mut input = input.to_owned();
        input.sort();
        input
            .iter()
            .for_each(|x| *counts.entry(*x).or_insert(0) += 1);

        let min = input[0];
        let max = input[input.len() - 1];

        let mut min_fuel = usize::MAX;
        for position in min..max {
            let mut cost = 0;
            for (start, count) in &counts {
                let moves = (*start as i32 - position as i32).abs() as usize;
                cost += (moves * (moves + 1) / 2) * count;
            }
            if cost < min_fuel {
                min_fuel = cost;
            }
        }
        min_fuel
    }
}
