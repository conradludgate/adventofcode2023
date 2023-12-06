use std::fmt::Display;

use aoc::{Challenge, Parser as ChallengeParser};
use nom::{bytes::complete::tag, IResult, Parser};

#[derive(Debug, PartialEq, Clone)]
pub struct Solution {
    time: Vec<u32>,
    dist: Vec<u32>,
}

impl ChallengeParser for Solution {
    fn parse(_input: &'static str) -> IResult<&'static str, Self> {
        Ok((
            "",
            Self {
                // time: vec![7, 15, 30],
                // dist: vec![9, 40, 200],
                time: vec![44, 70, 70, 80],
                dist: vec![283, 1134, 1134, 1491],
            },
        ))
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> impl Display {
        let mut prod = 1;
        for (t, d) in std::iter::zip(self.time, self.dist) {
            // d1 = (t-i) * i = t*i - i*i
            // d1 > d
            // i*i - i*t + d < 0
            //
            // t*i - d > i*i

            let mut sum = 0;
            for i in 0..t {
                let d1 = (t-i)*i;
                sum += (d1 > d) as usize;
            }
            dbg!(sum);
            prod *= sum;
        }
        prod
    }

    fn part_two(self) -> impl Display {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one().to_string(), "288");
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two().to_string(), "0");
    }
}
