use std::time::Instant;

use day11::Solution;

fn main() {
    // let input = std::hint::black_box(include_str!("../input.txt"));
    let input = String::leak(std::fs::read_to_string("challenges/day11/input.txt").unwrap());

    let start = Instant::now();
    let challenge = Solution::parse(input);
    let p1 = challenge.clone().part_one();
    let p2 = challenge.part_two();
    println!("took: {:?}", start.elapsed());

    println!("\tAnswer to part one: {p1}");
    println!("\tAnswer to part two: {p2}");

    // aoc::check::<Solution>(input);
    // aoc_client::run_and_upload::<Solution>(input);
}
