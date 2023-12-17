use core::panic;
use std::collections::hash_map::Entry;

use arrayvec::ArrayVec;
use rustc_hash::FxHashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution<'a> {
    widthd: u32,
    width: u16,
    height: u16,
    grid: &'a [u8],
}

impl<'a> aoc::Parser<'a> for Solution<'a> {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        let (width, widthd, height) = if 142 * 141 == input.len() {
            (142, u32::MAX / 142 + 1, 141)
        } else {
            (14, u32::MAX / 14 + 1, 13)
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

impl Dir {
    fn vert(self) -> u16 {
        match self {
            Dir::North => 1 << 15,
            Dir::South => 1 << 15,
            Dir::West => 0,
            Dir::East => 0,
        }
    }
}

fn div_rem(i: u16, width: u16, widthd: u32) -> (u16, u16) {
    let numerator128 = i as u32;
    let multiplied_hi = numerator128 * (widthd >> 16);
    let multiplied_lo = (numerator128 * (widthd as u16 as u32)) >> 16;

    let y = ((multiplied_hi + multiplied_lo) >> 16) as u16;
    let x = i - y * width;
    (x, y)
}

impl Solution<'_> {
    // #[inline(never)]
    fn apply(&self, dir: Dir, i: u16) -> Option<u16> {
        let (x, y) = div_rem(i, self.width, self.widthd);
        match dir {
            Dir::North if y > 0 => Some(i - self.width),
            Dir::South if y < self.height - 1 => Some(i + self.width),
            Dir::East if x < self.width - 2 => Some(i + 1),
            Dir::West if x > 0 => Some(i - 1),
            _ => None,
        }
    }

    fn solve<const MIN: usize, const MAX: usize, const MAX2: usize>(self) -> u16 {
        let end = self.grid.len() as u16 - 2;
        let heuristic = |pos| {
            let pos = pos & 0x7fff;
            let (x, y) = div_rem(self.grid.len() as u16 - 1 - pos, self.width, self.widthd);
            x + y
        };

        let mut to_see = BucketQueue::<_, 512>::new(1024);
        let mut parents: FxHashMap<u16, u16> = FxHashMap::default();
        parents.insert(0, 0);

        for facing in [Dir::East, Dir::South] {
            let mut pos = 0;
            let mut move_cost = 0;
            for run in 1..=MAX {
                let Some(p) = self.apply(facing, pos) else {
                    break;
                };
                pos = p;
                move_cost += (self.grid[pos as usize] & 0xf) as u16;
                if run >= MIN {
                    let successor = facing.vert() | pos;
                    let new_cost = move_cost;
                    match parents.entry(successor) {
                        Entry::Vacant(e) => {
                            e.insert(new_cost);
                        }
                        Entry::Occupied(mut e) => {
                            if *e.get() <= new_cost {
                                continue;
                            }
                            e.insert(new_cost);
                        }
                    }
                    let h = heuristic(successor);

                    to_see.insert((new_cost + h) as usize, (new_cost, successor));
                }
            }
        }

        while let Some((_, (cost, node))) = to_see.extract() {
            let vert = node >> 15 == 1;
            let pos = node & 0x7fff;
            if pos == end {
                return cost;
            }
            if cost > *parents.get(&node).unwrap() {
                continue;
            }

            let dirs = match vert {
                true => [Dir::East, Dir::West],
                false => [Dir::North, Dir::South],
            };

            for facing in dirs {
                let mut pos = pos;
                let mut move_cost = 0;
                for run in 1..=MAX {
                    let Some(p) = self.apply(facing, pos) else {
                        break;
                    };
                    pos = p;
                    move_cost += (self.grid[pos as usize] & 0xf) as u16;
                    if run >= MIN {
                        let successor = (((!vert) as u16) << 15) | pos;
                        let new_cost = cost + move_cost;
                        match parents.entry(successor) {
                            Entry::Vacant(e) => {
                                e.insert(new_cost);
                            }
                            Entry::Occupied(mut e) => {
                                if *e.get() <= new_cost {
                                    continue;
                                }
                                e.insert(new_cost);
                            }
                        }
                        let h = heuristic(successor);

                        to_see.insert((new_cost + h) as usize, (new_cost, successor));
                    }
                }
            }
        }
        panic!();
    }

    fn part_one(self) -> impl std::fmt::Display {
        self.solve::<1, 3, 8>()
    }

    fn part_two(self) -> impl std::fmt::Display {
        self.solve::<4, 10, 16>()
    }
}

pub struct BucketQueue<T, const N: usize> {
    buckets: Vec<ArrayVec<T, N>>,
    min: usize,
}

impl<T: Clone, const N: usize> BucketQueue<T, N> {
    fn new(buckets: usize) -> Self {
        Self {
            buckets: vec![ArrayVec::new_const(); buckets],
            min: buckets,
        }
    }
    fn insert(&mut self, p: usize, t: T) {
        self.min = usize::min(self.min, p);
        self.buckets[p].push(t)
    }
    fn extract(&mut self) -> Option<(usize, T)> {
        while self.min < self.buckets.len() {
            match self.buckets[self.min].pop() {
                Some(t) => return Some((self.min, t)),
                None => self.min += 1,
            }
        }
        None
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
