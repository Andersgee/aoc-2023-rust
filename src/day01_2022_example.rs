pub const EXPECTED_A: &str = "24000";
pub const EXPECTED_B: &str = "45000";

pub fn solve_a(input: String) -> String {
    let input = parse(input);
    let max = input.iter().max().unwrap();
    max.to_string()
}

pub fn solve_b(input: String) -> String {
    let mut input = parse(input);
    input.sort_by(|a, b| b.cmp(a));
    let sum = input.iter().take(3).sum::<u32>();
    sum.to_string()
}

fn parse(input: String) -> Vec<u32> {
    let elfs = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    elfs
}
