use std::{borrow::Cow, collections::HashMap, fmt};

use aoc::{Challenge, Parser as ChallengeParser};
// use arrayvec::ArrayVec;
use nom::IResult;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct Line<'a> {
    springs: Cow<'a, [Spring]>,
    runs: Cow<'a, [u8]>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution<'a>(Vec<Line<'a>>);

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
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
                runs: Cow::Owned(out2),
            });
        }

        Ok(("", Self(out)))
    }
}

impl<'a> From<&'a Line<'a>> for LineRef<'a> {
    fn from(value: &'a Line<'a>) -> Self {
        LineRef {
            springs: &value.springs,
            runs: &value.runs,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct LineRef<'a> {
    springs: &'a [Spring],
    runs: &'a [u8],
}

impl<'a> Line<'a> {
    fn solve(self) -> u64 {
        let mut cache = HashMap::with_capacity(1024);
        LineRef::from(&self).solve_inner(&mut cache)
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
            runs: runs.into(),
        }
    }
}

impl<'a> LineRef<'a> {
    fn solve_cached(self, cache: &mut HashMap<LineRef<'a>, u64>) -> u64 {
        if let Some(res) = cache.get(&self) {
            *res
        } else {
            let res = self.clone().solve_inner(cache);
            cache.insert(self, res);
            res
        }
    }

    fn solve_inner(mut self, cache: &mut HashMap<LineRef<'a>, u64>) -> u64 {
        while let Some((first, s)) = self.springs.split_first() {
            match first {
                Spring::Operational => self.springs = s,
                Spring::Damaged => match self.skip_damaged_run() {
                    Ok(s) => return s.solve_cached(cache),
                    Err(n) => return n,
                },
                Spring::Unknown => {
                    // either this unknown spring is operational and we don't consume a run
                    let a = LineRef::<'a> {
                        springs: s,
                        runs: self.runs,
                    }
                    .solve_cached(cache);

                    // or the spring is damaged and we satisfy the entire run
                    let b = match self.skip_damaged_run() {
                        Ok(s) => s.solve_cached(cache),
                        Err(n) => n,
                    };

                    return a + b;
                }
            }
        }

        self.runs.is_empty() as u64
    }

    fn skip_damaged_run(self) -> Result<Self, u64> {
        let Some((&run, runs)) = self.runs.split_first() else {
            return Err(0);
        };
        let run = run as usize;
        assert_ne!(run, 0);

        if self.springs.len() < run {
            return Err(0);
        }
        let (start, rest_springs) = self.springs.split_at(run);
        if start.contains(&Spring::Operational) {
            return Err(0);
        }
        let Some((next_spring, springs)) = rest_springs.split_first() else {
            return Err(runs.is_empty() as u64);
        };
        if *next_spring == Spring::Damaged {
            return Err(0);
        }

        Ok(LineRef { springs, runs })
    }
}

impl Challenge for Solution<'_> {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> impl fmt::Display {
        self.0.into_par_iter().map(Line::solve).sum::<u64>()
    }

    fn part_two(self) -> impl fmt::Display {
        self.0
            .into_par_iter()
            .map(Line::into_part_two)
            .map(Line::solve)
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
