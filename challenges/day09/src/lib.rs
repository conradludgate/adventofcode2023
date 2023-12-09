use std::fmt;

use aoc::{Challenge, Parser as ChallengeParser};
use nom::{bytes::complete::tag, character::complete::digit1, IResult, Parser};
use nom_supreme::ParserExt;
use parsers::ParserExt2;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(Vec<Vec<i64>>);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let number = tag("-")
            .opt_precedes(digit1.parse_from_str::<i64>())
            .map(|(neg, val)| if neg.is_some() { -val } else { val });

        nom_supreme::multi::collect_separated_terminated(number, tag(" "), tag("\n"))
            .many1()
            .map(Self)
            .parse(input)
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> impl fmt::Display {
        self.0.into_iter().map(predict).sum::<i64>()
    }

    fn part_two(self) -> impl fmt::Display {
        self.0.into_iter().map(predict_back).sum::<i64>()
    }
}

fn predict(mut x: Vec<i64>) -> i64 {
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

fn predict_back(mut x: Vec<i64>) -> i64 {
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

    // x[end..].iter().sum()
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
