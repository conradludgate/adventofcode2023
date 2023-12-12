use std::fmt;

use aoc::Challenge;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution {
    all: Vec<i64>,
    len: usize,
}

impl<'a> aoc::Parser<'a> for Solution {
    fn parse(mut input: &'a str) -> nom::IResult<&'a str, Self> {
        let mut len = 0;
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

            if len == 0 {
                len = all.len();
            }
        }
        Ok(("", Self { all, len }))
    }
}

impl Challenge for Solution {
    fn part_one(self) -> impl fmt::Display {
        if self.len == 21 {
            self.all.chunks_exact(21).map(formula21).sum::<i64>()
        } else {
            self.all.chunks_exact(6).map(formula6).sum::<i64>()
        }
    }

    fn part_two(self) -> impl fmt::Display {
        if self.len == 21 {
            self.all.chunks_exact(21).map(formula21m1).sum::<i64>()
        } else {
            self.all.chunks_exact(6).map(formula6m1).sum::<i64>()
        }
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
