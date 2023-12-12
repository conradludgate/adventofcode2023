use std::fmt;

use aoc::{Challenge, Parser as ChallengeParser};
use arrayvec::ArrayVec;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution<'a>(Vec<(&'a [Springs], ArrayVec<u8, 8>)>);

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(dead_code)]
enum Springs {
    Operational = b'.',
    Damaged = b'#',
    Unknown = b'?',
}

impl ChallengeParser for Solution<'static> {
    fn parse(mut input: &'static str) -> IResult<&'static str, Self> {
        let mut out = Vec::with_capacity(1000);
        while !input.is_empty() {
            let (springs, rest) = input.split_once(' ').unwrap();
            let (numbers, rest) = rest.split_once('\n').unwrap();
            input = rest;

            let springs = unsafe { std::mem::transmute::<&[u8], &[Springs]>(springs.as_bytes()) };
            let mut out2 = ArrayVec::new();
            let mut n = 0;
            for b in numbers.bytes() {
                match b {
                    b',' => {
                        out2.push(n);
                        n = 0;
                    }
                    _ => {
                        n = 10 * n + (b & 0xf);
                    }
                }
            }
            out2.push(n);
            out.push((springs, out2));
        }

        Ok(("", Self(out)))
    }
}

fn is_valid(x: &[Springs], mut counts: &[u8]) -> bool {
    let mut d = 0;

    for s in x {
        match *s {
            Springs::Damaged => d += 1,
            _ if d > 0 => {
                let Some((d1, rest)) = counts.split_first() else {
                    return false;
                };
                counts = rest;
                if *d1 != d {
                    return false;
                }

                d = 0
            }
            _ => {}
        };
    }
    if d > 0 {
        let Some((d1, rest)) = counts.split_first() else {
            return false;
        };
        counts = rest;
        if *d1 != d {
            return false;
        }
    }
    counts.is_empty()
}

impl Challenge for Solution<'_> {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> impl fmt::Display {
        let mut test = Vec::new();

        let mut sum = 0;
        for line in self.0 {
            // let mut unknown = ArrayVec::<u8, 8>::new();
            // let mut damaged = ArrayVec::<u8, 8>::new();
            let mut u = 0;
            let mut d = 0;

            for s in line.0 {
                match *s {
                    // Springs::Operational if u > 0 => {
                    //     unknown.push(u);
                    //     u = 0
                    // }
                    // Springs::Operational if d > 0 => {
                    //     damaged.push(d);
                    //     d = 0
                    // }
                    Springs::Operational => {}
                    // Springs::Damaged if u > 0 => {
                    //     unknown.push(u);
                    //     u = 0;
                    //     d += 1;
                    // }
                    Springs::Damaged => d += 1,
                    // Springs::Unknown if d > 0 => {
                    //     damaged.push(d);
                    //     d = 0;
                    //     u += 1;
                    // }
                    Springs::Unknown => u += 1,
                };
            }
            let to_fit = line.1.iter().sum::<u8>() - d;
            let spaces = u;

            for mut i in 0u64..(1 << spaces) {
                if i.count_ones() != to_fit as u32 {
                    continue;
                }

                test.clear();
                test.extend_from_slice(line.0);

                for s in &mut test {
                    if *s == Springs::Unknown {
                        if i % 2 == 0 {
                            *s = Springs::Operational;
                        } else {
                            *s = Springs::Damaged;
                        }
                        i >>= 1;
                    }
                }

                sum += is_valid(&test, &line.1) as u32;
            }

            // if d > 0 {
            //     damaged.push(d);
            // } else if u > 0 {
            //     unknown.push(u);
            // }
            // dbg!(damaged, unknown);
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

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one().to_string(), "21");
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two().to_string(), "0");
    }
}
