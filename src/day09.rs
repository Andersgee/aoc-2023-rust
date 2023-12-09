pub const EXPECTED_A: &str = "114";
pub const EXPECTED_B: &str = "2";

use std::collections::VecDeque;

pub fn solve_a(input: String) -> String {
    let lines = parse_input(input);

    let mut sum = 0;
    for numbers in lines {
        let mut sequences = vec![VecDeque::from(numbers)];
        while !(sequences.last().unwrap().iter().all(|x| *x == 0)) {
            sequences.push(diffsequence(sequences.last().unwrap()));
        }
        sequences.reverse();

        for i in 0..sequences.len() {
            if i == 0 {
                sequences[i].push_back(0);
            } else {
                let prev = sequences[i - 1].iter().last().unwrap().to_owned();
                let x = sequences[i].iter().last().unwrap().to_owned();
                sequences[i].push_back(prev + x);
            }
        }
        sum += sequences.last().unwrap().iter().last().unwrap();
    }

    sum.to_string()
}

pub fn solve_b(input: String) -> String {
    let lines = parse_input(input);

    let mut sum = 0;
    for numbers in lines {
        let mut sequences = vec![VecDeque::from(numbers)];
        while !(sequences.last().unwrap().iter().all(|x| *x == 0)) {
            sequences.push(diffsequence(sequences.last().unwrap()));
        }
        sequences.reverse();

        for i in 0..sequences.len() {
            if i == 0 {
                sequences[i].push_front(0);
            } else {
                let prev = sequences[i - 1][0].to_owned();
                let x = sequences[i][0].to_owned();
                sequences[i].push_front(x - prev);
            }
        }
        sum += sequences.last().unwrap()[0];
    }

    sum.to_string()
}

fn parse_input(input: String) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

fn diffsequence(numbers: &VecDeque<i64>) -> VecDeque<i64> {
    let mut diffs = VecDeque::new();
    for i in 0..numbers.len() - 1 {
        let diff = numbers[i + 1] - numbers[i];
        diffs.push_back(diff);
    }
    diffs
}
