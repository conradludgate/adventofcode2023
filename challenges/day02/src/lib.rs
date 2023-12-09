use std::fmt::Display;

use aoc::{Challenge, Parser as ChallengeParser};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, sequence::tuple, IResult,
    Parser,
};
use nom_supreme::ParserExt;
use parsers::{number, ParserExt2 as _};

enum Colour {
    Red(u8),
    Green(u8),
    Blue(u8),
}

impl Colour {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        alt((
            number.terminated(tag(" red")).map(Self::Red),
            number.terminated(tag(" green")).map(Self::Green),
            number.terminated(tag(" blue")).map(Self::Blue),
        ))
        .parse(input)
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
struct Round {
    red: u8,
    green: u8,
    blue: u8,
}

impl Extend<Colour> for Round {
    fn extend<T: IntoIterator<Item = Colour>>(&mut self, iter: T) {
        for i in iter {
            match i {
                Colour::Red(r) => self.red = r,
                Colour::Green(g) => self.green = g,
                Colour::Blue(b) => self.blue = b,
            }
        }
    }
}

impl Round {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        Colour::parse.separated_list1(tag(", ")).parse(input)
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
struct Game {
    exceeds_quota: bool,
    fewest_cubes: Round,
}

impl Extend<Round> for Game {
    fn extend<T: IntoIterator<Item = Round>>(&mut self, iter: T) {
        for i in iter {
            self.exceeds_quota |= !i.part_one_valid();
            self.fewest_cubes = self.fewest_cubes.max(i);
        }
    }
}

impl Game {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let prefix = tuple((tag("Game "), digit1, tag(": ")));
        nom_supreme::multi::collect_separated_terminated(Round::parse, tag("; "), tag("\n"))
            .preceded_by(prefix)
            .parse(input)
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Solution {
    game: usize,
    part_one: usize,
    part_two: usize,
}

impl Extend<Game> for Solution {
    fn extend<T: IntoIterator<Item = Game>>(&mut self, iter: T) {
        for i in iter {
            self.game += 1;
            self.part_one += if i.part_one_valid() { self.game } else { 0 };
            self.part_two += i.part_two();
        }
    }
}

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        Game::parse.many1().parse(input)
    }
}

impl Round {
    fn part_one_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn max(self, other: Round) -> Round {
        Round {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
}

impl Game {
    fn part_one_valid(&self) -> bool {
        !self.exceeds_quota
    }

    fn part_two(self) -> usize {
        let r = self.fewest_cubes;
        (r.red as usize) * (r.green as usize) * (r.blue as usize)
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> impl Display {
        self.part_one
    }

    fn part_two(self) -> impl Display {
        self.part_two
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one().to_string(), "8");
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two().to_string(), "2286");
    }
}
