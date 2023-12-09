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
        let mut all = Vec::with_capacity(200 * 21);
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

    fn part_one(self) -> impl fmt::Display {
        self.ranges
            .into_iter()
            .map(|r| {
                if r.len() == 21 {
                    formula21(&self.all[r])
                } else {
                    formula6(&self.all[r])
                }
            })
            .sum::<i64>()
    }

    fn part_two(self) -> impl fmt::Display {
        self.ranges
            .into_iter()
            .map(|r| {
                if r.len() == 21 {
                    formula21m1(&self.all[r])
                } else {
                    formula6m1(&self.all[r])
                }
            })
            .sum::<i64>()
    }
}

const fn bases<const N: usize>(x: i64) -> [i64; N] {
    let mut out = [0; N];

    let mut l = 1;
    let mut m = 0;
    while m < N as i64 {
        l *= (x - m) as i128;
        m += 1;
    }

    let mut j = 0;
    while j < N as i64 {
        let mut wj = 1;
        let mut m = 0;
        while m < N as i64 {
            if m == j {
                m += 1;
                continue;
            }
            wj *= j - m;
            m += 1;
        }

        out[j as usize] = (l / wj as i128) as i64 / (x - j);
        j += 1;
    }

    out
}

const BASES21: [i64; 21] = bases(21);
const BASES6: [i64; 6] = bases(6);
const BASESM21: [i64; 21] = bases(-1);
const BASESM6: [i64; 6] = bases(-1);

fn formula21(ys: &[i64]) -> i64 {
    assert_eq!(ys.len(), 21);

    let mut sum = 0;
    for j in 0..21 {
        sum += ys[j] * BASES21[j];
    }
    sum
}

fn formula6(ys: &[i64]) -> i64 {
    assert_eq!(ys.len(), 6);

    let mut sum = 0;
    for j in 0..6 {
        sum += ys[j] * BASES6[j];
    }
    sum
}

fn formula21m1(ys: &[i64]) -> i64 {
    assert_eq!(ys.len(), 21);

    let mut sum = 0;
    for j in 0..21 {
        sum += ys[j] * BASESM21[j];
    }
    sum
}

fn formula6m1(ys: &[i64]) -> i64 {
    assert_eq!(ys.len(), 6);

    let mut sum = 0;
    for j in 0..6 {
        sum += ys[j] * BASESM6[j];
    }
    sum
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
