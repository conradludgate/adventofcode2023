use std::{collections::BTreeMap, fmt::Display};

use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution {
    steps: &'static [u8],
    paths: BTreeMap<u16, [u16; 2]>,
}

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let (steps, rest) = input.split_once('\n').unwrap();
        let mut input = rest.as_bytes();
        let mut paths = BTreeMap::new();
        while input.len() > 17 {
            let (line, r) = input.split_at(17);
            input = r;

            let start = elem(line[1..4].try_into().unwrap());
            let left = elem(line[8..11].try_into().unwrap());
            let right = elem(line[13..16].try_into().unwrap());

            paths.insert(start, [left, right]);
        }

        Ok((
            "",
            Self {
                steps: steps.as_bytes(),
                paths,
            },
        ))
    }
}

const fn elem(x: [u8; 3]) -> u16 {
    let [a, b, c] = x;
    ((c as u16 & 0x1f) << 10) | ((b as u16 & 0x1f) << 5) | (a as u16 & 0x1f)
}

const fn lr(x: u8) -> u8 {
    (x >> 4) & 0x1
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> impl Display {
        const GOAL: u16 = elem(*b"ZZZ");
        let mut state = elem(*b"AAA");
        let mut i = 0;
        while state != GOAL {
            let step = lr(self.steps[i % self.steps.len()]);
            state = self.paths[&state][step as usize];
            i += 1;
        }
        i
    }

    fn part_two(self) -> impl Display {
        let mut x = 1;

        for (&state, _) in self.paths.range(elem(*b"AAA")..=elem(*b"ZZA")) {
            let mut state = state;
            let mut j = 0;
            while state >> 10 != 26 {
                let step = lr(self.steps[j % self.steps.len()]);
                state = self.paths[&state][step as usize];
                j += 1;
            }
            x = (x * j) / gcd(x, j)
        }

        x
    }
}

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y != 0 {
        let tmp = x % y;
        x = y;
        y = tmp;
    }
    x
}

#[cfg(test)]
mod tests {
    use crate::{elem, lr};

    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
    const INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
    const INPUT3: &str = "LR

FFA = (FFB, XXX)
FFB = (XXX, FFZ)
FFZ = (FFB, XXX)
GGA = (GGB, XXX)
GGB = (GGC, GGC)
GGC = (GGZ, GGZ)
GGZ = (GGB, GGB)
XXX = (XXX, XXX)
";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT3).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT1).unwrap().1;
        assert_eq!(output.part_one().to_string(), "2");
        let output = Solution::parse(INPUT2).unwrap().1;
        assert_eq!(output.part_one().to_string(), "6");
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT3).unwrap().1;
        assert_eq!(output.part_two().to_string(), "6");
    }

    #[test]
    fn test_lr() {
        assert_eq!(lr(b'L'), 0);
        assert_eq!(lr(b'R'), 1);
    }

    #[test]
    fn test_elem() {
        let mut i = 0;
        for b in b'A'..=b'Z' {
            i += 1;
            assert_eq!(b & 0x1f, i);
        }

        assert!(elem(*b"ZZA") < elem(*b"AAB"));
        // assert_eq!(elem(*b"ZZA"), (26<<10)+(26<<5)+26);
        // assert_eq!(elem(*b"AAA"), (1<<10)+(1<<5)+1);
    }
}
