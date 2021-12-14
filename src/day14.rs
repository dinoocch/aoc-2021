use itertools::Itertools;
use std::collections::HashMap;

#[derive(Default)]
pub struct Day14 {}

#[derive(Debug, Clone)]
pub struct Game {
    template: Vec<char>,
    pair_rules: HashMap<(char, char), char>,
}

fn create_pair_rules(input: &str) -> HashMap<(char, char), char> {
    let mut rules = HashMap::new();
    input
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .for_each(|(from, to)| {
            let from: Vec<char> = from.chars().collect();
            let to: Vec<char> = to.chars().collect();
            rules.insert((from[0], from[1]), to[0]);
        });
    rules
}

impl crate::aoc::AoCSolution for Day14 {
    type ConvertedType = Game;
    type ReturnType = usize;

    const DAY: usize = 14;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        let (template, pairs) = input.split_once("\n\n").unwrap();
        Game {
            template: template.chars().collect(),
            pair_rules: create_pair_rules(pairs),
        }
    }

    fn part1(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        simulate(input, 10)
    }

    fn part2(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        simulate(input, 40)
    }
}

fn simulate(input: &Game, steps: usize) -> usize {
    let mut initial: HashMap<(char, char), usize> = HashMap::new();
    input.template.iter().tuple_windows().for_each(|(a, b)| {
        *(initial.entry((*a, *b)).or_insert(0)) += 1;
    });
    let pairs = (0..steps).fold(initial, |current, _| {
        let mut next: HashMap<(char, char), usize> = HashMap::new();
        current.iter().for_each(|((a, b), count)| {
            if let Some(c) = input.pair_rules.get(&(*a, *b)) {
                *(next.entry((*a, *c)).or_insert(0)) += count;
                *(next.entry((*c, *b)).or_insert(0)) += count;
            } else {
                *(next.entry((*a, *b)).or_insert(0)) += count;
            }
        });
        next
    });
    let mut character_counts = HashMap::new();
    for ((a, _), count) in pairs.iter() {
        *(character_counts.entry(*a).or_insert(0)) += count;
    }
    *(character_counts
        .entry(*input.template.last().unwrap())
        .or_insert(0)) += 1;
    let min_count = character_counts.values().min().unwrap();
    let max_count = character_counts.values().max().unwrap();
    max_count - min_count
}
