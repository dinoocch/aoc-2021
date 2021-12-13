use ::std::collections::HashSet;

#[derive(Default)]
pub struct Day13 {}

#[derive(Debug)]
enum Fold {
    Left(usize),
    Up(usize),
}

#[derive(Debug)]
pub struct Game {
    points: HashSet<(usize, usize)>,
    folds: Vec<Fold>,
}

impl crate::aoc::AoCSolution for Day13 {
    type ConvertedType = Game;
    type ReturnType = usize;

    const DAY: usize = 13;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        let mut points: HashSet<(usize, usize)> = HashSet::new();
        let mut folds: Vec<Fold> = Vec::new();
        input.lines().for_each(|line| {
            if line.contains(",") {
                let (a, b) = line.split_once(",").unwrap();
                points.insert((a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()));
            }
            if line.contains("fold") {
                let fold_str = line.rsplit_once(" ").unwrap().1;
                let fold = match fold_str.split_once("=").unwrap() {
                    ("y", v) => Fold::Up(v.parse::<usize>().unwrap()),
                    ("x", v) => Fold::Left(v.parse::<usize>().unwrap()),
                    _ => panic!("Found invalid fold"),
                };
                folds.push(fold)
            }
        });
        Game { points, folds }
    }

    fn part1(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let mut points = input.points.clone();
        for fold in &input.folds {
            points = match fold {
                Fold::Left(fold) => fold_left(&points, *fold),
                Fold::Up(fold) => fold_up(&points, *fold),
            };
            return points.len();
        }
        unreachable!();
    }

    fn part2(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let mut points = input.points.clone();
        for fold in &input.folds {
            // print_points(&points);
            points = match fold {
                Fold::Left(fold) => fold_left(&points, *fold),
                Fold::Up(fold) => fold_up(&points, *fold),
            };
        }
        print_points(&points);
        42
    }
}

fn fold_left(points: &HashSet<(usize, usize)>, fold: usize) -> HashSet<(usize, usize)> {
    let mut new_points = HashSet::new();
    for (x, y) in points {
        let (x, y) = (*x, *y);
        if x <= fold {
            new_points.insert((x, y));
        } else {
            if 2 * fold < x {
                continue;
            }
            new_points.insert((2 * fold - x, y));
        }
    }
    new_points
}

fn fold_up(points: &HashSet<(usize, usize)>, fold: usize) -> HashSet<(usize, usize)> {
    let mut new_points = HashSet::new();
    for (x, y) in points {
        let (x, y) = (*x, *y);
        if y <= fold {
            new_points.insert((x, y));
        } else {
            if 2 * fold < y {
                continue;
            }
            new_points.insert((x, 2 * fold - y));
        }
    }
    new_points
}

fn print_points(points: &HashSet<(usize, usize)>) {
    let max_x = points.iter().map(|(x, _)| x).max().unwrap();
    let max_y = points.iter().map(|(_, y)| y).max().unwrap();

    for y in 0..=*max_y {
        let line: String = (0..=*max_x)
            .map(|x| if points.contains(&(x, y)) { '█' } else { ' ' })
            .collect();
        println!("{}", line);
    }
    println!();
}
