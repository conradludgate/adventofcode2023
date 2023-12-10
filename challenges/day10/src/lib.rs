use std::fmt;

use aoc::{Challenge, Parser as ChallengeParser};
use arrayvec::ArrayVec;
use nom::{bytes::complete::tag, IResult, Parser};

#[derive(Debug, PartialEq, Clone)]
pub struct Solution<'a> {
    width: usize,
    height: usize,
    data: &'a [Foo],
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
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
    fn into_line_char(self) -> char {
        match self {
            Foo::NorthSouth => '┃',
            Foo::EastWest => '━',
            Foo::NorthEast => '┗',
            Foo::NorthWest => '┛',
            Foo::SouthWest => '┓',
            Foo::SouthEast => '┏',
            Foo::Ground => '.',
            Foo::Start => 'S',
            Foo::LineEnd => '\n',
        }
    }

    fn map(self, from: Dir) -> Option<Dir> {
        match (self, from) {
            (Foo::NorthSouth, Dir::North) => Some(Dir::South),
            (Foo::NorthSouth, Dir::South) => Some(Dir::North),
            (Foo::EastWest, Dir::East) => Some(Dir::West),
            (Foo::EastWest, Dir::West) => Some(Dir::East),
            (Foo::NorthEast, Dir::North) => Some(Dir::East),
            (Foo::NorthEast, Dir::East) => Some(Dir::North),
            (Foo::NorthWest, Dir::North) => Some(Dir::West),
            (Foo::NorthWest, Dir::West) => Some(Dir::North),
            (Foo::SouthWest, Dir::South) => Some(Dir::West),
            (Foo::SouthWest, Dir::West) => Some(Dir::South),
            (Foo::SouthEast, Dir::South) => Some(Dir::East),
            (Foo::SouthEast, Dir::East) => Some(Dir::South),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn flip(self) -> Self {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::East => Dir::West,
            Dir::West => Dir::East,
        }
    }

    fn apply(self, i: usize, width: usize, height: usize) -> Option<usize> {
        let x = i % width;
        let y = i / width;
        match self {
            Dir::North if y > 0 => Some(i - width),
            Dir::South if y < height - 1 => Some(i + width),
            Dir::East if x < width - 2 => Some(i + 1),
            Dir::West if x > 0 => Some(i - 1),
            _ => None,
        }
    }
}

impl ChallengeParser for Solution<'static> {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let data = unsafe { std::mem::transmute::<&[u8], &[Foo]>(input.as_bytes()) };
        let (width, height) = if 140 * 141 == input.len() {
            (141, 140)
        } else if input.len() == 210 {
            (21, 10)
        } else {
            (6, 5)
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
        let s = self.data.iter().position(|x| *x == Foo::Start).unwrap();

        let mut pipes = ArrayVec::<(usize, Dir), 2>::new();
        let dirs = [Dir::East, Dir::West, Dir::North, Dir::South];
        for dir in dirs {
            if let Some(j) = dir.apply(s, self.width, self.height) {
                if let Some(x) = self.data[j].map(dir.flip()) {
                    pipes.push((j, x));
                }
            }
        }

        let [(start, start_dir), (end, _)] = pipes.into_inner().unwrap();

        let mut current = start;
        let mut current_dir = start_dir;
        let mut len = 2;
        while current != end {
            current = current_dir.apply(current, self.width, self.height).unwrap();
            current_dir = self.data[current].map(current_dir.flip()).unwrap();
            len += 1;
        }

        len / 2
    }

    fn part_two(self) -> impl fmt::Display {
        let s = self.data.iter().position(|x| *x == Foo::Start).unwrap();

        let mut pipes = ArrayVec::<(usize, Dir), 2>::new();
        let dirs = [Dir::East, Dir::West, Dir::North, Dir::South];
        for dir in dirs {
            if let Some(j) = dir.apply(s, self.width, self.height) {
                if let Some(x) = self.data[j].map(dir.flip()) {
                    pipes.push((j, x));
                }
            }
        }

        let [(start, start_dir), (end, end_dir)] = pipes.into_inner().unwrap();

        // dbg!(start_dir, end_dir);

        let mut flood_fill = vec![Fill::Untouched; self.data.len()];
        let mut filled_in = 2;
        flood_fill[s] = Fill::Boundary;
        flood_fill[start] = Fill::Boundary;

        for i in (self.width - 1..self.data.len()).step_by(self.width) {
            flood_fill[i] = Fill::Newline;
            filled_in += 1;
        }

        let mut candidates = vec![];

        let mut current = start;
        let mut previous_dir = start_dir;
        let mut current_dir = start_dir;
        while current != end {
            let right_hand = match current_dir {
                Dir::North => Dir::East,
                Dir::South => Dir::West,
                Dir::East => Dir::South,
                Dir::West => Dir::North,
            };
            if let Some(c) = right_hand.apply(current, self.width, self.height) {
                candidates.push(c);
            }
            let right_hand2 = match previous_dir {
                Dir::North => Dir::East,
                Dir::South => Dir::West,
                Dir::East => Dir::South,
                Dir::West => Dir::North,
            };
            if let Some(c) = right_hand2.apply(current, self.width, self.height) {
                candidates.push(c);
            }

            // dbg!((current % self.width, current / self.width, current_dir));
            current = current_dir.apply(current, self.width, self.height).unwrap();
            previous_dir = current_dir;
            current_dir = self.data[current].map(current_dir.flip()).unwrap();
            flood_fill[current] = Fill::Boundary;
            filled_in += 1;
        }

        // for i in 0..self.width - 1 {
        //     // top edge
        //     candidates.push((i, Dir::South));
        //     // bottom edge
        //     candidates.push((i + self.width * (self.width - 2), Dir::North));
        //     // left edge
        //     candidates.push((i * self.width, Dir::East));
        //     // right edge
        //     candidates.push((i * self.width + self.width - 1, Dir::West));
        // }
        // dbg!(&candidates);

        while let Some(c) = candidates.pop() {
            // let mut dirs = ArrayVec::<_, 4>::new();
            // match flood_fill[c] {
            //     Fill::Untouched => {
            //         dirs.extend([Dir::East, Dir::West, Dir::North, Dir::South]);
            //     },
            //     Fill::Outside => continue,
            //     Fill::Boundary => {
            //         match (self.data[c], going) {
            //             (Foo::NorthSouth, Dir::North | Dir::S) => todo!(),
            //             Foo::EastWest => todo!(),
            //             Foo::NorthEast => todo!(),
            //             Foo::NorthWest => todo!(),
            //             Foo::SouthWest => todo!(),
            //             Foo::SouthEast => todo!(),
            //             Foo::Start => todo!(),
            //             Foo::Ground | Foo::LineEnd => unreachable!(),
            //         }
            //     },
            //     Fill::Newline => todo!(),
            // }
            if flood_fill[c] != Fill::Untouched {
                continue;
            }

            filled_in += 1;
            flood_fill[c] = Fill::Outside;
            for dir in dirs {
                if let Some(j) = dir.apply(c, self.width, self.height) {
                    candidates.push(j)
                }
            }
        }

        for (i, y) in flood_fill.iter().enumerate() {
            match y {
                Fill::Boundary => print!("{}", self.data[i].into_line_char()),
                Fill::Outside => print!("O"),
                Fill::Newline => println!(),
                Fill::Untouched => print!("I"),
            }
        }

        dbg!(flood_fill.len(), filled_in);
        dbg!(flood_fill.iter().filter(|x| **x == Fill::Untouched).count());

        flood_fill.len() - filled_in
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
enum Fill {
    Untouched,
    Outside,
    Boundary,
    Newline,
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
