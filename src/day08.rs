pub const EXPECTED_A: &str = "6";
pub const EXPECTED_B: &str = "6";

use std::collections::HashMap;

pub fn solve_a(input: String) -> String {
    let (cmds, instruction_map) = parse_input(input);
    let starting_node = "AAA".to_string();
    let ending_node_ends_with = "ZZZ";

    let count = step_count(
        &starting_node,
        ending_node_ends_with,
        &cmds,
        &instruction_map,
    );
    count.to_string()
}

pub fn solve_b(input: String) -> String {
    let (cmds, instruction_map) = parse_input(input);

    let starting_nodes: Vec<&String> = instruction_map
        .iter()
        .filter(|(node, _instr)| node.ends_with("A"))
        .map(|x| x.0)
        .collect();
    let ending_node_ends_with = "Z";

    let counts: Vec<u64> = starting_nodes
        .iter()
        .map(|s| step_count(*s, ending_node_ends_with, &cmds, &instruction_map))
        .collect();
    lcm(counts).to_string()
}

fn step_count(
    starting_node: &String,
    ending_node_endswith: &str,
    cmds: &Vec<char>,
    instruction_map: &HashMap<String, Instruction>,
) -> u64 {
    let mut steps = 0;
    let mut node = starting_node.clone();
    loop {
        for cmd in cmds {
            let instr = instruction_map.get(&node).unwrap();
            if *cmd == 'R' {
                node = instr.right.to_owned();
            } else {
                node = instr.left.to_owned();
            }
            steps += 1;
            if node.ends_with(ending_node_endswith) {
                return steps;
            }
        }
    }
}

struct Instruction {
    left: String,
    right: String,
}

fn parse_input(input: String) -> (Vec<char>, HashMap<String, Instruction>) {
    let (a, b) = input.split_once("\n\n").unwrap();

    let instr = b
        .lines()
        .map(|line| {
            let (label, choices) = line.split_once(" = ").unwrap();
            //let x = choices.split_once(" ,").unwrap();
            let (choice1, choice2) = choices[1..choices.len() - 1].split_once(", ").unwrap();

            (
                label.to_owned(),
                Instruction {
                    left: choice1.to_owned(),
                    right: choice2.to_owned(),
                },
            )
        })
        .collect();
    (a.chars().collect(), instr)
}

//https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
fn lcm(nums: Vec<u64>) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(nums[1..].to_vec());
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
