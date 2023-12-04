#![feature(slice_partition_dedup)]

use std::fmt::Display;

use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Card {
    winning: Vec<[u8; 2]>,
    holding: Vec<[u8; 2]>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(Vec<Card>);

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
            card.winning = winning.chunks_exact(3).map(|x| [x[0], x[1]]).collect();
            card.holding = holding.chunks_exact(3).map(|x| [x[0], x[1]]).collect();
            output.push(card);
        }

        Ok(("", Self(output)))
    }
}

impl Card {
    fn count(mut self) -> usize {
        // self.winning.sort();
        // self.holding.sort();
        // self.winning.dedup();
        // self.holding.dedup();

        self.winning.extend(self.holding);
        self.winning.sort();
        let len = self.winning.partition_dedup().1.len();
        if len == 0 {
            0
        } else {
            1 << (len - 1)
        }
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> impl Display {
        self.0.into_iter().map(Card::count).sum::<usize>()
    }

    fn part_two(self) -> impl Display {
        0
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
        assert_eq!(output.part_two().to_string(), "0");
    }
}
