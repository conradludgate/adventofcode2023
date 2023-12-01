use aoc::{Challenge, Parser};
use day01::Solution;
use divan::black_box;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn parse(bencher: divan::Bencher) {
    bencher
        .counter(divan::counter::BytesCount::new(INPUT.len()))
        .bench(|| Solution::parse(black_box(INPUT)))
}

#[divan::bench]
fn part_one(bencher: divan::Bencher) {
    let challenge = Solution::parse(INPUT).unwrap().1;
    bencher
        .with_inputs(|| challenge.clone())
        .bench_values(Solution::part_one)
}

#[divan::bench]
fn part_two(bencher: divan::Bencher) {
    let challenge = Solution::parse(INPUT).unwrap().1;
    bencher
        .with_inputs(|| challenge.clone())
        .bench_values(Solution::part_two)
}
