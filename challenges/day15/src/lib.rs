#[derive(Debug, PartialEq, Clone)]
pub struct Solution<'a>(&'a str);

impl<'a> aoc::Parser<'a> for Solution<'a> {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        Ok(("", Self(input.trim_end())))
    }
}

fn hash(b: &[u8]) -> u32 {
    let mut hash = 0u32;
    for &b in b {
        hash = hash.wrapping_add(b as u32).wrapping_mul(17);
    }
    hash & 0xff
}

impl Solution<'_> {
    fn part_one(self) -> impl std::fmt::Display {
        self.0.split(',').map(|b| hash(b.as_bytes())).sum::<u32>()
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
    use crate::hash;

    use super::Solution;
    use aoc::Parser;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";

    #[test]
    fn parse() {
        let output = Solution::must_parse(INPUT);
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::must_parse(INPUT);
        assert_eq!(output.part_one().to_string(), "1320");
    }

    #[test]
    fn part_two() {
        let output = Solution::must_parse(INPUT);
        assert_eq!(output.part_two().to_string(), "0");
    }
}
