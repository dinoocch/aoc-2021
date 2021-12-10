#[derive(Default)]
pub struct Day10 {}

pub enum ParsedLine {
    InvalidLine(char),
    IncompleteLine(Vec<char>),
    CompleteLine,
}

fn parse_line(line: &str) -> ParsedLine {
    fn is_open(value: &char) -> bool {
        match value {
            '(' | '[' | '<' | '{' => true,
            _ => false,
        }
    }

    let mut stack = Vec::new();
    for current_char in line.chars() {
        if is_open(&current_char) {
            stack.push(current_char);
            continue;
        }
        if stack.len() == 0 {
            return ParsedLine::InvalidLine(current_char);
        }
        let past_char = stack[stack.len() - 1];
        if matches(&past_char, &current_char) {
            stack.pop();
            continue;
        }
        return ParsedLine::InvalidLine(current_char);
    }
    if stack.len() == 0 {
        ParsedLine::CompleteLine
    } else {
        ParsedLine::IncompleteLine(stack)
    }
}

impl crate::aoc::AoCSolution for Day10 {
    type ConvertedType = Vec<ParsedLine>;
    type ReturnType = u64;

    const DAY: usize = 10;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        input.lines().map(|x| parse_line(x)).collect()
    }

    fn part1(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        input
            .iter()
            .map(|line| match line {
                ParsedLine::InvalidLine(value) => score_part1(value),
                _ => 0,
            })
            .sum()
    }

    fn part2(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let mut scores: Vec<u64> = input
            .iter()
            .map(|line| match line {
                ParsedLine::IncompleteLine(stack) => stack
                    .iter()
                    .rev()
                    .fold(0, |acc, c| acc * 5 + score_part2(c)),
                _ => 0,
            })
            .filter(|x| *x > 0)
            .collect();
        scores.sort();
        scores[scores.len() / 2]
    }
}

fn matches(a: &char, b: &char) -> bool {
    match (a, b) {
        ('(', ')') | ('<', '>') | ('[', ']') | ('{', '}') => true,
        _ => false,
    }
}

fn score_part1(a: &char) -> u64 {
    match a {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn score_part2(a: &char) -> u64 {
    match a {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}
