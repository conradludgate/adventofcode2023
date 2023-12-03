use std::{fmt::Display, process::Output};

use aoc::{Challenge, Parser as ChallengeParser};
use nom::{bytes::complete::tag, IResult, Parser};

#[derive(Debug, PartialEq, Clone)]
pub struct Solution {
    width: usize,
    states: Vec<State>,
}

#[derive(Debug, PartialEq, Clone)]
enum State {
    None,
    Num(usize),
    Symbol,
}

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let width = input.find('\n').unwrap();
        let mut output = Vec::with_capacity(input.len());
        for chunk in input.as_bytes().chunks_exact(width + 1) {
            let chunk = &chunk[..width];
            for b in chunk {
                if b.is_ascii_digit() {
                    output.push(State::Num((b - b'0') as usize));
                } else if *b == b'.' {
                    output.push(State::None);
                } else {
                    output.push(State::Symbol);
                }
            }
        }

        Ok((
            "",
            Self {
                width,
                states: output,
            },
        ))
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> impl Display {
        let mut sum = 0;
        let mut current_num = 0;
        let mut is_next_to = false;
        for (i, state) in self.states.iter().enumerate() {
            match state {
                State::None | State::Symbol => {
                    if is_next_to {
                        sum += current_num;
                        is_next_to = false;
                    }
                    current_num = 0;
                },
                State::Num(s) => {
                    current_num *= 10;
                    current_num += s;
                    is_next_to |= self.is_next_to_symbol(i);
                },
            }
        }
        sum
    }

    fn part_two(self) -> impl Display {
        0
    }
}

impl Solution {
    fn is_next_to_symbol(&self, pos: usize) -> bool {
        let mut outcome = false;
        let left = pos % self.width > 0;
        let right = pos % self.width + 1 < self.width;
        let up = pos >= self.width;
        let down = pos + self.width < self.states.len();

        if left {
            outcome |= self.check(pos - 1);
        }
        if right {
            outcome |= self.check(pos + 1);
        }
        if up {
            outcome |= self.check(pos - self.width);
        }
        if down {
            outcome |= self.check(pos + self.width);
        }
        if left && up {
            outcome |= self.check(pos - 1 - self.width);
        }
        if right && down {
            outcome |= self.check(pos + 1 + self.width);
        }
        if left && down {
            outcome |= self.check(pos - 1 + self.width);
        }
        if right && up {
            outcome |= self.check(pos + 1 - self.width);
        }

        outcome
    }

    fn check(&self, pos: usize) -> bool {
        matches!(self.states[pos], State::Symbol)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one().to_string(), "4361");
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two().to_string(), "0");
    }
}
