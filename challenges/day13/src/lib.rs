use itertools::Itertools;

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy, Eq)]
#[allow(dead_code)]
enum Node {
    Ash = b'.',
    Rock = b'#',
    LineEnding = b'\n',
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block<'a> {
    width: usize,
    height: usize,
    data: &'a [Node],
}

impl<'a> Block<'a> {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        let width = input.find('\n').unwrap() + 1;
        let (data, input) = match input.find("\n\n") {
            Some(x) => (&input.as_bytes()[..x + 1], &input[x + 2..]),
            None => (input.as_bytes(), ""),
        };
        let height = data.len() / width;
        Ok((
            input,
            Self {
                width,
                height,
                data: unsafe { std::mem::transmute(data) },
            },
        ))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution<'a>(Vec<Block<'a>>);

impl<'a> aoc::Parser<'a> for Solution<'a> {
    fn parse(mut input: &'a str) -> nom::IResult<&'a str, Self> {
        let mut out = Vec::with_capacity(50);
        while !input.is_empty() {
            let (i, x) = Block::parse(input)?;
            out.push(x);
            input = i;
        }

        Ok((input, Self(out)))
    }
}

impl Block<'_> {
    fn part_one(self) -> usize {
        // rows
        let mut sum = 0;
        'outer: for (i, (rowa, rowb)) in self
            .data
            .chunks_exact(self.width)
            .tuple_windows()
            .enumerate()
        {
            if rowa == rowb {
                let mut j = i;
                let mut k = j + 1;

                while j > 0 && k + 1 < self.height {
                    j -= 1;
                    k += 1;

                    let rowj = &self.data[j * self.width..j * self.width + self.width];
                    let rowk = &self.data[k * self.width..k * self.width + self.width];
                    if rowj != rowk {
                        continue 'outer;
                    }
                }

                sum += 100 * (i + 1)
            }
        }

        // cols
        'outer: for i in 0..self.width - 2 {
            for h in 0..self.height {
                if self.data[h * self.width + i] != self.data[h * self.width + i + 1] {
                    continue 'outer;
                }
            }

            let mut j = i;
            let mut k = j + 1;

            while j > 0 && k + 2 < self.width {
                j -= 1;
                k += 1;

                for h in 0..self.height {
                    if self.data[h * self.width + j] != self.data[h * self.width + k] {
                        continue 'outer;
                    }
                }
            }

            sum += i + 1
        }

        sum
    }
}

impl Solution<'_> {
    fn part_one(self) -> impl std::fmt::Display {
        self.0.into_iter().map(Block::part_one).sum::<usize>()
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

    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    #[test]
    fn parse() {
        let output = Solution::must_parse(INPUT);
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::must_parse(INPUT);
        assert_eq!(output.part_one().to_string(), "405");
    }

    #[test]
    fn part_two() {
        let output = Solution::must_parse(INPUT);
        assert_eq!(output.part_two().to_string(), "0");
    }
}
