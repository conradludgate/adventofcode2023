use std::{borrow::Cow, fmt};

use aoc::{Challenge, Parser as ChallengeParser};
use arrayvec::ArrayVec;
use nom::IResult;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Debug, PartialEq, Clone)]
struct Line<'a> {
    springs: Cow<'a, [Spring]>,
    runs: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution<'a>(Vec<Line<'a>>);

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(dead_code)]
enum Spring {
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

            let springs = unsafe { std::mem::transmute::<&[u8], &[Spring]>(springs.as_bytes()) };
            let mut out2 = Vec::new();
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
            out.push(Line {
                springs: Cow::Borrowed(springs),
                runs: out2,
            });
        }

        Ok(("", Self(out)))
    }
}

fn is_valid(x: &[Spring], mut counts: &[u8]) -> bool {
    let mut d = 0;

    for s in x {
        match *s {
            Spring::Damaged => d += 1,
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

impl Line<'_> {
    fn part_one_orig(self) -> u64 {
        let mut test = Vec::new();

        let mut sum = 0;
        let mut u = 0;
        let mut d = 0;

        for s in &*self.springs {
            match *s {
                Spring::Operational => {}
                Spring::Damaged => d += 1,
                Spring::Unknown => u += 1,
            };
        }
        let to_fit = self.runs.iter().sum::<u8>() - d;
        let spaces = u;

        for mut i in 0u64..(1 << spaces) {
            if i.count_ones() != to_fit as u32 {
                continue;
            }

            test.clear();
            test.extend_from_slice(&*self.springs);

            for s in &mut test {
                if *s == Spring::Unknown {
                    if i % 2 == 0 {
                        *s = Spring::Operational;
                    } else {
                        *s = Spring::Damaged;
                    }
                    i >>= 1;
                }
            }

            sum += is_valid(&test, &self.runs) as u64;
        }

        sum
    }

    fn into_part_two(self) -> Line<'static> {
        let mut springs = Vec::with_capacity(self.springs.len() * 5 + 4);
        springs.extend_from_slice(&self.springs);
        springs.push(Spring::Unknown);
        springs.extend_from_slice(&self.springs);
        springs.push(Spring::Unknown);
        springs.extend_from_slice(&self.springs);
        springs.push(Spring::Unknown);
        springs.extend_from_slice(&self.springs);
        springs.push(Spring::Unknown);
        springs.extend_from_slice(&self.springs);
        let runs = self.runs.repeat(5);
        Line {
            springs: springs.into(),
            runs,
        }
    }
}

impl Challenge for Solution<'_> {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> impl fmt::Display {
        self.0.into_iter().map(Line::part_one_orig).sum::<u64>()
    }

    fn part_two(self) -> impl fmt::Display {
        self.0
            .into_par_iter()
            .map(Line::into_part_two)
            .map(Line::part_one_orig)
            .sum::<u64>()
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
        assert_eq!(output.part_two().to_string(), "525152");
    }
}
