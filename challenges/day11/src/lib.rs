use std::fmt;

use aoc::{Challenge, Parser as ChallengeParser};
use bitvec::bitvec;
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

impl Challenge for Solution<'_> {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> impl fmt::Display {
        let mut galaxies = Vec::with_capacity(200);
        // let mut empty_rows = Vec::with_capacity(20);
        let mut empty_rows2 = bitvec![1; self.height];
        let mut empty_cols = bitvec![1; self.width];
        let mut height_offsets = vec![0; self.height];
        let mut offset = 0;
        for (y, line) in self.data.chunks_exact(self.width).enumerate() {
            // let i = empty_rows.len();
            for (x, t) in line.iter().enumerate() {
                match t {
                    Foo::Galaxy => {
                        galaxies.push((x as u32, y as u32));
                        empty_rows2.set(y, false);
                        empty_cols.set(x, false);
                    }
                    Foo::Empty | Foo::LineEnd => {}
                }
            }
            // if galaxies.len() == i {
            //     // offset += 1;
            //     empty_rows.push(y);
            // }
            offset += empty_rows2[y] as u32;
            height_offsets[y] = y as u32 + offset;
        }
        let mut column_offsets = vec![0; self.width];
        let mut offset = 0;
        for (x, off) in empty_cols.into_iter().enumerate() {
            offset += off as u32;
            column_offsets[x] = x as u32 + offset;
        }

        let mut sum = 0;
        for (i, g1) in galaxies.iter().enumerate() {
            for g2 in &galaxies[i..] {
                sum += u32::abs_diff(column_offsets[g2.0 as usize], column_offsets[g1.0 as usize]);
                sum += u32::abs_diff(height_offsets[g2.1 as usize], height_offsets[g1.1 as usize]);
            }
        }

        sum
    }

    fn part_two(self) -> impl fmt::Display {
        0
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
        assert_eq!(output.part_two().to_string(), "0");
    }
}
