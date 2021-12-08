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
        let signals = signals.split_whitespace().map(|x| x.to_owned()).collect();
        let outputs = outputs.split_whitespace().map(|x| x.to_owned()).collect();
        return Ok(Line { signals, outputs });
    }
}

impl crate::aoc::AoCSolution for Day8 {
    type ConvertedType = Vec<Line>;
    type ReturnType = usize;

    const DAY: usize = 8;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        input
            .lines()
            .map(|x| Line::from_str(x))
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
                values[1] = line.signals.iter().filter(|x| x.len() == 2).next().unwrap();
                values[7] = line.signals.iter().filter(|x| x.len() == 3).next().unwrap();
                values[4] = line.signals.iter().filter(|x| x.len() == 4).next().unwrap();
                values[8] = line.signals.iter().filter(|x| x.len() == 7).next().unwrap();
                values[9] = line
                    .signals
                    .iter()
                    .filter(|x| x.len() == 6 && values[4].chars().all(|c| x.contains(c)))
                    .next()
                    .unwrap();
                values[6] = line
                    .signals
                    .iter()
                    .filter(|x| {
                        &**x != values[9]
                            && x.len() == 6
                            && !values[1].chars().all(|c| x.contains(c))
                    })
                    .next()
                    .unwrap();
                values[3] = line
                    .signals
                    .iter()
                    .filter(|x| {
                        !values.iter().any(|v| v == x)
                            && x.len() == 5
                            && values[1].chars().all(|c| x.contains(c))
                    })
                    .next()
                    .unwrap();
                values[0] = line
                    .signals
                    .iter()
                    .filter(|x| !values.iter().any(|v| v == x) && x.len() == 6)
                    .next()
                    .unwrap();
                values[5] = line
                    .signals
                    .iter()
                    .filter(|x| {
                        !values.iter().any(|v| v == x)
                            && values[6].chars().fold(0, |missing, v| {
                                if x.contains(v) {
                                    missing
                                } else {
                                    missing + 1
                                }
                            }) == 1
                    })
                    .next()
                    .unwrap();
                values[2] = line
                    .signals
                    .iter()
                    .filter(|x| !values.iter().any(|v| v == x))
                    .next()
                    .unwrap();
                let sorted_value_strings: Vec<String> = values
                    .iter()
                    .map(|value| {
                        let mut chars: Vec<_> = value.chars().collect();
                        chars.sort();
                        chars.iter().collect::<String>()
                    })
                    .collect();
                let sorted_output_strings: Vec<String> = line
                    .outputs
                    .iter()
                    .map(|value| {
                        let mut chars: Vec<_> = value.chars().collect();
                        chars.sort();
                        chars.iter().collect::<String>()
                    })
                    .collect();
                let output: Vec<usize> = line
                    .outputs
                    .iter()
                    .map(|output| {
                        let mut chars: Vec<_> = output.chars().collect();
                        chars.sort();
                        let sorted_output: String = chars.iter().collect();

                        sorted_value_strings
                            .iter()
                            .enumerate()
                            .filter(|(_, v)| &sorted_output == *v)
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
