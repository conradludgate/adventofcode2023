use std::fmt::Display;

use aoc::{Challenge, Parser as ChallengeParser};
use nom::{branch::alt, bytes::complete::tag, sequence::tuple, IResult, Parser};
use parsers::{number, ParserExt};

enum Colour {
    Red(u8),
    Green(u8),
    Blue(u8),
}

impl Colour {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        alt((
            number.followed_by(tag(" red")).map(Self::Red),
            number.followed_by(tag(" green")).map(Self::Green),
            number.followed_by(tag(" blue")).map(Self::Blue),
        ))
        .parse(input)
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
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

#[derive(Debug, PartialEq, Clone)]
struct Game(Vec<Round>);

impl Game {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let rounds = Round::parse.separated_list1(tag("; "));
        let prefix = tuple((tag("Game "), number::<u8>, tag(": ")));

        rounds.preceded_by(prefix).map(Self).parse(input)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(Vec<Game>);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        Game::parse.lines().map(Self).parse(input)
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
        self.0.iter().all(Round::part_one_valid)
    }

    fn part_two(self) -> usize {
        let r = self.0.into_iter().reduce(Round::max).unwrap();
        (r.red as usize) * (r.green as usize) * (r.blue as usize)
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> impl Display {
        self.0
            .iter()
            .enumerate()
            .filter(|(_, x)| x.part_one_valid())
            .map(|(i, _)| i + 1)
            .sum::<usize>()
    }

    fn part_two(self) -> impl Display {
        self.0.into_iter().map(Game::part_two).sum::<usize>()
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
