use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution<'a> {
    width: usize,
    height: usize,
    data: &'a [Foo],
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(dead_code)]
enum Foo {
    Empty = b'.',
    Galaxy = b'#',
    LineEnd = b'\n',
}

impl Solution<'static> {
    pub fn parse(input: &'static str) -> Self {
        let data = unsafe { std::mem::transmute::<&[u8], &[Foo]>(input.as_bytes()) };
        let (width, height) = if 140 * 141 == input.len() {
            (141, 140)
        } else {
            (11, 10)
        };
        Self {
            data,
            width,
            height,
        }
    }
}

impl Solution<'_> {
    fn inner<const N: usize>(self) -> impl fmt::Display {
        let mut cols = vec![0u8; self.width];
        let mut sum = 0;
        let mut last_y = 0;
        let mut last_sum = 0;
        let mut galaxies = 0;
        for (y, line) in self.data.chunks_exact(self.width).enumerate() {
            for (x, t) in line.iter().enumerate() {
                match t {
                    Foo::Galaxy => {
                        let mut diff = y - last_y;
                        if diff > 1 {
                            diff = 1 + (diff - 1) * N;
                        }
                        last_sum += diff * galaxies;
                        sum += last_sum;
                        galaxies += 1;
                        last_y = y;
                        cols[x] += 1;
                    }
                    Foo::Empty | Foo::LineEnd => {}
                }
            }
        }

        let mut last_x = 0;
        let mut last_sum = 0;
        let mut galaxies = 0;
        for (x, mut count) in cols.drain(..).enumerate() {
            while count > 0 {
                let mut diff = x - last_x;
                if diff > 1 {
                    diff = 1 + (diff - 1) * N;
                }
                last_sum += diff * galaxies;
                sum += last_sum;
                galaxies += 1;
                last_x = x;

                count -= 1;
            }
        }

        sum
    }
}

impl Solution<'_> {
    #[inline(never)]
    pub fn part_one(self) -> impl fmt::Display {
        self.inner::<2>()
    }

    #[inline(never)]
    pub fn part_two(self) -> impl fmt::Display {
        self.inner::<1000000>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    // s0 = 1 + 2 + 5 + 6 + 7 + 10 + 11 + 11 = (s1 + 8 * 1)
    // s1 =     1 + 4 + 5 + 6 +  9 + 10 + 10 = (s2 + 7 * 1)
    // s2 =         3 + 4 + 5 +  8 +  9 +  9 = (s3 + 6 * 3)
    // s3 =             1 + 2 +  5 +  6 +  6 = (s4 + 5 * 1)
    // s4 =                 1 +  4 +  5 +  5 = (s5 + 4 * 1)
    // s5 =                      3 +  4 +  4 = (s6 + 3 * 3)
    // s6 =                           1 +  1 = (s7 + 2 * 1)
    // s7 =                                0 = 0

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT);
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT);
        assert_eq!(output.part_one().to_string(), "374");
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT);
        assert_eq!(output.inner::<10>().to_string(), "1030");
        let output = Solution::parse(INPUT);
        assert_eq!(output.inner::<100>().to_string(), "8410");
    }
}
