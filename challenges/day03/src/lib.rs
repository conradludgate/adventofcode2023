use std::fmt::Display;

use aoc::{Challenge, Parser as ChallengeParser};
use bitvec::{bitvec, vec::BitVec};
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution {
    width: isize,
    nums: Vec<u8>,
    gears: BitVec,
    symbols: BitVec,
}

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let width = input.find('\n').unwrap();
        let height = input.len() / (width + 1);

        let mut nums = vec![255; (width + 2) * (height + 2)];
        let mut gears = bitvec![0; (width+2) * (height + 2)];
        let mut symbols = bitvec![0; (width+2) * (height + 2)];
        let mut pos = width + 3;
        for chunk in input.as_bytes().chunks_exact(width + 1) {
            let chunk = &chunk[..width];
            for b in chunk {
                if b.is_ascii_digit() {
                    nums[pos] = *b - b'0';
                } else if *b == b'*' {
                    gears.set(pos, true);
                    symbols.set(pos, true);
                } else if *b != b'.' {
                    symbols.set(pos, true);
                }
                pos += 1;
            }
            pos += 2;
        }

        Ok((
            "",
            Self {
                width: width as isize + 2,
                nums,
                gears,
                symbols,
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
        for (i, &num) in self.nums.iter().enumerate() {
            match num {
                255 => {
                    if is_next_to {
                        sum += current_num;
                        is_next_to = false;
                    }
                    current_num = 0;
                }
                s => {
                    is_next_to = is_next_to || {
                        if current_num == 0 {
                            self.is_next_to_symbol(i)
                        } else {
                            self.is_next_to_symbol_skip(i)
                        }
                    };
                    current_num *= 10;
                    current_num += s as usize;
                }
            }
        }
        sum
    }

    fn part_two(self) -> impl Display {
        // no part numbers are > 999
        let mut spots = bitvec![0; 15]; // 3 * (3+2);
        let mut gears: Vec<GearRatio> = vec![GearRatio::One(0); self.gears.len()];
        let mut current_num = 0;
        let mut len = 0;
        for (i, &num) in self.nums.iter().enumerate() {
            match num {
                255 => {
                    if len != 0 {
                        for (j, x) in spots.drain(..).enumerate() {
                            if x {
                                // 0369c <- l = -1
                                // 147ad <- l = 0
                                // 258be <- l = 1
                                let k = (j / 3) as isize - 1 - len as isize;
                                let l = (j % 3) as isize - 1;
                                let i = i.wrapping_add_signed(k + l * self.width);

                                let gear: &mut GearRatio = &mut gears[i];
                                gear.insert(current_num);
                            }
                        }
                        spots.resize(15, false);

                        current_num = 0;
                        len = 0;
                    }
                }
                s => {
                    if len == 0 {
                        self.is_next_to_gear(i, &mut spots);
                    } else {
                        self.is_next_to_gear_skip(i, 6 + 3 * len, &mut spots)
                    }
                    len += 1;
                    current_num *= 10;
                    current_num += s as usize;
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
        let diffs: [isize; 8] = [
            -self.width - 1,
            -self.width,
            -self.width + 1,
            -1,
            1,
            self.width - 1,
            self.width,
            self.width + 1,
        ];

        for diff in diffs {
            if self.symbols[pos.wrapping_add_signed(diff)] {
                return true;
            }
        }

        false
    }

    fn is_next_to_symbol_skip(&self, pos: usize) -> bool {
        let diffs: [isize; 3] = [-self.width + 1, 1, self.width + 1];

        for diff in diffs {
            if self.symbols[pos.wrapping_add_signed(diff)] {
                return true;
            }
        }

        false
    }

    fn is_next_to_gear(&self, pos: usize, spaces: &mut BitVec) {
        let diffs: [isize; 9] = [
            -self.width - 1,
            -1,
            self.width - 1,
            -self.width,
            0,
            self.width,
            -self.width + 1,
            1,
            self.width + 1,
        ];

        for i in 0..9 {
            let diff = diffs[i];
            spaces.set(i, self.gears[pos.wrapping_add_signed(diff)]);
        }
    }

    fn is_next_to_gear_skip(&self, pos: usize, offset: usize, spaces: &mut BitVec) {
        let diffs: [isize; 3] = [-self.width + 1, 1, self.width + 1];

        for i in 0..3 {
            let diff = diffs[i];
            spaces.set(i + offset, self.gears[pos.wrapping_add_signed(diff)]);
        }
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
