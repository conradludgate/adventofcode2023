use std::fmt;

use aoc::Challenge;
use arrayvec::ArrayVec;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution<'a> {
    widthd: u64,
    width: u32,
    height: u32,
    s: u32,
    start: u32,
    start_dir: Dir,
    end: u32,
    data: &'a [Foo],
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(dead_code)]
enum Foo {
    NorthSouth = b'|',
    EastWest = b'-',
    NorthEast = b'L',
    NorthWest = b'J',
    SouthWest = b'7',
    SouthEast = b'F',
    Ground = b'.',
    Start = b'S',
    LineEnd = b'\n',
}

impl Foo {
    fn map(self, from: Dir) -> Option<Dir> {
        match (self, from) {
            (Foo::NorthSouth, Dir::South) => Some(Dir::South),
            (Foo::NorthSouth, Dir::North) => Some(Dir::North),
            (Foo::EastWest, Dir::West) => Some(Dir::West),
            (Foo::EastWest, Dir::East) => Some(Dir::East),
            (Foo::NorthEast, Dir::South) => Some(Dir::East),
            (Foo::NorthEast, Dir::West) => Some(Dir::North),
            (Foo::NorthWest, Dir::South) => Some(Dir::West),
            (Foo::NorthWest, Dir::East) => Some(Dir::North),
            (Foo::SouthWest, Dir::North) => Some(Dir::West),
            (Foo::SouthWest, Dir::East) => Some(Dir::South),
            (Foo::SouthEast, Dir::North) => Some(Dir::East),
            (Foo::SouthEast, Dir::West) => Some(Dir::South),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Dir {
    North,
    South,
    West,
    East,
}

fn div_rem(i: u32, width: u32, widthd: u64) -> (u32, u32) {
    let numerator128 = i as u64;
    let multiplied_hi = numerator128 * (widthd >> 32);
    let multiplied_lo = (numerator128 * (widthd as u32 as u64)) >> 32;

    let y = ((multiplied_hi + multiplied_lo) >> 32) as u32;
    let x = i - y * width;
    (x, y)
}

impl Dir {
    // #[inline(never)]
    fn apply(self, i: u32, width: u32, height: u32, widthd: u64) -> Option<u32> {
        let (x, y) = div_rem(i, width, widthd);
        match self {
            Dir::North if y > 0 => Some(i - width),
            Dir::South if y < height - 1 => Some(i + width),
            Dir::East if x < width - 2 => Some(i + 1),
            Dir::West if x > 0 => Some(i - 1),
            _ => None,
        }
    }
}

impl<'a> aoc::Parser<'a> for Solution<'a> {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        let data = unsafe { std::mem::transmute::<&[u8], &[Foo]>(input.as_bytes()) };
        let (width, widthd, height) = if 140 * 141 == input.len() {
            (141, u64::MAX / 141 + 1, 140)
        } else if input.len() == 210 {
            (21, u64::MAX / 21 + 1, 10)
        } else {
            (6, u64::MAX / 6 + 1, 5)
        };

        let s = data.iter().position(|x| *x == Foo::Start).unwrap() as u32;

        let mut pipes = ArrayVec::<(u32, Dir), 2>::new();
        let dirs = [Dir::East, Dir::West, Dir::North, Dir::South];
        for dir in dirs {
            if let Some(j) = dir.apply(s, width, height, widthd) {
                if let Some(x) = data[j as usize].map(dir) {
                    pipes.push((j, x));
                }
            }
        }
        let [(start, start_dir), (end, _)] = pipes.into_inner().unwrap();

        Ok((
            "",
            Self {
                widthd,
                data,
                width,
                height,
                s,
                start,
                start_dir,
                end,
            },
        ))
    }
}

impl Challenge for Solution<'_> {
    fn part_one(self) -> impl fmt::Display {
        let mut len = 1;
        self.walk(|_| len += 1);
        len / 2
    }

    fn part_two(self) -> impl fmt::Display {
        // shoelace formula:
        // 2*area = sum(y[i] * (x[i-1] - x[i+1]))
        // picks theorem:
        // 2*interior points = 2*area - exterior points - 2

        let mut b = 1;
        let mut area = 0;
        let (mut x2, _) = div_rem(self.end, self.width, self.widthd);
        let (mut x1, mut y1) = div_rem(self.s, self.width, self.widthd);

        self.walk(|current| {
            let (x0, y0) = div_rem(current, self.width, self.widthd);
            area += y1 as i32 * (x2 as i32 - x0 as i32);
            x2 = x1;
            x1 = x0;
            y1 = y0;

            b += 1;
        });

        let (x0, _) = div_rem(self.end, self.width, self.widthd);
        area += y1 as i32 * (x2 as i32 - x0 as i32);

        (area.abs() - b + 2) / 2
    }
}

impl Solution<'_> {
    // #[inline(never)]
    fn walk(&self, mut f: impl FnMut(u32)) {
        let mut current = self.start;
        let mut current_dir = self.start_dir;
        while current != self.end {
            f(current);

            current = current_dir
                .apply(current, self.width, self.height, self.widthd)
                .unwrap();
            current_dir = self.data[current as usize].map(current_dir).unwrap();
        }
        f(current)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

    const INPUT2: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

    const INPUT3: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one().to_string(), "8");
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT2).unwrap().1;
        assert_eq!(output.part_two().to_string(), "8");
        let output = Solution::parse(INPUT3).unwrap().1;
        assert_eq!(output.part_two().to_string(), "10");
    }
}
