use std::{fmt, time::Instant};

pub trait Parser<'a>: Sized + Challenge {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self>;

    fn must_parse(input: &'a str) -> Self {
        Self::parse(input).unwrap().1
    }
}

pub trait Challenge {
    fn part_one(self) -> impl fmt::Display;

    fn part_two(self) -> impl fmt::Display;
}

pub fn check<'a, C: Parser<'a> + Clone>(input: &'a str) {
    let start = Instant::now();
    let challenge = C::must_parse(input);
    let p1 = challenge.clone().part_one();
    let p2 = challenge.part_two();
    println!("took: {:?}", start.elapsed());

    println!("\tAnswer to part one: {p1}");
    println!("\tAnswer to part two: {p2}");
}
