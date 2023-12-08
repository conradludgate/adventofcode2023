use std::{fmt, time::Instant};

pub trait Parser: Sized + Challenge {
    fn parse(input: &'static str) -> nom::IResult<&'static str, Self>;
}

pub trait Challenge {
    const NAME: &'static str;

    fn part_one(self) -> impl fmt::Display;

    fn part_two(self) -> impl fmt::Display;
}

pub fn check<C: Parser + Clone>(input: &'static str) {
    let start = Instant::now();
    let challenge = C::parse(input).unwrap().1;
    let p1 = challenge.clone().part_one();
    let p2 = challenge.part_two();
    println!("took: {:?}", start.elapsed());

    println!("\tAnswer to part one: {p1}");
    println!("\tAnswer to part two: {p2}");
}
