#[derive(Default)]
pub struct Day2 {}

pub enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl crate::aoc::AoCSolution for Day2 {
    type ConvertedType = Vec<Direction>;
    type ReturnType = i32;

    const DAY: usize = 2;

    fn convert(&self, input: &str) -> Vec<Direction> {
        input
            .lines()
            .map(|x| match x.split_once(' ') {
                Some(("forward", x)) => Direction::Forward(x.parse().unwrap()),
                Some(("down", x)) => Direction::Down(x.parse().unwrap()),
                Some(("up", x)) => Direction::Up(x.parse().unwrap()),
                Some((_, _)) => panic!(),
                None => panic!(),
            })
            .collect()
    }

    fn part1(&self, input: &Vec<Direction>) -> i32 {
        let result = input.iter().fold((0, 0), |(x, y), command| match command {
            Direction::Forward(n) => (x + n, y),
            Direction::Up(n) => (x, y - n),
            Direction::Down(n) => (x, y + n),
        });
        result.0 * result.1
    }

    fn part2(&self, input: &Vec<Direction>) -> i32 {
        let result = input
            .iter()
            .fold((0, 0, 0), |(x, y, aim), command| match command {
                Direction::Down(n) => (x, y, aim + n),
                Direction::Up(n) => (x, y, aim - n),
                Direction::Forward(n) => (x + n, y + (aim * n), aim),
            });
        result.0 * result.1
    }
}
