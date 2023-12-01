pub const EXPECTED_A: &str = "142";
pub const EXPECTED_B: &str = "281";

pub fn solve_a(input: String) -> String {
    let v = input
        .lines()
        .map(|line| line.chars().filter(|c| c.is_ascii_digit()).collect())
        .collect::<Vec<Vec<char>>>();

    let mut sum = 0;
    for x in v {
        let a = x.first().unwrap();
        let b = x.last().unwrap_or(a);
        sum += [a.to_string(), b.to_string()]
            .join("")
            .parse::<i32>()
            .unwrap();
    }
    sum.to_string()
}

pub fn solve_b(input: String) -> String {
    let patterns: [(&str, usize); 18] = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];
    let v = input
        .lines()
        .map(|line| {
            let mut first: (Option<usize>, usize) = (None, 999);
            let mut last: (Option<usize>, usize) = (None, 0);

            for (str, x) in patterns {
                (first, last) = maybe_update(first, last, line, str, x);
            }

            let a = first.0.unwrap();
            let b = last.0.unwrap();

            let n = [a.to_string(), b.to_string()]
                .join("")
                .parse::<i32>()
                .unwrap();
            n
        })
        .collect::<Vec<i32>>();

    let mut sum = 0;
    for x in v {
        sum += x;
    }

    sum.to_string()
}

fn maybe_update(
    prev_first: (Option<usize>, usize),
    prev_last: (Option<usize>, usize),
    line: &str,
    str: &str,
    x: usize,
) -> ((Option<usize>, usize), (Option<usize>, usize)) {
    let first = match line.find(str) {
        Some(i) => {
            if i <= prev_first.1 {
                (Some(x), i)
            } else {
                prev_first
            }
        }
        None => prev_first,
    };
    let last = match line.rfind(str) {
        Some(i) => {
            if i >= prev_last.1 {
                (Some(x), i)
            } else {
                prev_last
            }
        }
        None => prev_last,
    };

    (first, last)
}
