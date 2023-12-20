use std::collections::VecDeque;

use arrayvec::ArrayVec;
use petgraph::{
    dot::Dot,
    graph::NodeIndex,
    stable_graph::GraphIndex,
    visit::{self, EdgeRef},
    Graph,
};
use rustc_hash::FxHashMap;

#[derive(Debug, PartialEq, Clone)]
enum Type {
    /// on low pulse -> flip and send current value
    /// on high pule -> do nothing
    FlipFlop(ArrayVec<u16, 8>, bool),

    /// on low pulse -> set low and send high
    /// on high pulse -> set high and send low
    Conjunction(FxHashMap<u16, bool>),
}

#[derive(Debug, PartialEq, Clone)]
enum Type2 {
    /// on low pulse -> flip and send current value
    /// on high pule -> do nothing
    FlipFlop(u8),

    /// on low pulse -> set low and send high
    /// on high pulse -> set high and send low
    Conjunction,

    Button,
    End,
}

#[derive(Debug, PartialEq, Clone)]
struct Module {
    typ: Type,
    members: ArrayVec<u16, 8>,
}

#[derive(Debug, Clone)]
pub struct Solution {
    broadcaster: ArrayVec<u16, 8>,
    map: FxHashMap<u16, Module>,

    broadcaster1: NodeIndex,
    map1: FxHashMap<u16, NodeIndex>,
    graph: Graph<Type2, u8>,
}

fn parse_members(mut input: &str) -> nom::IResult<&str, ArrayVec<u16, 8>> {
    let mut members = ArrayVec::new();
    loop {
        let id = u16::from_ne_bytes([input.as_bytes()[0], input.as_bytes()[1]]);
        members.push(id);
        match input.as_bytes()[2] {
            b'\n' => {
                input = &input[3..];
                break;
            }
            _ => {
                input = &input[4..];
            }
        }
    }
    Ok((input, members))
}

impl<'a> aoc::Parser<'a> for Solution {
    fn parse(mut input: &'a str) -> nom::IResult<&'a str, Self> {
        let mut broadcaster = ArrayVec::new();
        let mut broadcaster1 = NodeIndex::new(0);
        let mut map = FxHashMap::with_capacity_and_hasher(60, Default::default());
        let mut map1 = FxHashMap::with_capacity_and_hasher(60, Default::default());
        let mut graph = Graph::new();

        while !input.is_empty() {
            let (typ, typ2) = match input.as_bytes()[0] {
                b'%' => (Type::FlipFlop(ArrayVec::new(), false), Type2::FlipFlop(0)),
                b'&' => (Type::Conjunction(FxHashMap::default()), Type2::Conjunction),
                _ => {
                    // broadcaster
                    (input, broadcaster) = parse_members(&input[15..])?;

                    broadcaster1 = graph.add_node(Type2::Button);
                    for &b in &broadcaster {
                        let node = map1.entry(b).or_insert_with(|| graph.add_node(Type2::End));
                        graph.add_edge(broadcaster1, *node, 255u8);
                    }

                    continue;
                }
            };
            let id = u16::from_ne_bytes([input.as_bytes()[1], input.as_bytes()[2]]);
            let members;
            (input, members) = parse_members(&input[7..])?;

            let node = *map1.entry(id).or_insert_with(|| graph.add_node(Type2::End));
            *graph.node_weight_mut(node).unwrap() = typ2;
            for &b in &members {
                let node2 = map1.entry(b).or_insert_with(|| graph.add_node(Type2::End));
                graph.add_edge(node, *node2, 255u8);
            }

            map.insert(id, Module { typ, members });
        }

        for &member in &broadcaster {
            let Some(this) = map.get_mut(&member) else {
                continue;
            };
            if let Type::Conjunction(m) = &mut this.typ {
                m.insert(0, false);
            }
        }
        let keys: Vec<u16> = map.keys().copied().collect();
        for id in keys {
            let members = map[&id].members.clone();
            for member in members {
                let Some(this) = map.get_mut(&member) else {
                    continue;
                };
                match &mut this.typ {
                    Type::FlipFlop(m, _) => m.push(id),
                    Type::Conjunction(m) => {
                        m.insert(id, false);
                    }
                }
            }
        }

        // println!("{:?}", Dot::new(&graph));

        Ok((
            "",
            Self {
                broadcaster,
                map,
                map1,
                broadcaster1,
                graph,
            },
        ))
    }
}

impl Solution {
    fn part_one(mut self) -> impl std::fmt::Display {
        let mut commands = VecDeque::new();
        let mut pulses = [0; 2];

        for _ in 0..1000 {
            // dbg!("push!");
            pulses[0] += 1;
            for &member in &self.broadcaster {
                commands.push_back((0, member, false));
            }

            while let Some((from, to, pulse)) = commands.pop_front() {
                // println!(
                //     "{}{} - recieved {}",
                //     to.to_ne_bytes()[0] as char,
                //     to.to_ne_bytes()[1] as char,
                //     pulse
                // );

                pulses[pulse as usize] += 1;

                let Some(this) = self.map.get_mut(&to) else {
                    continue;
                };
                match &mut this.typ {
                    Type::FlipFlop(_, state) if !pulse => {
                        *state = !*state;

                        for &member in &this.members {
                            commands.push_back((to, member, *state));
                        }
                    }
                    Type::Conjunction(states) => {
                        *states.get_mut(&from).unwrap() = pulse;
                        let send = !states.values().all(|x| *x);

                        for &member in &this.members {
                            commands.push_back((to, member, send));
                        }
                    }
                    _ => {}
                }
            }
        }

        pulses[0] * pulses[1]
    }

    /// finds when the node will emit the given value
    /// returns first instance, and then length of the cycle
    fn p2(mut self, goal: NodeIndex, state: bool) -> (u64, u64) {
        for edge in self
            .graph
            .edges_directed(goal, petgraph::Direction::Incoming)
        {
            // edge.source()
        }
        // let t = self.map.remove(&goal).unwrap();
        // match t.typ {
        //     Type::FlipFlop(p, current) => {
        //         for member in p {
        //             let m = self.map.get_mut(&member).unwrap();
        //             m.members.retain(|x| *x != goal);
        //         }
        //     }
        //     Type::Conjunction(p) => {
        //         for &member in p.keys() {
        //             let m = self.map.get_mut(&member).unwrap();
        //             m.members.retain(|x| *x != goal);
        //         }
        //     }
        // }
        (0, 0)
    }

    fn part_two(mut self) -> impl std::fmt::Display {
        let mut commands = VecDeque::new();
        let mut goals = FxHashMap::from_iter([
            (u16::from_ne_bytes(*b"mk"), None::<u64>),
            (u16::from_ne_bytes(*b"fp"), None),
            (u16::from_ne_bytes(*b"xt"), None),
            (u16::from_ne_bytes(*b"zc"), None),
        ]);
        let mut lens = vec![];

        for when in 0.. {
            // dbg!("push!");
            // pulses[0] += 1;
            for &member in &self.broadcaster {
                commands.push_back((0, member, false));
            }

            while let Some((from, to, pulse)) = commands.pop_front() {
                // println!(
                //     "{}{} - recieved {}",
                //     to.to_ne_bytes()[0] as char,
                //     to.to_ne_bytes()[1] as char,
                //     pulse
                // );
                if !pulse {
                    if let Some(x) = goals.get_mut(&to) {
                        if let Some(y) = x {
                            lens.push(when - *y);
                            goals.remove(&to);
                            if goals.len() == 0 {
                                return lens.iter().product::<u64>();
                            }
                        } else {
                            *x = Some(when);
                        }
                    }
                }

                // pulses[pulse as usize] += 1;

                let Some(this) = self.map.get_mut(&to) else {
                    continue;
                };
                match &mut this.typ {
                    Type::FlipFlop(_, state) if !pulse => {
                        *state = !*state;

                        for &member in &this.members {
                            commands.push_back((to, member, *state));
                        }
                    }
                    Type::Conjunction(states) => {
                        *states.get_mut(&from).unwrap() = pulse;
                        let send = !states.values().all(|x| *x);

                        for &member in &this.members {
                            commands.push_back((to, member, send));
                        }
                    }
                    _ => {}
                }
            }
        }
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
    use petgraph::dot::Dot;

    const INPUT1: &str = "broadcaster -> aa, bb, cc
%aa -> bb
%bb -> cc
%cc -> in
&in -> aa
";

    const INPUT2: &str = "broadcaster -> aa
%aa -> in, co
&in -> bb
%bb -> co
&co -> rx
";

    #[test]
    fn parse() {
        let output = Solution::must_parse(INPUT2);
        println!("{output:?}");
        println!("{:?}", Dot::new(&output.graph));
    }

    #[test]
    fn part_one() {
        let output = Solution::must_parse(INPUT1);
        assert_eq!(output.part_one().to_string(), "32000000");
        let output = Solution::must_parse(INPUT2);
        assert_eq!(output.part_one().to_string(), "11687500");
    }

    // #[test]
    // fn part_two() {
    //     let output = Solution::must_parse(INPUT2);
    //     assert_eq!(output.part_two().to_string(), "0");
    // }
}
