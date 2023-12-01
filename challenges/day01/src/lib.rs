use std::fmt::Display;

use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(Vec<Vec<(usize, bool)>>);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let mut output = vec![];
        let numbers = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        for line in input.lines() {
            let mut line_output = vec![];
            let line = line.trim();
            let mut i = 0;
            while i < line.len() {
                let line = &line[i..];
                i += 1;
                if line.starts_with(|c: char| c.is_ascii_digit()) {
                    line_output.push(((line.as_bytes()[0] - b'0') as usize, true));
                } else {
                    for (j, case) in numbers.iter().enumerate() {
                        if line.starts_with(case) {
                            line_output.push((j + 1, false));
                            break;
                        }
                    }
                }
            }
            if !line_output.is_empty() {
                output.push(line_output)
            }
        }
        Ok(("", Self(output)))
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> impl Display {
        self.0
            .into_iter()
            .map(|line| {
                let mut line = line.into_iter().filter_map(|(x, y)| y.then_some(x));
                let last = line.next_back().unwrap();
                let first = line.next().unwrap_or(last);
                first * 10 + last
            })
            .sum::<usize>()
    }

    fn part_two(self) -> impl Display {
        self.0
            .into_iter()
            .map(|line| {
                let mut line = line.into_iter().map(|(x, _)| x);
                let last = line.next_back().unwrap();
                let first = line.next().unwrap_or(last);
                first * 10 + last
            })
            .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet

";

    const INPUT2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen

";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one().to_string(), "142");
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT2).unwrap().1;
        assert_eq!(output.part_two().to_string(), "281");
    }
}
