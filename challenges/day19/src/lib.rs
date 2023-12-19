use arrayvec::ArrayVec;
use parsers::split_many;
use rustc_hash::FxHashMap;

type WorkflowName = [u8; 4];

#[derive(Debug, PartialEq, Clone)]
pub struct Solution {
    workflows: FxHashMap<WorkflowName, Rules>,
    parts: Vec<Part>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
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
        let mut workflows = FxHashMap::default();
        while input.as_bytes()[0] != b'\n' {
            let workflow = match input.as_bytes()[0..4] {
                [a, b, c, b'{'] => {
                    input = unsafe { input.get_unchecked(4..) };
                    [a, b, c, 0]
                }
                [a, b, b'{', _] => {
                    input = unsafe { input.get_unchecked(3..) };
                    [a, b, 0, 0]
                }
                _ => unreachable!(),
            };
            let rules;
            (input, rules) = Rules::parse(input)?;
            workflows.insert(workflow, rules);
        }

        let mut parts = Vec::new();
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
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        let (mut input, rest) = input.split_once("}\n").unwrap();

        let mut rules = ArrayVec::new();
        while let Some((rule, i)) = input.split_once(',') {
            input = i;
            let (rule, outcome) = rule.split_once(':').unwrap();
            let xmas = rule.as_bytes()[0];
            let op = rule.as_bytes()[1];
            let val = rule[2..].parse().unwrap();

            let outcome = match outcome.as_bytes() {
                b"A" => Outcome::Accept,
                b"R" => Outcome::Reject,
                &[a, b, c] => Outcome::Move([a, b, c, 0]),
                &[a, b] => Outcome::Move([a, b, 0, 0]),
                _ => unreachable!(),
            };

            rules.push(Rule {
                xmas,
                op,
                val,
                outcome,
            })
        }

        let fallback = match input.as_bytes() {
            b"A" => Outcome::Accept,
            b"R" => Outcome::Reject,
            &[a, b, c] => Outcome::Move([a, b, c, 0]),
            &[a, b] => Outcome::Move([a, b, 0, 0]),
            _ => unreachable!(),
        };

        Ok((rest, Self { rules, fallback }))
    }
}

impl<'a> Part {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        let [x, m, a, s, input] = split_many(input, ["{x=", ",m=", ",a=", ",s=", "}\n"]).unwrap();

        Ok((
            input,
            Self {
                x: x.parse().unwrap(),
                m: m.parse().unwrap(),
                a: a.parse().unwrap(),
                s: s.parse().unwrap(),
            },
        ))
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
        let in_workflow = *b"in\0\0";

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
        0
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
        assert_eq!(output.part_two().to_string(), "0");
    }
}
