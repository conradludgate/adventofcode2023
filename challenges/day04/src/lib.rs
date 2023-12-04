#![feature(slice_partition_dedup)]

use std::fmt::{Debug, Display};

use aoc::{Challenge, Parser as ChallengeParser};
use bitvec::bitarr;
use nom::IResult;

#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
#[repr(C, align(1))]
pub struct Triple(u8, u8, u8);

impl Triple {
    fn into_u8(self) -> u8 {
        let tens = if self.0 == b' ' { 0 } else { self.0 - b'0' };
        tens * 10 + (self.1 - b'0')
    }
}

impl Debug for Triple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_u8())
    }
}

impl PartialEq for Triple {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl Eq for Triple {}
impl PartialOrd for Triple {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Triple {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0).then(self.1.cmp(&other.1))
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Card {
    winning: &'static [Triple],
    holding: &'static [Triple],
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(Vec<u8>);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let colon = input.find(':').unwrap();
        let bar = input.find('|').unwrap();
        let nl = input.find('\n').unwrap();

        let mut output = Vec::with_capacity(input.len() / (nl + 1));
        for line in input.as_bytes().chunks_exact(nl + 1) {
            let mut card = Card::default();
            let winning = &line[colon + 2..bar];
            let holding = &line[bar + 2..nl + 1];

            card.winning = bytemuck::cast_slice(winning);
            card.holding = bytemuck::cast_slice(holding);

            output.push(card.count() as u8);
        }

        Ok(("", Self(output)))
    }
}

impl Card {
    fn count(self) -> usize {
        // two digits can only go up to 100
        let mut bv = bitarr![0; 128];
        let expected_len = self.holding.len() + self.winning.len();
        for holding in self.holding {
            bv.set(holding.into_u8() as usize, true);
        }
        for winning in self.winning {
            bv.set(winning.into_u8() as usize, true);
        }
        expected_len - bv.count_ones()
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> impl Display {
        self.0
            .into_iter()
            .map(|len| if len == 0 { 0 } else { 1 << (len - 1) })
            .sum::<usize>()
    }

    fn part_two(self) -> impl Display {
        let mut score = self.0.len() as u32;
        let mut duplicates = vec![1u32; self.0.len()];

        for (i, len) in self.0.into_iter().enumerate() {
            let dup = duplicates[i];
            for j in (i + 1)..=(i + len as usize) {
                if j < duplicates.len() {
                    score += dup;
                    duplicates[j] += dup;
                }
            }
        }

        score
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one().to_string(), "13");
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two().to_string(), "30");
    }
}
