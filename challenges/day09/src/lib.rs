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
            .map(|r| predict(&mut self.all[r], 1, 0))
            .sum::<i64>()
    }

    fn part_two(mut self) -> impl fmt::Display {
        self.ranges
            .into_iter()
            .map(|r| predict_back2(&mut self.all[r], 1, 0, 1))
            .sum::<i64>()
    }
}

fn predict(x: &mut [i64], j: usize, sum: i64) -> i64 {
    let sum = sum + x[x.len() - 1];
    for i in x.len() - j..x.len() {
        x[i] -= x[i - 1];
    }
    if x[j..].iter().all(|x| *x == 0) {
        sum
    } else {
        predict(x, j + 1, sum)
    }
}

fn predict_back2(x: &mut [i64], j: usize, sum: i64, offset: i64) -> i64 {
    let sum = sum + offset * x[0];
    for i in (0..j).rev() {
        x[i] = x[i + 1] - x[i];
    }
    if x[..x.len() - j].iter().all(|x| *x == 0) {
        sum
    } else {
        predict_back2(x, j + 1, sum, -offset)
    }
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
