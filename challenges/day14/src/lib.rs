#[derive(Debug, PartialEq, Clone)]
pub struct Solution<'a> {
    rocks: &'a [Rock],
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

impl<'a> aoc::Parser<'a> for Solution<'a> {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        let width = input.find('\n').unwrap() + 1;
        let height = input.len() / width;
        let rocks = unsafe { std::mem::transmute::<&[u8], &[Rock]>(input.as_bytes()) };

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

impl Solution<'_> {
    fn part_one(self) -> impl std::fmt::Display {
        let mut vec = self.rocks.to_vec();

        let mut sum = 0;
        for row in 0..self.height {
            let mult = self.height - row;

            let row_offset = row * self.width;
            for col in 0..self.width - 1 {
                let col_offset = row_offset + col;
                let mut extra = 0;
                loop {
                    match vec.get_mut(col_offset + extra) {
                        Some(Rock::Empty) => extra += self.width,
                        Some(r @ Rock::Round) => {
                            *r = Rock::Empty;
                            vec[col_offset] = Rock::Round;
                            sum += mult;
                            break;
                        }
                        _ => break,
                    }
                }
            }
        }

        // for line in vec.chunks_exact(self.width) {
        //     for x in line {
        //         print!("{}", *x as u8 as char);
        //     }
        // }

        sum
    }

    fn part_two(self) -> impl std::fmt::Display {
        0
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
        assert_eq!(output.part_two().to_string(), "0");
    }
}
