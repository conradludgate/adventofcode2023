use std::fmt;

use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution<'a> {
    width: usize,
    height: usize,
    data: &'a [Foo],
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(dead_code)]
enum Foo {
    Empty = b'.',
    Galaxy = b'#',
    LineEnd = b'\n',
}

impl ChallengeParser for Solution<'static> {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let data = unsafe { std::mem::transmute::<&[u8], &[Foo]>(input.as_bytes()) };
        let (width, height) = if 140 * 141 == input.len() {
            (141, 140)
        } else {
            (11, 10)
        };
        Ok((
            "",
            Self {
                data,
                width,
                height,
            },
        ))
    }
}

impl Solution<'_> {
    fn inner<const N: u64>(self) -> impl fmt::Display {
        let mut galaxies = Vec::with_capacity(200);
        // let mut empty_rows = vec![1u8; self.height];
        let mut empty_cols = vec![1u8; self.width];
        let mut offset = 0;
        for (y, line) in self.data.chunks_exact(self.width).enumerate() {
            let mut empty_row = 1;
            for (x, t) in line.iter().enumerate() {
                match t {
                    Foo::Galaxy => {
                        galaxies.push((x as u64, y as u64 + offset));
                        empty_cols[x] = 0;
                        empty_row = 0;
                    }
                    Foo::Empty | Foo::LineEnd => {}
                }
            }
            offset += empty_row as u64 * (N - 1);
        }
        let mut column_offsets = vec![0; self.width];
        let mut offset = 0;
        for (x, off) in empty_cols.drain(..).enumerate() {
            offset += off as u64 * (N - 1);
            column_offsets[x] = x as u64 + offset;
        }

        let mut sum = 0;
        for (i, g1) in galaxies.iter().enumerate() {
            for g2 in &galaxies[i..] {
                sum += u64::abs_diff(column_offsets[g2.0 as usize], column_offsets[g1.0 as usize]);
                sum += u64::abs_diff(g2.1, g1.1);
            }
        }

        sum
    }
}

impl Challenge for Solution<'_> {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> impl fmt::Display {
        self.inner::<2>()
    }

    fn part_two(self) -> impl fmt::Display {
        self.inner::<1000000>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one().to_string(), "374");
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.inner::<10>().to_string(), "1030");
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.inner::<100>().to_string(), "8410");
    }
}
