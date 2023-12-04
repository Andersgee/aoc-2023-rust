pub const EXPECTED_A: &str = "142";
pub const EXPECTED_B: &str = "281";

pub fn solve_a(input: String) -> String {
    let lines = parse_input(input, true);

    let mut sum = 0;
    for line in lines {
        let a = line.numbers.first().unwrap();
        let b = line.numbers.last().unwrap_or(a);
        let number = format!("{a}{b}").parse::<u32>().unwrap();
        sum += number;
    }

    sum.to_string()
}

pub fn solve_b(input: String) -> String {
    let lines = parse_input(input, false);

    let mut sum = 0;
    for line in lines {
        let a = line.numbers.first().unwrap();
        let b = line.numbers.last().unwrap_or(a);
        let number = format!("{a}{b}").parse::<u32>().unwrap();
        sum += number;
    }

    sum.to_string()
}

struct Line {
    numbers: Vec<String>,
}

const PATTERNS: [(&str, &str); 18] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
    ("1", "1"),
    ("2", "2"),
    ("3", "3"),
    ("4", "4"),
    ("5", "5"),
    ("6", "6"),
    ("7", "7"),
    ("8", "8"),
    ("9", "9"),
];

fn parse_input(input: String, is_part1: bool) -> Vec<Line> {
    let mut lines: Vec<Line> = vec![];
    for str in input.lines() {
        let mut line = Line { numbers: vec![] };
        for x in 0..str.len() + 1 {
            for (pattern, val) in PATTERNS {
                if is_part1 && pattern != val {
                    continue;
                }
                if str[x..].starts_with(pattern) {
                    line.numbers.push(val.to_owned());
                }
            }
        }
        lines.push(line);
    }

    lines
}
