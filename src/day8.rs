use std::io::Error;
use std::str::FromStr;

#[derive(Default)]
pub struct Day8 {}

#[derive(Default, Debug, Clone)]
pub struct Line {
    signals: Vec<String>,
    outputs: Vec<String>,
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (signals, outputs) = s.split_once(" | ").unwrap();
        let signals = signals
            .split_whitespace()
            .map(|x| {
                let mut chars: Vec<char> = x.chars().collect();
                chars.sort_unstable();
                chars.iter().collect::<String>()
            })
            .collect();
        let outputs = outputs
            .split_whitespace()
            .map(|x| {
                let mut chars: Vec<char> = x.chars().collect();
                chars.sort_unstable();
                chars.iter().collect::<String>()
            })
            .collect();
        Ok(Line { signals, outputs })
    }
}

impl crate::aoc::AoCSolution for Day8 {
    type ConvertedType = Vec<Line>;
    type ReturnType = usize;

    const DAY: usize = 8;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        input
            .lines()
            .map(Line::from_str)
            .filter_map(Result::ok)
            .collect()
    }

    fn part1(&self, input: &Self::ConvertedType) -> usize {
        input
            .iter()
            .map(|x| {
                x.outputs
                    .iter()
                    .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
                    .count()
            })
            .sum()
    }

    fn part2(&self, input: &Self::ConvertedType) -> usize {
        input
            .iter()
            .map(|line| {
                let mut values = [""; 10];
                values[1] = line.signals.iter().find(|x| x.len() == 2).unwrap();
                values[7] = line.signals.iter().find(|x| x.len() == 3).unwrap();
                values[4] = line.signals.iter().find(|x| x.len() == 4).unwrap();
                values[8] = line.signals.iter().find(|x| x.len() == 7).unwrap();
                values[9] = line
                    .signals
                    .iter()
                    .find(|x| x.len() == 6 && values[4].chars().all(|c| x.contains(c)))
                    .unwrap();
                values[6] = line
                    .signals
                    .iter()
                    .find(|x| {
                        **x != values[9]
                            && x.len() == 6
                            && !values[1].chars().all(|c| x.contains(c))
                    })
                    .unwrap();
                values[3] = line
                    .signals
                    .iter()
                    .find(|x| {
                        !values.iter().any(|v| v == x)
                            && x.len() == 5
                            && values[1].chars().all(|c| x.contains(c))
                    })
                    .unwrap();
                values[0] = line
                    .signals
                    .iter()
                    .find(|x| !values.iter().any(|v| v == x) && x.len() == 6)
                    .unwrap();
                values[5] = line
                    .signals
                    .iter()
                    .find(|x| {
                        !values.iter().any(|v| v == x)
                            && values[6].chars().fold(0, |missing, v| {
                                if x.contains(v) {
                                    missing
                                } else {
                                    missing + 1
                                }
                            }) == 1
                    })
                    .unwrap();
                values[2] = line
                    .signals
                    .iter()
                    .find(|x| !values.iter().any(|v| v == x))
                    .unwrap();
                let output: Vec<usize> = line
                    .outputs
                    .iter()
                    .map(|output| {
                        values
                            .iter()
                            .enumerate()
                            .filter(|(_, v)| output == *v)
                            .map(|(index, _)| index)
                            .next()
                            .unwrap()
                    })
                    .collect();

                output
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap()
            })
            .sum()
    }
}
