use std::{collections::BTreeMap, fmt::Display};

use aoc::{Challenge, Parser as ChallengeParser};
use nom::{
    bytes::complete::{tag, take_until},
    character::{
        self,
        complete::{line_ending, space1},
    },
    sequence::tuple,
    IResult, Parser,
};
use parsers::{number, ParserExt};

#[derive(Debug, PartialEq, Clone, Copy)]
struct MapRange {
    dst: u32,
    src: u32,
    len: u32,
}

impl MapRange {
    fn map(self, x: u32) -> Option<u32> {
        if let Some(diff) = x.checked_sub(self.src) {
            if diff < self.len {
                Some(self.dst + diff)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        number
            .separated_array(tag(" "))
            .map(|[dst, src, len]| Self { dst, src, len })
            .parse(input)
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
struct MapInner {
    set: BTreeMap<u32, MapRange>,
}

impl Extend<MapRange> for MapInner {
    fn extend<T: IntoIterator<Item = MapRange>>(&mut self, iter: T) {
        for i in iter {
            self.set.insert(i.src, i);
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Map {
    name: &'static str,
    inner: MapInner,
}

impl Map {
    fn map(&self, x: u32) -> u32 {
        // todo: fix this
        for (_, mr) in self.inner.set.range(..=x).rev() {
            if let Some(output) = mr.map(x) {
                return output;
            }
        }
        x
    }

    fn into_map(self) -> impl Fn(u32) -> u32 {
        move |x| self.map(x)
    }

    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let (input, name) = take_until(" map:\n")
            .followed_by(tag(" map:\n"))
            .parse(input)?;
        MapRange::parse
            .lines()
            .map(|inner| Self { name, inner })
            .parse(input)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution {
    seeds: Vec<u32>,
    maps: [Map; 7],
}

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let (input, seeds) = number::<u32>
            .separated_list1(tag(" "))
            .preceded_by(tag("seeds: "))
            .followed_by(tag("\n\n"))
            .parse(input)?;
        let (input, maps) = Map::parse.separated_array(line_ending).parse(input)?;

        Ok((input, Self { seeds, maps }))
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(self) -> impl Display {
        let [soil, fertilizer, water, light, temp, humitiy, location] = self.maps;

        self.seeds
            .into_iter()
            .map(soil.into_map())
            .map(fertilizer.into_map())
            .map(water.into_map())
            .map(light.into_map())
            .map(temp.into_map())
            .map(humitiy.into_map())
            .map(location.into_map())
            .min()
            .unwrap()
    }

    fn part_two(self) -> impl Display {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::MapRange;

    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn map_range() {
        let mr = MapRange {
            dst: 50,
            src: 98,
            len: 2,
        };
        assert_eq!(mr.map(97), None);
        assert_eq!(mr.map(98), Some(50));
        assert_eq!(mr.map(99), Some(51));
        assert_eq!(mr.map(100), None);
    }

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:#?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one().to_string(), "35");
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two().to_string(), "0");
    }
}
