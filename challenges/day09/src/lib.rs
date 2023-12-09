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
            .map(|r| formulam1(&self.all[r]))
            .sum::<i64>()
    }
}

const fn l(x: i128, k: usize) -> i128 {
    let mut l = 1;
    let mut m = 0;
    while m <= k {
        l *= x - m as i128;
        m += 1;
    }
    l
}

const L21: [i128; 21] = {
    let mut out = [0; 21];
    let mut k = 0;
    while k <= 20 {
        out[k] = l(21, k);
        k += 1;
    }
    out
};

const LM: [i128; 21] = {
    let mut out = [0; 21];
    let mut k = 0;
    while k <= 20 {
        out[k] = l(-1, k);
        k += 1;
    }
    out
};

const L6: [i128; 6] = {
    let mut out = [0; 6];
    let mut k = 0;
    while k <= 5 {
        out[k] = l(6, k);
        k += 1;
    }
    out
};

const BASES21: [i64; 21 * 21] = {
    let mut out = [0; 21 * 21];
    let mut k = 0;
    while k <= 20 {
        let l = L21[k];

        let mut j = 0;
        while j <= k {
            let wj = lagrange_basis2(j as i64, k) as i128;
            out[k * 21 + j] = (l / wj) as i64 / (21 - j as i64);
            j += 1;
        }
        k += 1;
    }
    out
};

const BASES6: [i64; 6 * 6] = {
    let mut out = [0; 6 * 6];
    let mut k = 0;
    while k <= 5 {
        let l = L6[k];

        let mut j = 0;
        while j <= k {
            let wj = lagrange_basis2(j as i64, k) as i128;
            out[k * 6 + j] = (l / wj) as i64 / (6 - j as i64);
            j += 1;
        }
        k += 1;
    }
    out
};

const BASESM1: [i64; 21 * 21] = {
    let mut out = [0; 21 * 21];
    let mut k = 0;
    while k <= 20 {
        let l = LM[k];

        let mut j = 0;
        while j <= k {
            let wj = lagrange_basis2(j as i64, k) as i128;
            out[k * 21 + j] = (l / wj) as i64 / (-1 - j as i64);
            j += 1;
        }
        k += 1;
    }
    out
};

const fn lagrange_basis2(j: i64, k: usize) -> i64 {
    let mut prod = 1;
    let mut m = 0;
    while m <= k as i64 {
        if m == j {
            m += 1;
            continue;
        }
        prod *= j - m;
        m += 1;
    }
    prod
}

fn formula21(ys: &[i64]) -> i64 {
    let k = ys.len() - 1;

    let mut sum = 0;
    for j in 0..=k {
        sum += ys[j] * BASES21[k * 21 + j];
    }
    sum
}

fn formula6(ys: &[i64]) -> i64 {
    let k = ys.len() - 1;

    let mut sum = 0;
    for j in 0..=k {
        sum += ys[j] * BASES6[k * 6 + j];
    }
    sum
}

fn formulam1(ys: &[i64]) -> i64 {
    let k = ys.len() - 1;

    let mut sum = 0;
    for j in 0..=k {
        sum += ys[j] * BASESM1[k * 21 + j];
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
