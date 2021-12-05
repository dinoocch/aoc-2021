use regex::Regex;
use std::cmp;
use std::collections::HashMap;

#[derive(Copy, Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Day5 {}

impl crate::aoc::AoCSolution for Day5 {
    type ConvertedType = Vec<(Point, Point)>;
    type ReturnType = usize;

    const DAY: usize = 5;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        lazy_static! {
            static ref LINE_RE: Regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
        }

        input
            .lines()
            .filter_map(|x| LINE_RE.captures(x))
            .filter_map(|x| match (x.get(1), x.get(2), x.get(3), x.get(4)) {
                (Some(a), Some(b), Some(c), Some(d)) => {
                    Some((a.as_str(), b.as_str(), c.as_str(), d.as_str()))
                }
                _ => None,
            })
            .filter_map(
                |(a, b, c, d)| match (a.parse(), b.parse(), c.parse(), d.parse()) {
                    (Ok(a), Ok(b), Ok(c), Ok(d)) => {
                        Some((Point { x: a, y: b }, Point { x: c, y: d }))
                    }
                    _ => None,
                },
            )
            .collect()
    }

    fn part1(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let mut marked: HashMap<(usize, usize), usize> = HashMap::new();
        for (from, to) in input.iter() {
            if from.x != to.x && from.y != to.y {
                continue;
            }

            let points = if from.x == to.x && from.y == to.y {
                vec![(from.x, from.y)]
            } else if from.x == to.x {
                let min_value = cmp::min(from.y, to.y);
                let max_value = cmp::max(from.y, to.y);
                (min_value..=max_value).map(|y| (from.x, y)).collect()
            } else {
                let min_value = cmp::min(from.x, to.x);
                let max_value = cmp::max(from.x, to.x);
                (min_value..=max_value).map(|x| (x, from.y)).collect()
            };

            for (x, y) in points {
                let count = marked.entry((x, y)).or_insert(0);
                *count += 1;
            }
        }
        marked.iter().filter(|(_, count)| count > &&1).count()
    }

    fn part2(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let mut marked: HashMap<(usize, usize), usize> = HashMap::new();
        for (from, to) in input.iter() {
            let points = if from.x == to.x && from.y == to.y {
                vec![(from.x, from.y)]
            } else if from.x == to.x {
                let min_value = cmp::min(from.y, to.y);
                let max_value = cmp::max(from.y, to.y);
                (min_value..=max_value).map(|y| (from.x, y)).collect()
            } else if from.y == to.y {
                let min_value = cmp::min(from.x, to.x);
                let max_value = cmp::max(from.x, to.x);
                (min_value..=max_value).map(|x| (x, from.y)).collect()
            } else {
                // Diagonals
                let mut points = vec![];
                let mut current_point = (from.x, from.y);
                loop {
                    points.push(current_point);
                    if current_point.0 == to.x {
                        break;
                    }
                    if from.x < to.x {
                        current_point.0 += 1;
                    } else {
                        current_point.0 -= 1;
                    }
                    if from.y < to.y {
                        current_point.1 += 1;
                    } else {
                        current_point.1 -= 1;
                    }
                }
                points
            };

            for (x, y) in points {
                let count = marked.entry((x, y)).or_insert(0);
                *count += 1;
            }
        }
        marked.iter().filter(|(_, count)| count > &&1).count()
    }
}
