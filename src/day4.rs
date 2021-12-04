#[derive(Default)]
pub struct Day4 {}

#[derive(Debug, Default, Copy, Clone)]
pub struct Board {
    numbers: [[usize; 5]; 5],
    board: [[bool; 5]; 5],
    current_row: usize,
}

impl Board {
    fn add_line(&mut self, line: &str) -> bool {
        line.split_whitespace()
            .into_iter()
            .map(|x| x.parse::<usize>())
            .filter_map(Result::ok)
            .take(5)
            .enumerate()
            .for_each(|(index, value)| self.numbers[self.current_row][index] = value);
        self.current_row += 1;
        self.current_row == 5
    }

    fn draw(&mut self, number: usize) {
        for row in 0..5 {
            for column in 0..5 {
                if self.numbers[row][column] == number {
                    self.board[row][column] = true;
                }
            }
        }
    }

    fn check(&self) -> bool {
        for x in 0..5 {
            if self.board[x].iter().all(|item| *item) {
                return true;
            }

            if self.board.iter().all(|item| item[x]) {
                return true;
            }
        }
        false
    }

    fn score(&self, draw: usize) -> usize {
        let mut score = 0;
        for row in 0..5 {
            for column in 0..5 {
                if !self.board[row][column] {
                    score += self.numbers[row][column];
                }
            }
        }
        return score * draw;
    }
}

#[derive(Debug, Default, Clone)]
pub struct Game {
    draws: Vec<usize>,
    boards: Vec<Board>,
}

impl crate::aoc::AoCSolution for Day4 {
    type ConvertedType = Game;
    type ReturnType = usize;

    const DAY: usize = 4;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        let mut lines = input.lines();

        let draws: Vec<usize> = lines
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<usize>())
            .filter_map(Result::ok)
            .collect();

        let mut game = Game {
            draws,
            boards: vec![],
        };

        let mut current_board = Board::default();
        for line in lines {
            if line.len() == 0 {
                continue;
            }
            if current_board.add_line(&line) {
                game.boards.push(current_board);
                current_board = Board::default();
            }
        }

        game
    }

    fn part1(&self, input: &Self::ConvertedType) -> usize {
        let mut owned_input = input.to_owned();

        for current_draw in owned_input.draws {
            for board in owned_input.boards.iter_mut() {
                board.draw(current_draw);
                if board.check() {
                    return board.score(current_draw);
                }
            }
        }
        unreachable!();
    }

    fn part2(&self, input: &Self::ConvertedType) -> usize {
        let mut owned_input = input.to_owned();

        for current_draw in owned_input.draws {
            for board in owned_input.boards.iter_mut() {
                board.draw(current_draw);
            }
            if owned_input.boards.len() == 1 && owned_input.boards[0].check() {
                return owned_input.boards[0].score(current_draw);
            } else if owned_input.boards.len() > 1 {
                owned_input.boards.retain(|x| !x.check());
            }
        }
        unreachable!();
    }
}
