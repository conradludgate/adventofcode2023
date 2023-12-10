use std::fmt;

use aoc::{Challenge, Parser as ChallengeParser};
use arrayvec::ArrayVec;
use nom::{bytes::complete::tag, IResult, Parser};

#[derive(Debug, PartialEq, Clone)]
pub struct Solution<'a> {
    width: usize,
    data: &'a [Foo],
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
enum Foo {
    NorthSouth = b'|',
    EastWest = b'-',
    NorthEast = b'L',
    NorthWest = b'J',
    SouthWest = b'7',
    SouthEast = b'F',
    Ground = b'.',
    Start = b'S',
    LineEnd = b'\n',
}

impl Foo {
    fn map(self, from: Dir) -> Option<Dir> {
        match (self, from) {
            (Foo::NorthSouth, Dir::North) => Some(Dir::South),
            (Foo::NorthSouth, Dir::South) => Some(Dir::North),
            (Foo::EastWest, Dir::East) => Some(Dir::West),
            (Foo::EastWest, Dir::West) => Some(Dir::East),
            (Foo::NorthEast, Dir::North) => Some(Dir::East),
            (Foo::NorthEast, Dir::East) => Some(Dir::North),
            (Foo::NorthWest, Dir::North) => Some(Dir::West),
            (Foo::NorthWest, Dir::West) => Some(Dir::North),
            (Foo::SouthWest, Dir::South) => Some(Dir::West),
            (Foo::SouthWest, Dir::West) => Some(Dir::South),
            (Foo::SouthEast, Dir::South) => Some(Dir::East),
            (Foo::SouthEast, Dir::East) => Some(Dir::South),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn flip(self) -> Self {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::East => Dir::West,
            Dir::West => Dir::East,
        }
    }

    fn apply(self, i: usize, width: usize) -> Option<usize> {
        let x = i % width;
        let y = i / width;
        match self {
            Dir::North if y > 0 => Some(i - width),
            Dir::South if y < width - 1 => Some(i + width),
            Dir::East if x < width - 1 => Some(i + 1),
            Dir::West if x > 0 => Some(i - 1),
            _ => None,
        }
    }
}

impl ChallengeParser for Solution<'static> {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let data = unsafe { std::mem::transmute::<&[u8], &[Foo]>(input.as_bytes()) };
        let width = if 140 * 141 == input.len() { 141 } else { 6 };
        Ok(("", Self { data, width }))
    }
}

impl Challenge for Solution<'_> {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> impl fmt::Display {
        let s = self.data.iter().position(|x| *x == Foo::Start).unwrap();

        let mut pipes = ArrayVec::<(usize, Dir), 2>::new();
        let dirs = [Dir::East, Dir::West, Dir::North, Dir::South];
        for dir in dirs {
            if let Some(j) = dir.apply(s, self.width) {
                if let Some(x) = self.data[j].map(dir.flip()) {
                    pipes.push((j, x));
                }
            }
        }

        let [(start, start_dir), (end, _)] = pipes.into_inner().unwrap();

        let mut current = start;
        let mut current_dir = start_dir;
        let mut len = 1;
        while current != end {
            current = current_dir.apply(current, self.width).unwrap();
            current_dir = self.data[current].map(current_dir.flip()).unwrap();
            len += 1;
        }

        (len + 1) / 2
    }

    fn part_two(self) -> impl fmt::Display {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
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
        assert_eq!(output.part_two().to_string(), "0");
    }
}
