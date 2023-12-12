use std::{borrow::Cow, fmt};

use aoc::{Challenge, Parser as ChallengeParser};
// use arrayvec::ArrayVec;
use nom::IResult;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rustc_hash::FxHashMap;

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

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
struct LineRef<'a> {
    springs: &'a [Spring],
    runs: &'a [u8],
    min_len: usize,
}

impl core::hash::Hash for LineRef<'_> {
    fn hash<H: core::hash::Hasher>(&self, ra_expand_state: &mut H) {
        let LineRef {
            springs,
            runs,
            min_len,
        } = self;
        {
            ra_expand_state.write(unsafe { std::mem::transmute::<&[Spring], &[u8]>(springs) });
            ra_expand_state.write(runs);
            min_len.hash(ra_expand_state);
        }
    }
}

fn trim_back(mut x: &[Spring]) -> &[Spring] {
    while let Some((Spring::Operational, y)) = x.split_last() {
        x = y;
    }
    x
}

impl<'a> Line<'a> {
    fn as_ref(&self) -> LineRef<'_> {
        LineRef {
            springs: trim_back(&self.springs),
            runs: &self.runs,
            min_len: self.runs.iter().sum::<u8>() as usize + self.runs.len() - 1,
        }
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
    fn solve_cached(self, cache: &mut FxHashMap<LineRef<'a>, u64>) -> u64 {
        if let Some(res) = cache.get(&self) {
            *res
        } else {
            let res = self.solve_inner(cache);
            cache.insert(self, res);
            res
        }
    }

    fn solve_inner(mut self, cache: &mut FxHashMap<LineRef<'a>, u64>) -> u64 {
        loop {
            if self.springs.len() < self.min_len {
                return 0;
            }
            let res = match self.springs.split_first() {
                None => self.runs.is_empty() as u64,
                Some((Spring::Operational, s)) => {
                    self.springs = s;
                    continue;
                }
                Some((Spring::Damaged, _)) => match self.skip_damaged_run() {
                    Some(s) => s.solve_cached(cache),
                    None => 0,
                },
                Some((Spring::Unknown, s)) => {
                    // either the spring is damaged and we consume the entire run
                    let a = match self.skip_damaged_run() {
                        Some(s) => s.solve_cached(cache),
                        None => 0,
                    };

                    // or the spring is operational and we don't consume a run
                    self.springs = s;
                    let b = self.solve_cached(cache);

                    a + b
                }
            };
            break res;
        }
    }

    fn skip_damaged_run(self) -> Option<Self> {
        let Some((&run, runs)) = self.runs.split_first() else {
            return None;
        };
        let run = run as usize;

        if self.springs.len() < run {
            return None;
        }
        let (start, rest_springs) = self.springs.split_at(run);
        if start.contains(&Spring::Operational) {
            return None;
        }
        match rest_springs.split_first() {
            Some((Spring::Damaged, _)) => None,
            Some((Spring::Operational | Spring::Unknown, springs)) => Some(LineRef {
                springs,
                runs,
                min_len: (self.min_len - run).saturating_sub(1),
            }),
            None => Some(LineRef {
                springs: &[],
                runs,
                min_len: (self.min_len - run).saturating_sub(1),
            }),
        }
    }
}

impl Challenge for Solution<'_> {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> impl fmt::Display {
        let this = self.0;

        this.par_iter()
            .map(|l| l.as_ref())
            .map_init(
                || FxHashMap::with_capacity_and_hasher(1024, Default::default()),
                |cache, l| l.solve_cached(cache),
            )
            .sum::<u64>()
    }

    fn part_two(self) -> impl fmt::Display {
        let this = self
            .0
            .into_iter()
            .map(Line::into_part_two)
            .collect::<Vec<_>>();

        this.par_iter()
            .map(|l| l.as_ref())
            .map_init(
                || FxHashMap::with_capacity_and_hasher(1024, Default::default()),
                |cache, l| l.solve_cached(cache),
            )
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
