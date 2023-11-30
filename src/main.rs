use aoc::day::{solve_a, solve_b};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("solve_a: {}", solve_a(input));

    let input = fs::read_to_string("input.txt").unwrap();
    println!("solve_b: {}", solve_b(input));
}
