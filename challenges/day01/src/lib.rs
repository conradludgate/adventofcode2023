use std::{fmt::Display, ops::Range};

use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(Vec<u8>, Vec<Range<usize>>);

#[allow(clippy::upper_case_acronyms)]
#[repr(u8)]
enum State {
    Init,
    E,    // -> EI
    EI,   // -> EIG
    EIG,  // -> EIGH
    EIGH, // -> T
    F,    // -> FI, FO
    FO,   // -> ON, FOU
    FOU,  // ->
    FI,   // -> FIV
    FIV,  // -> E
    N,    // -> NI
    NI,   // -> NIN
    NIN,  // -> NI, E
    O,    // -> ON
    ON,   // -> E, NI
    S,    // -> SE, SI
    SE,   // -> SEV, EI
    SEV,  // -> SEVE
    SEVE, // -> N, EI
    SI,   // ->
    T,    // -> TW, TH
    TH,   // -> THR
    THR,  // -> THRE
    THRE, // -> E, EI
    TW,   // -> O
}

const CHARS: [u8; 256] = {
    let mut chars = [0; 256];
    let mut i = 1;
    while i < 10 {
        chars[(i + b'0') as usize] = i;
        i += 1;
    }
    while i < 36 {
        chars[(i - 10 + b'a') as usize] = i;
        i += 1;
    }
    chars[b'\n' as usize] = i;
    chars
};

impl State {
    const fn join(self, c: u8) -> usize {
        let x = (self as u8) as u16;
        let y = CHARS[c as usize] as u16;
        ((x << 6) | (y & 0x3f)) as usize
    }
    const fn output(self, y: u8) -> u16 {
        let x = (self as u8) as u16;
        (x << 6) | (y as u16 & 0x3f)
    }
}
const fn join(x: u8, c: u8) -> usize {
    let x = x as u16;
    let y = CHARS[c as usize] as u16;
    ((x << 6) | (y & 0x3f)) as usize
}

const STATE: [u16; 2048] = {
    let mut states = [0; 2048];

    // numbers
    let mut i = 1;
    while i < 10 {
        let mut j = 0;
        while j < 32 {
            let t = (j << 6) | i as u16;
            states[t as usize] = State::Init.output(i);
            j += 1;
        }
        i += 1;
    }

    // newlines
    let mut j = 0;
    while j < 32 {
        let t = (j << 6) | CHARS[b'\n' as usize] as u16;
        states[t as usize] = State::Init.output(0x20);
        j += 1;
    }

    // first
    let mut j = 0;
    while j < 32 {
        states[join(j, b'o')] = State::O.output(0);
        states[join(j, b't')] = State::T.output(0);
        states[join(j, b'f')] = State::F.output(0);
        states[join(j, b's')] = State::S.output(0);
        states[join(j, b'e')] = State::E.output(0);
        states[join(j, b'n')] = State::N.output(0);
        j += 1;
    }

    // second
    states[State::O.join(b'n')] = State::ON.output(0);
    states[State::T.join(b'w')] = State::TW.output(0);
    states[State::T.join(b'h')] = State::TH.output(0);
    states[State::F.join(b'o')] = State::FO.output(0);
    states[State::F.join(b'i')] = State::FI.output(0);
    states[State::S.join(b'i')] = State::SI.output(0);
    states[State::S.join(b'e')] = State::SE.output(0);
    states[State::E.join(b'i')] = State::EI.output(0);
    states[State::N.join(b'i')] = State::NI.output(0);

    // third
    states[State::TH.join(b'r')] = State::THR.output(0);
    states[State::FO.join(b'u')] = State::FOU.output(0);
    states[State::FI.join(b'v')] = State::FIV.output(0);
    states[State::SE.join(b'v')] = State::SEV.output(0);
    states[State::EI.join(b'g')] = State::EIG.output(0);
    states[State::NI.join(b'n')] = State::NIN.output(0);

    // fourth
    states[State::SEV.join(b'e')] = State::SEVE.output(0);
    states[State::EIG.join(b'h')] = State::EIGH.output(0);
    states[State::THR.join(b'e')] = State::THRE.output(0);

    // final
    states[State::ON.join(b'e')] = State::E.output(0x10 + 1);
    states[State::TW.join(b'o')] = State::O.output(0x10 + 2);
    states[State::THRE.join(b'e')] = State::E.output(0x10 + 3);
    states[State::FOU.join(b'r')] = State::Init.output(0x10 + 4);
    states[State::FIV.join(b'e')] = State::E.output(0x10 + 5);
    states[State::SI.join(b'x')] = State::Init.output(0x10 + 6);
    states[State::SEVE.join(b'n')] = State::N.output(0x10 + 7);
    states[State::EIGH.join(b't')] = State::T.output(0x10 + 8);
    states[State::NIN.join(b'e')] = State::E.output(0x10 + 9);

    // recovery
    states[State::FO.join(b'n')] = State::ON.output(0);
    states[State::SE.join(b'i')] = State::EI.output(0);
    states[State::SEVE.join(b'i')] = State::EI.output(0);
    states[State::THRE.join(b'i')] = State::EI.output(0);
    states[State::ON.join(b'i')] = State::NI.output(0);
    states[State::NIN.join(b'i')] = State::NI.output(0);

    states
};

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let mut numbers = Vec::with_capacity(8000);
        let mut lines = Vec::with_capacity(1000);

        let mut state = 0;

        let mut i = 0;
        for b in input.bytes() {
            let j = (state & 0x07c0) | (CHARS[b as usize] as u16 & 0x3f);
            state = STATE[j as usize];
            match state & 0x3f {
                0 => continue,
                0x20 => {
                    if numbers.len() > i {
                        lines.push(i..numbers.len());
                        i = numbers.len();
                    }
                }
                c => numbers.push(c as u8),
            }
        }

        if numbers.len() > i {
            lines.push(i..numbers.len());
        }

        dbg!(numbers.len());

        Ok(("", Self(numbers, lines)))
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> impl Display {
        self.1
            .into_iter()
            .map(|line| {
                let line = &self.0[line];
                let mut line = line.iter().filter(|&&x| x < 0x10);
                let last = line.next_back().unwrap();
                let first = line.next().unwrap_or(last);
                (first * 10 + last) as u32
            })
            .sum::<u32>()
    }

    fn part_two(self) -> impl Display {
        self.1
            .into_iter()
            .map(|line| {
                let line = &self.0[line];
                let mut line = line.iter().map(|&x| x & 0x0f);
                let last = line.next_back().unwrap();
                let first = line.next().unwrap_or(last);
                (first * 10 + last) as u32
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
        let output = Solution::parse(INPUT2).unwrap().1;
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
