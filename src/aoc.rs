use reqwest::header::COOKIE;
use reqwest::StatusCode;
use std::fs;
use std::fs::File;
use std::path::Path;

use std::io::prelude::*;

pub trait AoCSolution<ConvertedType, ReturnType>
where
    ReturnType: std::fmt::Display,
{
    fn year(&self) -> usize {
        2021
    }

    fn day(&self) -> usize;

    fn convert(&self, input: &str) -> ConvertedType;

    fn part1(&self, input: &ConvertedType) -> ReturnType;

    fn part2(&self, input: &ConvertedType) -> ReturnType;

    fn input_path(&self) -> String {
        format!("input/{}/day{}.txt", self.year(), self.day())
    }

    fn download_input(&self) {
        let token_path = Path::new("./session.token");
        let input_path_str = self.input_path();
        let input_path = Path::new(&input_path_str);
        if !token_path.exists() {
            panic!(
                "Set token from session cookie in session.token file or manually download input!"
            );
        }
        let session_token = fs::read_to_string(token_path)
            .expect("Error reading session token from session.token file?");

        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(format!(
                "https://adventofcode.com/{}/day/{}/input",
                self.year(),
                self.day()
            ))
            .header(COOKIE, format!("session={}", session_token.trim()))
            .send();

        match resp {
            Ok(response) => match response.status() {
                StatusCode::OK => {
                    fs::create_dir_all(&(input_path.parent().unwrap())).unwrap();
                    let body = response.text().expect("Error reading input");
                    let mut file = File::create(self.input_path()).unwrap();
                    file.write_all(body.as_bytes()).unwrap();
                }
                error_code => {
                    panic!(
                        "error getting aoc input. StatusCode: {}, Body: {:?}",
                        error_code,
                        response.text()
                    );
                }
            },
            Err(e) => {
                panic!("Failed to get a response. Err({})", e);
            }
        }
    }

    fn input(&self) -> String {
        let input_path_str = self.input_path();
        let input_path = Path::new(&input_path_str);
        if !input_path.exists() {
            self.download_input();
        }
        return fs::read_to_string(&self.input_path())
            .expect("Something went wrong reading input path for this day?");
    }

    fn run(&self) {
        let converted = self.convert(&self.input());
        println!("Part 1: {}", self.part1(&converted));
        println!("Part 2: {}", self.part2(&converted));
    }
}
