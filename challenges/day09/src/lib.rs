use std::{fmt, ops::Range};

use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution {
    all: Vec<i64>,
    ranges: Vec<Range<usize>>,
}

impl ChallengeParser for Solution {
    fn parse(mut input: &'static str) -> IResult<&'static str, Self> {
        let mut last = 0;
        let mut ranges = Vec::with_capacity(200);
        let mut all = Vec::with_capacity(2000);
        while !input.is_empty() {
            loop {
                let i = input.find([' ', '\n']).unwrap();
                let c = input.as_bytes()[i];

                let (start, input2) = input.split_at(i);
                input = &input2[1..];
                all.push(start.parse().unwrap());

                if c == b'\n' {
                    break;
                }
            }

            ranges.push(last..all.len());
            last = all.len();
        }
        Ok(("", Self { all, ranges }))
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(mut self) -> impl fmt::Display {
        self.ranges
            .into_iter()
            .map(|r| predict(&mut self.all[r]))
            .sum::<i64>()
    }

    fn part_two(mut self) -> impl fmt::Display {
        self.ranges
            .into_iter()
            .map(|r| predict_back(&mut self.all[r]))
            .sum::<i64>()
    }
}

fn predict(x: &mut [i64]) -> i64 {
    let mut end = x.len();
    loop {
        end -= 1;
        let mut xor = 0;
        for i in 0..end {
            x[i] = x[i + 1] - x[i];
            xor |= x[i];
        }
        if xor == 0 {
            break;
        }
    }
    x[end..].iter().sum()
}

fn predict_back(x: &mut [i64]) -> i64 {
    let mut end = 0;
    loop {
        end += 1;
        let mut xor = 0;
        for i in (end..x.len()).rev() {
            x[i] -= x[i - 1];
            xor |= x[i - 1];
        }
        if xor == 0 {
            break;
        }
    }
    x[..end].iter().copied().rev().reduce(|a, b| b - a).unwrap()
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one().to_string(), "114");
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two().to_string(), "2");
    }
}
