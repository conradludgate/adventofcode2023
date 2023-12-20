use std::{collections::HashMap, hash::BuildHasherDefault};

use arrayvec::ArrayVec;

type WorkflowName = u32;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution {
    workflows: HashMap<WorkflowName, Rules, BuildHasherDefault<rustc_hash::FxHasher>>,
    parts: Vec<Part>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Parts {
    x: RangeInclusive,
    m: RangeInclusive,
    a: RangeInclusive,
    s: RangeInclusive,
}

impl Parts {
    fn len(self) -> u64 {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RangeInclusive {
    start: u32,
    end: u32,
}

impl From<std::ops::RangeInclusive<u32>> for RangeInclusive {
    fn from(value: std::ops::RangeInclusive<u32>) -> Self {
        RangeInclusive {
            start: *value.start(),
            end: *value.end(),
        }
    }
}

impl RangeInclusive {
    fn len(self) -> u64 {
        (self.end - self.start + 1) as u64
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Rules {
    rules: ArrayVec<Rule, 4>,
    fallback: Outcome,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Rule {
    xmas: u8, // can be one of b"xmas",
    op: u8,   // can be one of b"<>",
    val: u32,
    outcome: Outcome,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Outcome {
    Accept,
    Reject,
    Move(WorkflowName),
}

impl<'a> aoc::Parser<'a> for Solution {
    fn parse(mut input: &'a str) -> nom::IResult<&'a str, Self> {
        let mut workflows = HashMap::with_capacity_and_hasher(600, Default::default());
        while input.as_bytes()[0] != b'\n' {
            let workflow = match input.as_bytes()[0..4] {
                [a, b, c, b'{'] => {
                    input = unsafe { input.get_unchecked(4..) };
                    u32::from_ne_bytes([a, b, c, 0])
                }
                [a, b, b'{', _] => {
                    input = unsafe { input.get_unchecked(3..) };
                    u32::from_ne_bytes([a, b, 0, 0])
                }
                _ => unreachable!(),
            };
            let rules;
            (input, rules) = Rules::parse(input)?;
            workflows.insert(workflow, rules);
        }

        let mut parts = Vec::with_capacity(100);
        // skip newline separator
        input = &input[1..];
        while !input.is_empty() {
            let part;
            (input, part) = Part::parse(input)?;
            parts.push(part);
        }

        Ok(("", Self { workflows, parts }))
    }
}

impl<'a> Rules {
    fn parse(mut input: &'a str) -> nom::IResult<&'a str, Self> {
        // s<537:gd,x>2440:R,A}

        let mut rules = ArrayVec::new();
        while input.as_bytes()[1] == b'<' || input.as_bytes()[1] == b'>' {
            let xmas = input.as_bytes()[0];
            let op = input.as_bytes()[1];
            input = &input[2..];
            let mut val = 0;
            loop {
                match input.as_bytes()[0] {
                    b':' => break,
                    v => val = val * 10 + (v & 0xf) as u32,
                }
                input = &input[1..];
            }
            input = &input[1..];
            let outcome = match input.as_bytes()[0] {
                b'A' => {
                    input = unsafe { input.get_unchecked(2..) };
                    Outcome::Accept
                }
                b'R' => {
                    input = unsafe { input.get_unchecked(2..) };
                    Outcome::Reject
                }
                _ => Outcome::Move(match input.as_bytes()[0..4] {
                    [a, b, c, b','] => {
                        input = unsafe { input.get_unchecked(4..) };
                        u32::from_ne_bytes([a, b, c, 0])
                    }
                    [a, b, b',', _] => {
                        input = unsafe { input.get_unchecked(3..) };
                        u32::from_ne_bytes([a, b, 0, 0])
                    }
                    _ => unreachable!(),
                }),
            };

            rules.push(Rule {
                xmas,
                op,
                val,
                outcome,
            })
        }

        let fallback = match input.as_bytes()[0] {
            b'A' => {
                input = unsafe { input.get_unchecked(3..) };
                Outcome::Accept
            }
            b'R' => {
                input = unsafe { input.get_unchecked(3..) };
                Outcome::Reject
            }
            _ => Outcome::Move(match input.as_bytes()[0..4] {
                [a, b, c, b'}'] => {
                    input = unsafe { input.get_unchecked(5..) };
                    u32::from_ne_bytes([a, b, c, 0])
                }
                [a, b, b'}', _] => {
                    input = unsafe { input.get_unchecked(4..) };
                    u32::from_ne_bytes([a, b, 0, 0])
                }
                _ => unreachable!(),
            }),
        };

        Ok((input, Self { rules, fallback }))
    }
}

impl<'a> Part {
    fn parse(mut input: &'a str) -> nom::IResult<&'a str, Self> {
        // let [x, m, a, s, input] = split_many(input, ["{x=", ",m=", ",a=", ",s=", "}\n"]).unwrap();
        //{x=787,m=2655,a=1222,s=2876}

        input = &input[3..];
        let mut x = 0;
        loop {
            match input.as_bytes()[0] {
                b',' => break,
                v => x = x * 10 + (v & 0xf) as u32,
            }
            input = &input[1..];
        }
        input = &input[3..];
        let mut m = 0;
        loop {
            match input.as_bytes()[0] {
                b',' => break,
                v => m = m * 10 + (v & 0xf) as u32,
            }
            input = &input[1..];
        }
        input = &input[3..];
        let mut a = 0;
        loop {
            match input.as_bytes()[0] {
                b',' => break,
                v => a = a * 10 + (v & 0xf) as u32,
            }
            input = &input[1..];
        }
        input = &input[3..];
        let mut s = 0;
        loop {
            match input.as_bytes()[0] {
                b'}' => break,
                v => s = s * 10 + (v & 0xf) as u32,
            }
            input = &input[1..];
        }
        input = &input[2..];

        Ok((input, Self { x, m, a, s }))
    }
}

impl Rule {
    fn apply(self, part: Part) -> Option<Outcome> {
        let xmas = match self.xmas {
            b'x' => part.x,
            b'm' => part.m,
            b'a' => part.a,
            b's' => part.s,
            _ => unreachable!(),
        };
        let cond = match self.op {
            b'>' => xmas > self.val,
            _ => xmas < self.val,
        };
        cond.then_some(self.outcome)
    }

    fn apply_many(self, parts: Parts) -> (Outcome, Parts, Parts) {
        let xmas = match self.xmas {
            b'x' => parts.x,
            b'm' => parts.m,
            b'a' => parts.a,
            b's' => parts.s,
            _ => unreachable!(),
        };

        let (accept, cont) = match self.op {
            b'>' => {
                // 1..=4000 > 1716
                // accept: 1717..=4000
                // continue: 1..=1716

                // 2000..=4000 > 1716
                // accept: 2000..=4000
                // continue: 2000..=1716

                // 1..=1500 > 1716
                // accept: 1717..=1500
                // continue: 1..=1500

                let accept = u32::max(xmas.start, self.val + 1)..=xmas.end;
                let cont = xmas.start..=u32::min(xmas.end, self.val);

                (accept, cont)
            }
            _ => {
                // 1..=4000 < 1716
                // accept: 1..=1715
                // continue: 1716..=4000

                // 2000..=4000 < 1716
                // accept: 2000..=1715
                // continue: 2000..=4000

                // 1..=1500 < 1716
                // accept: 1..=1500
                // continue: 1716..=1500

                let accept = xmas.start..=u32::min(xmas.end, self.val - 1);
                let cont = u32::max(xmas.start, self.val)..=xmas.end;

                (accept, cont)
            }
        };

        let mut accept_part = parts;
        let mut cont_part = parts;

        match self.xmas {
            b'x' => {
                accept_part.x = accept.into();
                cont_part.x = cont.into();
            }
            b'm' => {
                accept_part.m = accept.into();
                cont_part.m = cont.into();
            }
            b'a' => {
                accept_part.a = accept.into();
                cont_part.a = cont.into();
            }
            b's' => {
                accept_part.s = accept.into();
                cont_part.s = cont.into();
            }
            _ => unreachable!(),
        };

        (self.outcome, accept_part, cont_part)
    }
}

impl Rules {
    fn apply(&self, part: Part) -> Outcome {
        self.rules
            .iter()
            .filter_map(|rule| rule.apply(part))
            .next()
            .unwrap_or(self.fallback)
    }
}
impl Part {
    fn score(self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

impl Solution {
    fn part_one(self) -> impl std::fmt::Display {
        let in_workflow = u32::from_ne_bytes(*b"in\0\0");

        let mut sum = 0;
        for part in self.parts {
            let mut workflow = in_workflow;
            loop {
                match self.workflows[&workflow].apply(part) {
                    Outcome::Accept => {
                        sum += part.score();
                        break;
                    }
                    Outcome::Reject => break,
                    Outcome::Move(w) => workflow = w,
                }
            }
        }
        sum
    }

    fn part_two(self) -> impl std::fmt::Display {
        let mut dfs = Vec::new();

        dfs.push((
            u32::from_ne_bytes(*b"in\0\0"),
            Parts {
                x: (1..=4000).into(),
                m: (1..=4000).into(),
                a: (1..=4000).into(),
                s: (1..=4000).into(),
            },
        ));

        let mut sum = 0;
        while let Some((workflow, mut range)) = dfs.pop() {
            let workflow = &self.workflows[&workflow];
            for rule in &workflow.rules {
                let (outcome, accept, cont) = rule.apply_many(range);
                match outcome {
                    Outcome::Accept => sum += accept.len(),
                    Outcome::Move(w) if accept.len() > 0 => {
                        dfs.push((w, accept));
                    }
                    _ => {}
                }
                range = cont;
            }
            match workflow.fallback {
                Outcome::Accept => sum += range.len(),
                Outcome::Move(w) if range.len() > 0 => {
                    dfs.push((w, range));
                }
                _ => {}
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

    const INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

    #[test]
    fn parse() {
        let output = Solution::must_parse(INPUT);
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::must_parse(INPUT);
        assert_eq!(output.part_one().to_string(), "19114");
    }

    #[test]
    fn part_two() {
        let output = Solution::must_parse(INPUT);
        assert_eq!(output.part_two().to_string(), "167409079868000");
    }
}
