#![feature(array_chunks)]

use std::{collections::BTreeMap, fmt::Display, ops::Range};

use aoc::{Challenge, Parser as ChallengeParser};
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::line_ending,
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
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        number
            .separated_array(tag(" "))
            .map(|[dst, src, len]| Self { dst, src, len })
            .parse(input)
    }
}

#[derive(Debug, PartialEq, Clone)]
struct MapInner {
    set: BTreeMap<u32, u32>,
}

impl Default for MapInner {
    fn default() -> Self {
        let mut set = BTreeMap::new();
        set.insert(0, 0);
        Self { set }
    }
}

impl Extend<MapRange> for MapInner {
    fn extend<T: IntoIterator<Item = MapRange>>(&mut self, iter: T) {
        for i in iter {
            let end = i.src.saturating_add(i.len);
            self.set.entry(end).or_insert(end);
            self.set.insert(i.src, i.dst);
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
        let (src, dst) = self.inner.set.range(..=x).next_back().unwrap();
        let diff = x - src;
        dst + diff
    }

    fn map_ranges(&self, input: &mut Vec<Range<u32>>, output: &mut Vec<Range<u32>>) {
        for mut r in input.drain(..) {
            while !r.is_empty() {
                let (&src1, &dst1) = self.inner.set.range(..=r.start).next_back().unwrap();
                let output2 = self.inner.set.range(r.start + 1..r.end).next();

                let diff = dst1.wrapping_sub(src1);
                let start = r.start.wrapping_add(diff);

                let end = match output2 {
                    None => r.end,
                    Some((&src2, _)) => src2,
                };

                r.start = end;
                let end = end.wrapping_add(diff);
                output.push(start..end);
            }
        }
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
        let [soil, fertilizer, water, light, temp, humitiy, location] = self.maps;

        let mut ranges1 = self
            .seeds
            .array_chunks()
            .map(|&[start, len]| start..start + len)
            .collect::<Vec<_>>();
        let mut ranges2 = Vec::with_capacity(ranges1.len());

        soil.map_ranges(&mut ranges1, &mut ranges2);
        fertilizer.map_ranges(&mut ranges2, &mut ranges1);
        water.map_ranges(&mut ranges1, &mut ranges2);
        light.map_ranges(&mut ranges2, &mut ranges1);
        temp.map_ranges(&mut ranges1, &mut ranges2);
        humitiy.map_ranges(&mut ranges2, &mut ranges1);
        location.map_ranges(&mut ranges1, &mut ranges2);

        ranges2.into_iter().map(|r| r.start).min().unwrap()
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(output.part_two().to_string(), "46");
    }
}
