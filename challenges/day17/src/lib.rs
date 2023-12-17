use arrayvec::ArrayVec;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution<'a> {
    widthd: u64,
    width: u32,
    height: u32,
    grid: &'a [u8],
}

impl<'a> aoc::Parser<'a> for Solution<'a> {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        let (width, widthd, height) = if 142 * 141 == input.len() {
            (142, u64::MAX / 142 + 1, 141)
        } else {
            (14, u64::MAX / 14 + 1, 13)
        };

        Ok((
            "",
            Self {
                grid: input.as_bytes(),
                widthd,
                width,
                height,
            },
        ))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
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

impl Solution<'_> {
    fn solve<const MIN: usize, const MAX: usize>(self) -> u32 {
        #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        struct State {
            facing: Option<Dir>,
            pos: u32,
        }
        let start = State {
            facing: None,
            pos: 0,
        };
        pathfinding::directed::astar::astar(
            &start,
            |state| {
                let mut outputs = Vec::<(State, u32)>::new();
                match state.facing {
                    None => {
                        for facing in [Dir::East, Dir::South] {
                            let mut pos = state.pos;
                            let mut cost = 0;
                            for run in 1..=MAX {
                                let Some(p) =
                                    facing.apply(pos, self.width, self.height, self.widthd)
                                else {
                                    break;
                                };
                                pos = p;
                                cost += (self.grid[pos as usize] & 0xf) as u32;
                                if run >= MIN {
                                    outputs.push((
                                        State {
                                            facing: Some(facing),
                                            pos,
                                        },
                                        cost,
                                    ));
                                }
                            }
                        }
                    }
                    Some(facing) => {
                        let (dir1, dir2) = match facing {
                            Dir::North | Dir::South => (Dir::East, Dir::West),
                            Dir::East | Dir::West => (Dir::North, Dir::South),
                        };
                        for facing in [dir1, dir2] {
                            let mut pos = state.pos;
                            let mut cost = 0;
                            for run in 1..=MAX {
                                let Some(p) =
                                    facing.apply(pos, self.width, self.height, self.widthd)
                                else {
                                    break;
                                };
                                pos = p;
                                cost += (self.grid[pos as usize] & 0xf) as u32;
                                if run >= MIN {
                                    outputs.push((
                                        State {
                                            facing: Some(facing),
                                            pos,
                                        },
                                        cost,
                                    ));
                                }
                            }
                        }
                    }
                }
                outputs
            },
            |state| {
                let (x, y) = div_rem(
                    self.grid.len() as u32 - 1 - state.pos,
                    self.width,
                    self.widthd,
                );
                x + y
            },
            |state| state.pos as usize == self.grid.len() - 2,
        )
        .unwrap()
        .1
    }

    fn part_one(self) -> impl std::fmt::Display {
        self.solve::<1, 3>()
    }

    fn part_two(self) -> impl std::fmt::Display {
        self.solve::<4, 10>()
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

    const INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

    #[test]
    fn parse() {
        let output = Solution::must_parse(INPUT);
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::must_parse(INPUT);
        assert_eq!(output.part_one().to_string(), "102");
    }

    #[test]
    fn part_two() {
        let output = Solution::must_parse(INPUT);
        assert_eq!(output.part_two().to_string(), "94");
    }
}
