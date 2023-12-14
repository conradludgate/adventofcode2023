#[derive(Debug, PartialEq, Clone)]
pub struct Solution {
    rocks: Vec<Rock>,
    width: usize,
    height: usize,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
#[allow(dead_code)]
enum Rock {
    Empty = b'.',
    Cube = b'#',
    Round = b'O',
    LineEnding = b'\n',
}

impl<'a> aoc::Parser<'a> for Solution {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        let width = input.find('\n').unwrap() + 1;
        let height = input.len() / width;
        let rocks = unsafe { std::mem::transmute::<&[u8], &[Rock]>(input.as_bytes()) }.to_vec();

        Ok((
            "",
            Self {
                width,
                height,
                rocks,
            },
        ))
    }
}

impl Solution {
    fn north(&mut self) {
        for row in 0..self.height {
            let row_offset = row * self.width;
            for col in 0..self.width - 1 {
                let col_offset = row_offset + col;
                let mut extra = 0;
                loop {
                    match self.rocks.get_mut(col_offset + extra) {
                        Some(Rock::Empty) => extra += self.width,
                        Some(r @ Rock::Round) => {
                            *r = Rock::Empty;
                            self.rocks[col_offset] = Rock::Round;
                            break;
                        }
                        _ => break,
                    }
                }
            }
        }
    }
    fn south(&mut self) {
        for row in (0..self.height).rev() {
            let row_offset = row * self.width;
            for col in 0..self.width - 1 {
                let col_offset = row_offset + col;
                let mut extra = 0;
                loop {
                    match self.rocks.get_mut(col_offset.wrapping_add(extra)) {
                        Some(Rock::Empty) => extra = extra.wrapping_sub(self.width),
                        Some(r @ Rock::Round) => {
                            *r = Rock::Empty;
                            self.rocks[col_offset] = Rock::Round;
                            break;
                        }
                        _ => break,
                    }
                }
            }
        }
    }
    fn east(&mut self) {
        for col in (0..self.width - 1).rev() {
            for row in (0..self.height).rev() {
                let row_offset = row * self.width + col;
                let mut extra = 0;
                loop {
                    match self.rocks.get_mut(row_offset.wrapping_add(extra)) {
                        Some(Rock::Empty) => extra = extra.wrapping_sub(1),
                        Some(r @ Rock::Round) => {
                            *r = Rock::Empty;
                            self.rocks[row_offset] = Rock::Round;
                            break;
                        }
                        _ => break,
                    }
                }
            }
        }
    }
    fn west(&mut self) {
        for col in 0..self.width - 1 {
            for row in (0..self.height).rev() {
                let row_offset = row * self.width + col;
                let mut extra = 0;
                loop {
                    match self.rocks.get_mut(row_offset.wrapping_add(extra)) {
                        Some(Rock::Empty) => extra = extra.wrapping_add(1),
                        Some(r @ Rock::Round) => {
                            *r = Rock::Empty;
                            self.rocks[row_offset] = Rock::Round;
                            break;
                        }
                        _ => break,
                    }
                }
            }
        }
    }

    // fn print(&self) {
    //     for line in self.rocks.chunks_exact(self.width) {
    //         for x in line {
    //             print!("{}", *x as u8 as char);
    //         }
    //     }
    //     println!()
    // }

    fn part_one(mut self) -> impl std::fmt::Display {
        let mut sum = 0;
        for row in 0..self.height {
            let mult = self.height - row;

            let row_offset = row * self.width;
            for col in 0..self.width - 1 {
                let col_offset = row_offset + col;
                let mut extra = 0;
                loop {
                    match self.rocks.get_mut(col_offset + extra) {
                        Some(Rock::Empty) => extra += self.width,
                        Some(r @ Rock::Round) => {
                            *r = Rock::Empty;
                            self.rocks[col_offset] = Rock::Round;
                            sum += mult;
                            break;
                        }
                        _ => break,
                    }
                }
            }
        }

        sum
    }

    fn part_two(self) -> impl std::fmt::Display {
        let (len, mut this, idx) =
            pathfinding::directed::cycle_detection::brent(self, |mut this| {
                this.north();
                this.west();
                this.south();
                this.east();
                this
            });
        // dbg!(len, elem, idx);

        let goal = 1000000000;
        let remaining = (goal - idx) % len;
        for _ in 0..remaining {
            this.north();
            this.west();
            this.south();
            this.east();
        }

        let mut sum = 0;
        for row in 0..this.height {
            let mult = this.height - row;

            let row_offset = row * this.width;
            for col in 0..this.width - 1 {
                let col_offset = row_offset + col;
                if this.rocks[col_offset] == Rock::Round {
                    sum += mult;
                }
            }
        }

        sum
    }
}

// pub fn run(input: &str) -> impl std::fmt::Display {
//     Solution::parse(input).unwrap().1.part_one()
//     Solution::parse(input).unwrap().1.part_two()
// }

impl aoc::Challenge for Solution {
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

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    #[test]
    fn parse() {
        let output = Solution::must_parse(INPUT);
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::must_parse(INPUT);
        assert_eq!(output.part_one().to_string(), "136");
    }

    #[test]
    fn part_two() {
        let output = Solution::must_parse(INPUT);
        assert_eq!(output.part_two().to_string(), "64");
    }
}
