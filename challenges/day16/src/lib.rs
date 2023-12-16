use arrayvec::ArrayVec;
use rayon::iter::{ParallelBridge, ParallelIterator};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Solution<'a> {
    widthd: u64,
    width: u32,
    height: u32,
    data: &'a [Space],
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(dead_code)]
enum Space {
    Empty = b'.',
    NorthSouth = b'|',
    EastWest = b'-',
    NorthEast = b'/',
    NorthWest = b'\\',
    LineEnd = b'\n',
}

enum Spaces {
    One(Dir),
    Two(Dir, Dir),
}

impl Space {
    fn map(self, going: Dir) -> Spaces {
        match (self, going) {
            // `.`
            (Space::Empty, going) => Spaces::One(going),

            // `|`
            (Space::NorthSouth, Dir::North | Dir::South) => Spaces::One(going),
            (Space::NorthSouth, Dir::East | Dir::West) => Spaces::Two(Dir::North, Dir::South),

            // `-`
            (Space::EastWest, Dir::East | Dir::West) => Spaces::One(going),
            (Space::EastWest, Dir::North | Dir::South) => Spaces::Two(Dir::East, Dir::West),

            // `/`
            (Space::NorthEast, Dir::South) => Spaces::One(Dir::West),
            (Space::NorthEast, Dir::West) => Spaces::One(Dir::South),
            (Space::NorthEast, Dir::North) => Spaces::One(Dir::East),
            (Space::NorthEast, Dir::East) => Spaces::One(Dir::North),

            // `\`
            (Space::NorthWest, Dir::South) => Spaces::One(Dir::East),
            (Space::NorthWest, Dir::East) => Spaces::One(Dir::South),
            (Space::NorthWest, Dir::North) => Spaces::One(Dir::West),
            (Space::NorthWest, Dir::West) => Spaces::One(Dir::North),

            (Space::LineEnd, _) => unimplemented!(),
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
        let data = unsafe { std::mem::transmute::<&[u8], &[Space]>(input.as_bytes()) };
        let (width, widthd, height) = if 110 * 111 == input.len() {
            (111, u64::MAX / 111 + 1, 110)
        } else {
            (11, u64::MAX / 11 + 1, 10)
        };

        Ok((
            "",
            Self {
                widthd,
                data,
                width,
                height,
            },
        ))
    }
}

impl Solution<'_> {
    fn solve(self, start: u32, dir: Dir) -> usize {
        let mut grid = vec![0u8; self.data.len()];
        let mut beams = Vec::new();
        beams.push((start, dir));

        while let Some((pos, dir)) = beams.pop() {
            if grid[pos as usize] & (1 << dir as u32) != 0 {
                continue;
            }
            grid[pos as usize] |= 1 << dir as u32;
            match self.data[pos as usize].map(dir) {
                Spaces::One(dir) => {
                    // grid[pos as usize] |= 1 << dir as u32;
                    if let Some(pos) = dir.apply(pos, self.width, self.height, self.widthd) {
                        beams.push((pos, dir))
                    }
                }
                Spaces::Two(dir1, dir2) => {
                    // grid[pos as usize] |= 1 << dir1 as u32;
                    if let Some(pos) = dir1.apply(pos, self.width, self.height, self.widthd) {
                        beams.push((pos, dir1))
                    }
                    // grid[pos as usize] |= 1 << dir2 as u32;
                    if let Some(pos) = dir2.apply(pos, self.width, self.height, self.widthd) {
                        beams.push((pos, dir2))
                    }
                }
            }
        }

        // for (energized, space) in std::iter::zip(&grid, self.data) {
        //     match space {
        //         Space::Empty => match energized.count_ones() {
        //             0 => print!("."),
        //             1 => match energized.trailing_zeros() {
        //                 0 => print!("^"),
        //                 1 => print!("v"),
        //                 2 => print!("<"),
        //                 3 => print!(">"),
        //                 _ => unreachable!(),
        //             },
        //             n => print!("{n}"),
        //         },
        //         space => print!("{}", *space as u8 as char),
        //     }
        // }

        // println!();
        // for line in grid.chunks_exact(self.width as usize) {
        //     for col in &line[..(self.width as usize) - 1] {
        //         if *col > 0 {
        //             print!("#");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }

        grid.into_iter().filter(|x| *x > 0).count()
    }

    fn part_one(self) -> impl std::fmt::Display {
        self.solve(0, Dir::East)
    }

    fn part_two(self) -> impl std::fmt::Display {
        let top = (0..self.width - 1).map(|pos| (pos, Dir::South));
        let bottom =
            (0..self.width - 1).map(|pos| ((self.height - 1) * self.width + pos, Dir::North));
        let left = (0..self.height).map(|pos| (pos * self.width, Dir::East));
        let right = (0..self.height).map(|pos| (pos * self.width + self.width - 2, Dir::West));

        top.chain(bottom)
            .chain(left)
            .chain(right)
            .par_bridge()
            .map(|(pos, dir)| self.solve(pos, dir))
            .max()
            .unwrap()
    }
}

// pub fn run(input: &str) -> impl std::fmt::Display {
//     Solution::parse(input).unwrap().1.part_one()
//     Solution::parse(input).unwrap().1.part_two()
// }

impl aoc::Challenge for Solution<'_> {
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

    const INPUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;

    #[test]
    fn parse() {
        let output = Solution::must_parse(INPUT);
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::must_parse(INPUT);
        assert_eq!(output.part_one().to_string(), "46");
    }

    #[test]
    fn part_two() {
        let output = Solution::must_parse(INPUT);
        assert_eq!(output.part_two().to_string(), "51");
    }
}
