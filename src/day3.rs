#[derive(Default)]
pub struct Day3 {}

impl crate::aoc::AoCSolution for Day3 {
    type ConvertedType = Vec<Vec<bool>>;
    type ReturnType = u32;

    const DAY: usize = 3;

    fn convert(&self, input: &str) -> Vec<Vec<bool>> {
        input
            .lines()
            .map(|x| x.chars().map(|x| x == '1').collect())
            .collect()
    }

    fn part1(&self, input: &Vec<Vec<bool>>) -> u32 {
        let lines = input.len();
        let columns = input[0].len();

        let mut gamma = vec![];
        let mut epsilon = vec![];

        for column in 0..columns {
            let ones = input.iter().filter(|x| x[column]).count();
            if ones > lines / 2 {
                epsilon.push('0');
                gamma.push('1');
            } else {
                epsilon.push('1');
                gamma.push('0');
            }
        }

        let gamma: String = gamma.iter().collect();
        let epsilon: String = epsilon.iter().collect();

        let gamma = u32::from_str_radix(&gamma, 2).unwrap();
        let epsilon = u32::from_str_radix(&epsilon, 2).unwrap();
        gamma * epsilon
    }

    fn part2(&self, input: &Vec<Vec<bool>>) -> u32 {
        fn part2_inner(most_common: bool, input: &Vec<Vec<bool>>) -> u32 {
            let mut input = input.to_owned();

            for column in 0..input[0].len() {
                if input.len() == 1 {
                    break;
                }

                let ones = input.iter().filter(|value| value[column]).count();

                let zeros = input.len() - ones;

                let filter_value =
                    if (most_common && zeros <= ones) || (!most_common && zeros > ones) {
                        true
                    } else {
                        false
                    };

                input.retain(|x| x[column] == filter_value);
            }

            let result_str: String = input[0]
                .iter()
                .map(|x| match x {
                    true => '1',
                    false => '0',
                })
                .collect();
            return u32::from_str_radix(&result_str, 2).unwrap();
        }

        let oxygen = part2_inner(true, input);
        let co2 = part2_inner(false, input);

        oxygen * co2
    }
}
