#![feature(slice_partition_dedup)]

use std::fmt::{Debug, Display};

use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;

#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
#[repr(C, align(1))]
pub struct Triple(u8, u8, u8);

impl Triple {
    fn into_u8(self) -> u8 {
        (self.0 & 0xf) * 10 + (self.1 & 0xf)
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

#[derive(Debug, PartialEq, Clone)]
pub struct Card<const W: usize, const H: usize> {
    winning: &'static [Triple; W],
    holding: &'static [Triple; H],
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(Vec<u8>);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let colon = input.find(':').unwrap();
        let bar = input.find('|').unwrap();
        let nl = input.find('\n').unwrap();

        let mut output = Vec::with_capacity(input.len() / (nl + 1));

        if colon == 8 {
            for line in input.as_bytes().chunks_exact(nl + 1) {
                let winning = &line[colon + 2..bar];
                let holding = &line[bar + 2..nl + 1];

                let card = Card::<10, 25> {
                    winning: bytemuck::cast_slice(winning).try_into().unwrap(),
                    holding: bytemuck::cast_slice(holding).try_into().unwrap(),
                };

                output.push(card.count() as u8);
            }
        } else {
            for line in input.as_bytes().chunks_exact(nl + 1) {
                let winning = &line[colon + 2..bar];
                let holding = &line[bar + 2..nl + 1];

                let card = Card::<5, 8> {
                    winning: bytemuck::cast_slice(winning).try_into().unwrap(),
                    holding: bytemuck::cast_slice(holding).try_into().unwrap(),
                };

                output.push(card.count() as u8);
            }
        }

        Ok(("", Self(output)))
    }
}

impl<const W: usize, const H: usize> Card<W, H> {
    fn count(self) -> usize {
        // two digits can only go up to 100
        let mut bv = 0u128;
        let expected_len = self.holding.len() + self.winning.len();
        for holding in self.holding {
            bv |= 1 << holding.into_u8();
        }
        for winning in self.winning {
            bv |= 1 << winning.into_u8();
        }
        expected_len - bv.count_ones() as usize
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
        let mut score = 0u32;
        let mut current = 1u32;
        let mut changes = vec![0u32; self.0.len() + 10];

        let mut i = 0;
        for matches in self.0 {
            score += current;
            i += 1;
            changes[i + matches as usize] = changes[i + matches as usize].wrapping_sub(current);
            current = current.wrapping_add(current).wrapping_add(changes[i]);
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
