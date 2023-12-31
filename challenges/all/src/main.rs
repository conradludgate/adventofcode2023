use std::time::{Duration, Instant};

use aoc::Parser;

static DAY01: &str = include_str!("../../day01/input.txt");
static DAY02: &str = include_str!("../../day02/input.txt");
static DAY03: &str = include_str!("../../day03/input.txt");
static DAY04: &str = include_str!("../../day04/input.txt");
static DAY05: &str = include_str!("../../day05/input.txt");
static DAY06: &str = include_str!("../../day06/input.txt");
static DAY07: &str = include_str!("../../day07/input.txt");
static DAY08: &str = include_str!("../../day08/input.txt");
static DAY09: &str = include_str!("../../day09/input.txt");
static DAY10: &str = include_str!("../../day10/input.txt");
static DAY11: &str = include_str!("../../day11/input.txt");
static DAY12: &str = include_str!("../../day12/input.txt");
static DAY13: &str = include_str!("../../day13/input.txt");
static DAY14: &str = include_str!("../../day14/input.txt");
static DAY15: &str = include_str!("../../day15/input.txt");
static DAY16: &str = include_str!("../../day16/input.txt");
static DAY17: &str = include_str!("../../day17/input.txt");
static DAY18: &str = include_str!("../../day18/input.txt");
// static DAY19: &str = include_str!("../../day19/input.txt");
// static DAY20: &str = include_str!("../../day20/input.txt");
// static DAY21: &str = include_str!("../../day21/input.txt");
// static DAY22: &str = include_str!("../../day22/input.txt");
// static DAY23: &str = include_str!("../../day23/input.txt");
// static DAY24: &str = include_str!("../../day24/input.txt");
// static DAY25: &str = include_str!("../../day25/input.txt");

#[allow(unused_mut)]
fn main() {
    let start = Instant::now();
    let mut results = Vec::<Duration>::with_capacity(25);
    results.push(check::<day01::Solution>(DAY01));
    results.push(check::<day02::Solution>(DAY02));
    results.push(check::<day03::Solution>(DAY03));
    results.push(check::<day04::Solution>(DAY04));
    results.push(check::<day05::Solution>(DAY05));
    results.push(check::<day06::Solution>(DAY06));
    results.push(check::<day07::Solution>(DAY07));
    results.push(check::<day08::Solution>(DAY08));
    results.push(check::<day09::Solution>(DAY09));
    results.push(check::<day10::Solution>(DAY10));
    results.push(check::<day11::Solution>(DAY11));
    results.push(check::<day12::Solution>(DAY12));
    results.push(check::<day13::Solution>(DAY13));
    results.push(check::<day14::Solution>(DAY14));
    results.push(check::<day15::Solution>(DAY15));
    results.push(check::<day16::Solution>(DAY16));
    results.push(check::<day17::Solution>(DAY17));
    results.push(check::<day18::Solution>(DAY18));
    // results.push(check::<day19::Solution>(DAY19));
    // results.push(check::<day20::Solution>(DAY20));
    // results.push(check::<day21::Solution>(DAY21));
    // results.push(check::<day22::Solution>(DAY22));
    // results.push(check::<day23::Solution>(DAY23));
    // results.push(check::<day24::Solution>(DAY24));
    // results.push(check::<day25::Solution>(DAY25));

    let elapsed = start.elapsed();
    println!("Running {} days took {elapsed:?}", results.len());
    println!("{results:#?}");

    let start = Instant::now();
    let n = std::time::Duration::from_secs(5).as_nanos() / elapsed.as_nanos() * 2;
    for _ in 0..n {
        bench::<day01::Solution>(DAY01);
        bench::<day02::Solution>(DAY02);
        bench::<day03::Solution>(DAY03);
        bench::<day04::Solution>(DAY04);
        bench::<day05::Solution>(DAY05);
        bench::<day06::Solution>(DAY06);
        bench::<day07::Solution>(DAY07);
        bench::<day08::Solution>(DAY08);
        bench::<day09::Solution>(DAY09);
        bench::<day10::Solution>(DAY10);
        bench::<day11::Solution>(DAY11);
        bench::<day12::Solution>(DAY12);
        bench::<day13::Solution>(DAY13);
        bench::<day14::Solution>(DAY14);
        bench::<day15::Solution>(DAY15);
        bench::<day16::Solution>(DAY16);
        bench::<day17::Solution>(DAY17);
        bench::<day18::Solution>(DAY18);
        // bench::<day19::Solution>(DAY19);
        // bench::<day20::Solution>(DAY20);
        // bench::<day21::Solution>(DAY21);
        // bench::<day22::Solution>(DAY22);
        // bench::<day23::Solution>(DAY23);
        // bench::<day24::Solution>(DAY24);
        // bench::<day25::Solution>(DAY25);
    }
    let elapsed = start.elapsed();

    println!("Running {} days {n} times took {elapsed:?}", results.len());
    println!("Average {:?}", elapsed / n as u32);
}

#[allow(dead_code)]
fn check<C: Parser<'static> + Clone>(input: &'static str) -> Duration {
    let start = Instant::now();
    let challenge = C::must_parse(input);
    std::hint::black_box(challenge.clone().part_one().to_string());
    std::hint::black_box(challenge.part_two().to_string());
    start.elapsed()
}

#[allow(dead_code)]
fn bench<C: Parser<'static> + Clone>(input: &'static str) {
    let challenge = C::must_parse(std::hint::black_box(input));
    std::hint::black_box(challenge.clone().part_one().to_string());
    std::hint::black_box(challenge.part_two().to_string());
}
