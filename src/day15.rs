use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Default)]
pub struct Day15 {}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

// From BinaryHeap ¯\_(ツ)_/¯
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(input: &Vec<Vec<usize>>) -> usize {
    let goal = (input[0].len() - 1, input.len() - 1);
    let mut distances = HashMap::new();
    distances.insert((0, 0), 0);
    // let mut predecessor: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if (x, y) == (0, 0) {
                continue;
            }
            distances.insert((x, y), usize::MAX);
        }
    }
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(State {
        cost: 0,
        position: (0, 0),
    });

    while let Some(State {
        cost,
        position: (x, y),
    }) = queue.pop()
    {
        if (x, y) == goal {
            return cost;
        }

        if &cost > distances.get(&(x, y)).unwrap() {
            continue;
        }

        let mut next_positions = Vec::new();
        if x > 0 {
            next_positions.push((x - 1, y));
        }
        if y > 0 {
            next_positions.push((x, y - 1));
        }
        if x < input[0].len() - 1 {
            next_positions.push((x + 1, y));
        }
        if y < input.len() - 1 {
            next_positions.push((x, y + 1));
        }

        for neighbor in next_positions {
            let next = State {
                cost: cost + input[neighbor.1][neighbor.0],
                position: neighbor,
            };
            if &next.cost < distances.get(&next.position).unwrap() {
                queue.push(next);
                distances.insert(next.position, next.cost);
            }
        }
    }

    unreachable!();
}

impl crate::aoc::AoCSolution for Day15 {
    type ConvertedType = Vec<Vec<usize>>;
    type ReturnType = usize;

    const DAY: usize = 15;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<usize>>()
            })
            .collect()
    }

    fn part1(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        dijkstra(input)
    }

    fn part2(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let mut larger_input = Vec::new();
        for y_repeat in 0..5 {
            for y in 0..input.len() {
                let mut row = Vec::new();
                for x_repeat in 0..5 {
                    for x in 0..input[0].len() {
                        let value = input[y][x] + x_repeat + y_repeat;
                        if value < 10 {
                            row.push(value);
                        } else {
                            row.push(value % 10 + 1);
                        }
                    }
                }
                larger_input.push(row);
            }
        }
        dijkstra(&larger_input)
    }
}
