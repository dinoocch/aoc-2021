#[derive(Default)]
pub struct Day6 {}

#[derive(Debug, Default)]
pub struct LanternFishSchool {
    days_to_hatch: [usize; 9],
}

impl LanternFishSchool {
    fn simulate(&self, days: usize) -> usize {
        let mut school = self.days_to_hatch.to_owned();
        for _ in 0..days {
            let birthing_fish = school[0];
            for i in 0..8 {
                school[i] = school[i + 1];
            }
            school[6] += birthing_fish;
            school[8] = birthing_fish;
        }
        school.iter().sum()
    }
}

impl crate::aoc::AoCSolution for Day6 {
    type ConvertedType = LanternFishSchool;
    type ReturnType = usize;

    const DAY: usize = 6;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        let mut school = LanternFishSchool::default();
        input
            .split(',')
            .map(|x| x.trim().parse::<usize>())
            .filter_map(Result::ok)
            .for_each(|x| school.days_to_hatch[x] += 1);
        school
    }

    fn part1(&self, input: &Self::ConvertedType) -> usize {
        input.simulate(80)
    }

    fn part2(&self, input: &Self::ConvertedType) -> usize {
        input.simulate(256)
    }
}
