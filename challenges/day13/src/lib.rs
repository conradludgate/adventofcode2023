use arrayvec::ArrayVec;

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    rows: ArrayVec<u32, 24>,
    cols: ArrayVec<u32, 24>,
}

impl<'a> Block {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        let mut input2 = input;

        let mut rows = ArrayVec::new();
        let mut cols = ArrayVec::new();
        for _ in 0..24 {
            cols.push(0);
        }

        let mut current_row = 0;
        let mut col = 0;
        let mut max_col = 0;

        for (i, b) in input2.bytes().enumerate() {
            if b == b'\n' {
                if col == 0 {
                    col = max_col;
                    input2 = &input2[i + 1..];
                    break;
                }
                max_col = col;
                col = 0;
                rows.push(current_row);
                current_row = 0;
            } else {
                current_row |= ((b & 1) as u32) << col;
                cols[col] |= ((b & 1) as u32) << rows.len();
                col += 1;
            }
        }
        if col == 0 {
            input2 = "";
        }
        cols.truncate(max_col);

        Ok((input2, Self { rows, cols }))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(Vec<Block>);

impl<'a> aoc::Parser<'a> for Solution {
    fn parse(mut input: &'a str) -> nom::IResult<&'a str, Self> {
        let mut out = Vec::with_capacity(100);
        while !input.is_empty() {
            let (i, x) = Block::parse(input)?;
            out.push(x);
            input = i;
        }

        Ok((input, Self(out)))
    }
}

impl Block {
    fn solve<const D: u32>(self) -> usize {
        // rows
        let mut sum = 0;
        'outer: for i in 0..self.rows.len() - 1 {
            let mut j = i;
            let mut k = j + 1;

            let mut diffs = 0;
            diffs += (self.rows[j] ^ self.rows[k]).count_ones();

            if diffs > D {
                continue 'outer;
            }

            while j > 0 && k + 1 < self.rows.len() {
                j -= 1;
                k += 1;

                diffs += (self.rows[j] ^ self.rows[k]).count_ones();

                if diffs > 1 {
                    continue 'outer;
                }
            }

            sum = 100 * (i + 1);
            if diffs == D {
                return sum;
            }
        }

        // cols
        'outer: for i in 0..self.cols.len() - 1 {
            let mut j = i;
            let mut k = j + 1;

            let mut diffs = 0;
            diffs += (self.cols[j] ^ self.cols[k]).count_ones();
            if diffs > D {
                continue 'outer;
            }

            while j > 0 && k + 1 < self.cols.len() {
                j -= 1;
                k += 1;

                diffs += (self.cols[j] ^ self.cols[k]).count_ones();
                if diffs > 1 {
                    continue 'outer;
                }
            }

            sum = i + 1;
            if diffs == D {
                return sum;
            }
        }

        sum
    }
}

impl Solution {
    fn part_one(self) -> impl std::fmt::Display {
        self.0.into_iter().map(Block::solve::<0>).sum::<usize>()
    }

    fn part_two(self) -> impl std::fmt::Display {
        self.0.into_iter().map(Block::solve::<1>).sum::<usize>()
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
        assert_eq!(output.part_two().to_string(), "400");
    }
}
