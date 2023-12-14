use std::collections::hash_map::Entry;

use rustc_hash::FxHashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution {
    rocks: Vec<Rock>,
    width: usize,
    height: usize,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
#[allow(dead_code)]
enum Rock {
    Empty = b'.',
    Cube = b'#',
    Round = b'O',
    LineEnding = b'\n',
}

impl<'a> aoc::Parser<'a> for Solution {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        let width = input.find('\n').unwrap() + 1;
        let height = input.len() / width;
        let rocks = unsafe { std::mem::transmute::<&[u8], &[Rock]>(input.as_bytes()) }.to_vec();

        Ok((
            "",
            Self {
                width,
                height,
                rocks,
            },
        ))
    }
}

impl Solution {
    // #[inline(always)]
    fn slide(
        &mut self,
        x: impl IntoIterator<Item = usize>,
        y: impl IntoIterator<Item = usize> + Clone,
        offset: usize,
    ) {
        for x in x {
            for y in y.clone() {
                let i = x + y;
                let mut extra = 0;
                loop {
                    match self.rocks.get_mut(i.wrapping_add(extra)) {
                        Some(Rock::Empty) => extra = extra.wrapping_add(offset),
                        Some(r @ Rock::Round) => {
                            *r = Rock::Empty;
                            self.rocks[i] = Rock::Round;
                            break;
                        }
                        _ => break,
                    }
                }
            }
        }
    }

    // #[inline(never)]
    fn north(&mut self) {
        let w = self.width;
        self.slide((0..self.height).map(|row| row * w), 0..w - 1, w);
    }
    // #[inline(never)]
    fn south(&mut self) {
        let w = self.width;
        self.slide(
            (0..self.height).rev().map(|row| row * w),
            0..w - 1,
            0usize.wrapping_sub(w),
        );
    }
    // #[inline(never)]
    fn east(&mut self) {
        let w = self.width;
        self.slide(
            (0..w - 1).rev(),
            (0..self.height).map(|row| row * w),
            0usize.wrapping_sub(1),
        );
    }
    // #[inline(never)]
    fn west(&mut self) {
        let w = self.width;
        self.slide(0..w - 1, (0..self.height).map(|row| row * w), 1);
    }

    // #[inline(never)]
    fn north_weight(&self) -> usize {
        let mut sum = 0;
        for row in 0..self.height {
            let mult = self.height - row;

            let row_offset = row * self.width;
            for col in 0..self.width - 1 {
                let col_offset = row_offset + col;
                if self.rocks[col_offset] == Rock::Round {
                    sum += mult;
                }
            }
        }
        sum
    }

    // #[inline(never)]
    fn to_bitset(&self) -> Vec<u64> {
        let mut vec = vec![0; ((self.width - 1) * self.height + 63) / 64];
        let mut offset = 0;
        for d in &self.rocks {
            match d {
                Rock::LineEnding => {
                    continue;
                }
                Rock::Round => {
                    vec[offset / 64] |= 1 << (offset % 64);
                }
                _ => {}
            }
            offset += 1;
        }
        vec
    }

    fn part_one(mut self) -> impl std::fmt::Display {
        self.north();
        self.north_weight()
    }

    fn part_two(mut self) -> impl std::fmt::Display {
        let mut cache = FxHashMap::with_capacity_and_hasher(256, Default::default());
        let mut i = 0;
        let (idx, len) = loop {
            match cache.entry(self.to_bitset()) {
                Entry::Occupied(o) => break (*o.get(), i - *o.get()),
                Entry::Vacant(v) => {
                    v.insert(i);

                    self.north();
                    self.west();
                    self.south();
                    self.east();
                }
            }
            i += 1;
        };

        let goal = 1000000000;
        let remaining = (goal - idx) % len;
        for _ in 0..remaining {
            self.north();
            self.west();
            self.south();
            self.east();
        }

        self.north_weight()
    }
}

// pub fn run(input: &str) -> impl std::fmt::Display {
//     Solution::parse(input).unwrap().1.part_one()
//     Solution::parse(input).unwrap().1.part_two()
// }

impl aoc::Challenge for Solution {
    fn part_one(self) -> impl std::fmt::Display {
        self.part_one()
    }

    fn part_two(self) -> impl std::fmt::Display {
        self.part_two()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::Parser;

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    #[test]
    fn parse() {
        let output = Solution::must_parse(INPUT);
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::must_parse(INPUT);
        assert_eq!(output.part_one().to_string(), "136");
    }

    #[test]
    fn part_two() {
        let output = Solution::must_parse(INPUT);
        assert_eq!(output.part_two().to_string(), "64");
    }
}
