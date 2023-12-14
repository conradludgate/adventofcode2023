use day14::Solution;

fn main() {
    let input = include_str!("../input.txt");
    // aoc::check::<Solution>(input);
    aoc_client::run_and_upload::<Solution>(env!("CARGO_PKG_NAME"), input);
}
