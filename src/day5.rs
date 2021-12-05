use regex::Regex;
use std::collections::HashMap;

#[derive(Copy, Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn points_between(&self, other: &Point) -> Vec<Point> {
        let mut points = vec![];
        let mut current_point = self.to_owned();
        loop {
            points.push(current_point.to_owned());
            if current_point.x == other.x && current_point.y == other.y {
                break;
            }
            if self.x < other.x {
                current_point.x += 1;
            } else if self.x > other.x {
                current_point.x -= 1;
            }
            if self.y < other.y {
                current_point.y += 1;
            } else if self.y > other.y {
                current_point.y -= 1;
            }
        }
        points
    }
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
        let mut marked: HashMap<Point, usize> = HashMap::new();
        input
            .iter()
            .filter(|(from, to)| from.x == to.x || from.y == to.y)
            .map(|(from, to)| from.points_between(to))
            .flatten()
            .for_each(|point| *marked.entry(point).or_insert(0) += 1);
        marked.iter().filter(|(_, count)| **count > 1).count()
    }

    fn part2(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let mut marked: HashMap<Point, usize> = HashMap::new();
        input
            .iter()
            .map(|(from, to)| from.points_between(to))
            .flatten()
            .for_each(|point| *marked.entry(point).or_insert(0) += 1);
        marked.iter().filter(|(_, count)| **count > 1).count()
    }
}
