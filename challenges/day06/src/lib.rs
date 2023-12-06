use std::fmt::Display;

use aoc::{Challenge, Parser as ChallengeParser};
use arrayvec::ArrayVec;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution {
    time_list: ArrayVec<u64, 4>,
    time_join: u64,
    dist_list: ArrayVec<u64, 4>,
    dist_join: u64,
}

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        const PREFIX_LEN: usize = "Distance: ".len();
        let bytes = input.as_bytes();
        let line_len = bytes.len() / 2;
        let (time, dist) = bytes.split_at(line_len);
        let (time_list, time_join) = parse_line(&time[PREFIX_LEN..line_len - 1]);
        let (dist_list, dist_join) = parse_line(&dist[PREFIX_LEN..line_len - 1]);

        Ok((
            "",
            Self {
                time_join,
                time_list,
                dist_join,
                dist_list,
            },
        ))
    }
}

fn parse_line(s: &[u8]) -> (ArrayVec<u64, 4>, u64) {
    let mut list = ArrayVec::new();
    let mut join = 0_u64;
    let mut indv = 0_u64;

    for &b in s {
        if b == b' ' && indv != 0 {
            list.push(indv);
            indv = 0;
        } else if b != b' ' {
            indv *= 10;
            join *= 10;
            let x = (b & 0xf) as u64;
            indv += x;
            join += x;
        }
    }

    list.push(indv);
    (list, join)
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> impl Display {
        if self.time_list.len() == 4 && self.dist_list.len() == 4 {
            std::iter::zip(self.time_list, self.dist_list)
                .map(|(t, d)| solve(t, d))
                .product::<u64>()
        } else {
            std::iter::zip(self.time_list, self.dist_list)
                .map(|(t, d)| solve(t, d))
                .product::<u64>()
        }
    }

    fn part_two(self) -> impl Display {
        let t = self.time_join;
        let d = self.dist_join;
        solve(t, d)
    }
}

fn solve(t: u64, d: u64) -> u64 {
    // distance per race = (t-i)*i, where i is how long we spend speeding up the boat
    // solve for i: (t-i)*i > d
    // ti - ii > d
    // ii - ti < d
    // ii - ti - d < 0
    // a = 1, b = -t, c = -d
    // quadratic forumla: disc = bb - 4ac = tt - 4d
    // i = (t +- disc.sqrt()) / 2

    let disc = t * t - 4 * d;
    let disc = (disc as f64).sqrt() - 2.0;
    let upper = ((t as f64 + disc) / 2.0).ceil();
    let lower = ((t as f64 - disc) / 2.0).floor();
    (upper - lower) as u64 + 1
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
        assert_eq!(output.part_two().to_string(), "71503");
    }
}
