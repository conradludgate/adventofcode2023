#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Dir {
    R,
    D,
    L,
    U,
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Line {
    dir: Dir,
    dist: i64,
}

impl Line {
    fn apply(self, (x, y): (i64, i64)) -> (i64, i64) {
        let (x1, y1) = match self.dir {
            Dir::R => (self.dist, 0),
            Dir::L => (-(self.dist), 0),
            Dir::D => (0, -(self.dist)),
            Dir::U => (0, self.dist),
        };
        (x + x1, y + y1)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(Vec<(Line, Line)>);

fn hex(x: u8) -> i64 {
    match x {
        b'0' => 0,
        b'1' => 1,
        b'2' => 2,
        b'3' => 3,
        b'4' => 4,
        b'5' => 5,
        b'6' => 6,
        b'7' => 7,
        b'8' => 8,
        b'9' => 9,
        b'a' => 10,
        b'b' => 11,
        b'c' => 12,
        b'd' => 13,
        b'e' => 14,
        b'f' => 15,
        _ => unimplemented!(),
    }
}

impl<'a> aoc::Parser<'a> for Solution {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        let mut input = input.as_bytes();
        let mut output = Vec::with_capacity(1024);
        while !input.is_empty() {
            let dir = match input[0] {
                b'R' => Dir::R,
                b'L' => Dir::L,
                b'U' => Dir::U,
                b'D' => Dir::D,
                _ => unimplemented!(),
            };
            let len = if input[13] == b'\n' { 14 } else { 15 };
            let dist = if len == 14 {
                (input[2] & 0xf) as i64
            } else {
                ((input[2] & 0xf) * 10 + (input[3] & 0xf)) as i64
            };
            let line1 = Line { dir, dist };

            let &[a, b, c, d, e, f] = &input[len - 8..len - 2] else {
                panic!()
            };
            let dist = hex(a) << 16 | hex(b) << 12 | hex(c) << 8 | hex(d) << 4 | hex(e);
            let dir = match f {
                b'0' => Dir::R,
                b'1' => Dir::D,
                b'2' => Dir::L,
                b'3' => Dir::U,
                _ => unimplemented!(),
            };
            let line2 = Line { dir, dist };

            output.push((line1, line2));
            input = &input[len..];
        }

        Ok(("", Self(output)))
    }
}

fn solve(mut inst: impl Iterator<Item = Line>) -> i64 {
    // shoelace formula:
    // 2*area = sum(y[i] * (x[i-1] - x[i+1]))
    // picks theorem:
    // 2*interior points = 2*area - exterior points - 2

    let first_inst = inst.next().unwrap();

    let start: (i64, i64) = (0, 0);
    let next = first_inst.apply(start);

    let mut b = first_inst.dist;
    let mut area = 0;

    let mut x2 = start.0;
    let mut p1 = next;
    for i in inst {
        let (x0, y0) = i.apply(p1);
        let (x1, y1) = p1;

        area += y1 * (x2 - x0);
        b += i.dist;

        x2 = x1;
        p1 = (x0, y0);
    }

    debug_assert_eq!(p1, start);
    let (x0, _) = next;
    let (_, y1) = p1;
    area += y1 * (x2 - x0);

    (area.abs() - b + 2) / 2 + b
}

impl Solution {
    fn part_one(self) -> impl std::fmt::Display {
        solve(self.0.into_iter().map(|l| l.0))
    }

    fn part_two(self) -> impl std::fmt::Display {
        solve(self.0.into_iter().map(|l| l.1))
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

    const INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

    #[test]
    fn parse() {
        let output = Solution::must_parse(INPUT);
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::must_parse(INPUT);
        assert_eq!(output.part_one().to_string(), "62");
    }

    #[test]
    fn part_two() {
        let output = Solution::must_parse(INPUT);
        assert_eq!(output.part_two().to_string(), "952408144115");
    }
}
