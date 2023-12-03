use std::{collections::HashSet, fmt::Display, process::Output};

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
    Gear,
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
                } else if *b == b'*' {
                    output.push(State::Gear);
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
                State::None | State::Symbol | State::Gear => {
                    if is_next_to {
                        sum += current_num;
                        is_next_to = false;
                    }
                    current_num = 0;
                }
                State::Num(s) => {
                    current_num *= 10;
                    current_num += s;
                    is_next_to |= self.is_next_to_symbol(i);
                }
            }
        }
        sum
    }

    fn part_two(self) -> impl Display {
        let mut spots = HashSet::new();
        let mut gears: Vec<GearRatio> = vec![GearRatio::One(0); self.states.len()];
        let mut current_num = 0;
        for (i, state) in self.states.iter().enumerate() {
            match state {
                State::None | State::Symbol | State::Gear => {
                    for spot in spots.drain() {
                        // why is this necessary????
                        let gear: &mut GearRatio = &mut gears[spot];
                        gear.insert(current_num);
                    }
                    current_num = 0;
                }
                State::Num(s) => {
                    current_num *= 10;
                    current_num += s;
                    self.is_next_to_gear(i, &mut spots);
                }
            }
        }
        gears.into_iter().map(GearRatio::eval).sum::<usize>()
    }
}

#[derive(Clone, Copy)]
enum GearRatio {
    One(usize),
    Two(usize),
}

impl GearRatio {
    fn insert(&mut self, x: usize) {
        *self = match *self {
            GearRatio::One(0) => GearRatio::One(x),
            GearRatio::One(y) => GearRatio::Two(x * y),
            GearRatio::Two(_) => GearRatio::Two(0),
        }
    }
    fn eval(self) -> usize {
        match self {
            GearRatio::One(_) => 0,
            GearRatio::Two(x) => x,
        }
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
        matches!(self.states[pos], State::Symbol | State::Gear)
    }

    fn is_next_to_gear(&self, pos: usize, spaces: &mut HashSet<usize>) {
        let left = pos % self.width > 0;
        let right = pos % self.width + 1 < self.width;
        let up = pos >= self.width;
        let down = pos + self.width < self.states.len();

        if left && self.check_gear(pos - 1) {
            spaces.insert(pos - 1);
        }
        if right && self.check_gear(pos + 1) {
            spaces.insert(pos + 1);
        }
        if up && self.check_gear(pos - self.width) {
            spaces.insert(pos - self.width);
        }
        if down && self.check_gear(pos + self.width) {
            spaces.insert(pos + self.width);
        }
        if left && up && self.check_gear(pos - 1 - self.width) {
            spaces.insert(pos - 1 - self.width);
        }
        if right && down && self.check_gear(pos + 1 + self.width) {
            spaces.insert(pos + 1 + self.width);
        }
        if left && down && self.check_gear(pos - 1 + self.width) {
            spaces.insert(pos - 1 + self.width);
        }
        if right && up && self.check_gear(pos + 1 - self.width) {
            spaces.insert(pos + 1 - self.width);
        }
    }

    fn check_gear(&self, pos: usize) -> bool {
        matches!(self.states[pos], State::Gear)
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
        assert_eq!(output.part_two().to_string(), "467835");
    }
}
