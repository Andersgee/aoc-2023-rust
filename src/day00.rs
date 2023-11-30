//copy paste this to for example day01.rs
pub const EXPECTED_A: &str = "";
pub const EXPECTED_B: &str = "";

pub fn solve_a(input: String) -> String {
    "".to_string()
}

pub fn solve_b(input: String) -> String {
    "".to_string()
}

fn some_util(x: u64) -> u64 {
    x * 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_util_works() {
        let result = some_util(9);
        assert_eq!(result, 27);
    }
}
