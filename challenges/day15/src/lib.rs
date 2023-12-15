use arrayvec::ArrayVec;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution<'a>(&'a str);

impl<'a> aoc::Parser<'a> for Solution<'a> {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        Ok(("", Self(input.trim_end())))
    }
}

// #[inline(never)]
fn hash(b: &[u8]) -> u32 {
    let mut hash = 0u32;
    let mut chunks = b.rchunks_exact(2);
    for chunk in &mut chunks {
        let &[a, b] = chunk else { panic!("not two") };

        hash = hash.wrapping_mul(33);
        let a = (a as u32).wrapping_mul(33);
        let b = (b as u32).wrapping_mul(17);
        hash = hash.wrapping_add(a).wrapping_add(b);
    }
    if let &[a] = chunks.remainder() {
        hash = hash.wrapping_add(a as u32).wrapping_mul(17);
    }
    hash & 0xff
}

// fn hash(b: &str) -> u32 {
//     let mut hash = 0u32;
//     let (h, m, t) = unsafe { b.as_bytes().align_to::<[u8; 2]>() };

//     for &b in h {
//         hash = hash.wrapping_add(b as u32).wrapping_mul(17);
//     }
//     for &[a, b] in m {
//         hash = hash
//             .wrapping_mul(33)
//             .wrapping_add((a as u32).wrapping_mul(33))
//             .wrapping_add((b as u32).wrapping_mul(17));
//     }
//     // for &[a, b, c, d] in m {
//     //     // x = x * 17 * 17 * 17 * 17 + a * 17 * 17 * 17 * 17 + b * 17 * 17 * 17 + c * 17 * 17 + d * 17;

//     //     // 65
//     //     // 49
//     //     // 33
//     //     // 17
//     //     hash = hash
//     //         .wrapping_mul(65)
//     //         .wrapping_add((a as u32).wrapping_mul(65))
//     //         .wrapping_add((b as u32).wrapping_mul(49))
//     //         .wrapping_add((c as u32).wrapping_mul(33))
//     //         .wrapping_add((d as u32).wrapping_mul(17));

//     //     // hash = hash.wrapping_add(b[0] as u32).wrapping_mul(17);
//     //     // hash = hash.wrapping_add(b[1] as u32).wrapping_mul(17*17);
//     //     // hash = hash.wrapping_add(b[2] as u32).wrapping_mul(17*17*17);
//     //     // hash = hash.wrapping_add(b[3] as u32).wrapping_mul(17*17*17*17);
//     // }
//     for &b in t {
//         hash = hash.wrapping_add(b as u32).wrapping_mul(17);
//     }
//     hash & 0xff
// }

impl Solution<'_> {
    fn part_one(self) -> impl std::fmt::Display {
        let mut sum = 0;
        let mut iter = memchr::memchr_iter(b',', self.0.as_bytes());
        let mut i = iter.next().unwrap();
        for j in iter {
            sum += hash(&self.0.as_bytes()[i..j]);
            i = j;
        }
        sum
        // self.0.split(',').map(hash).sum::<u32>()
    }

    fn part_two(self) -> impl std::fmt::Display {
        const BOX: ArrayVec<(&[u8], u8), 8> = ArrayVec::<(&[u8], u8), 8>::new_const();
        let mut boxes = [BOX; 256];
        let mut iter = memchr::memchr_iter(b',', self.0.as_bytes());
        let mut i = iter.next().unwrap();
        for j in iter {
            let step = &self.0.as_bytes()[i..j];
            i = j;

            if let Some(label) = step.strip_suffix(b"-") {
                let id = hash(label);
                if let Some(i) = boxes[id as usize].iter().position(|&(l, _)| l == label) {
                    boxes[id as usize].remove(i);
                }
            } else {
                let (label, lens) = step.split_at(step.len() - 2);
                let &[_, lens] = lens else { panic!() };
                let id = hash(label);
                if let Some(i) = boxes[id as usize].iter().position(|&(l, _)| l == label) {
                    boxes[id as usize][i].1 = lens;
                } else {
                    boxes[id as usize].push((label, lens))
                }
            }
        }
        let mut sum = 0;
        for (i, b) in boxes.iter().enumerate() {
            for (j, &(_, f)) in b.iter().enumerate() {
                sum += (i + 1) * (j + 1) * (f & 0xf) as usize;
            }
        }
        sum
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
        assert_eq!(output.part_two().to_string(), "145");
    }
}
