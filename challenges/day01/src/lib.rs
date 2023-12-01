use std::fmt::Display;

use aoc::{Challenge, Parser as ChallengeParser};
use arrayvec::ArrayVec;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(Vec<ArrayVec<u32, 16>>);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let mut output = Vec::with_capacity(1000);
        let digits3 = [(*b"one", 1), (*b"six", 6), (*b"two", 2)];
        let digits4 = [(*b"five", 5), (*b"four", 4), (*b"nine", 9)];
        let digits5 = [(*b"eight", 8), (*b"seven", 7), (*b"three", 3)];

        for line in input.lines() {
            let mut line_output = ArrayVec::new();
            let line = line.trim();
            for i in 0..line.len() {
                let line = &line[i..];

                if line.starts_with(|c: char| c.is_ascii_digit()) {
                    line_output.push((line.as_bytes()[0] - b'0') as u32);
                } else {
                    if let [a, b, c, d, e, ..] = *line.as_bytes() {
                        let five = [a, b, c, d, e];
                        for (x, j) in digits5 {
                            if five == x {
                                line_output.push(j | 0x10);
                                break;
                            }
                        }
                    }
                    if let [a, b, c, d, ..] = *line.as_bytes() {
                        let four = [a, b, c, d];
                        for (x, j) in digits4 {
                            if four == x {
                                line_output.push(j | 0x10);
                                break;
                            }
                        }
                    }
                    if let [a, b, c, ..] = *line.as_bytes() {
                        let three = [a, b, c];
                        for (x, j) in digits3 {
                            if three == x {
                                line_output.push(j | 0x10);
                                break;
                            }
                        }
                    }
                }
            }
            if !line_output.is_empty() {
                output.push(line_output)
            }
        }
        Ok(("", Self(output)))
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> impl Display {
        self.0
            .into_iter()
            .map(|line| {
                let mut line = line.into_iter().filter(|&x| x < 0x10);
                let last = line.next_back().unwrap();
                let first = line.next().unwrap_or(last);
                first * 10 + last
            })
            .sum::<u32>()
    }

    fn part_two(self) -> impl Display {
        self.0
            .into_iter()
            .map(|line| {
                let mut line = line.into_iter();
                let last = line.next_back().unwrap();
                let first = line.next().unwrap_or(last);
                first * 10 + last
            })
            .sum::<u32>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet

";

    const INPUT2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen

";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one().to_string(), "142");
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT2).unwrap().1;
        assert_eq!(output.part_two().to_string(), "281");
    }
}
