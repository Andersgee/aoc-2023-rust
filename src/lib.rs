pub mod day00;
pub use day00 as day;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn solve_a_works() {
        let input = fs::read_to_string("input_test.txt").unwrap();
        let result = day::solve_a(input);
        assert_eq!(result, day::EXPECTED_A);
    }

    #[test]
    fn solve_b_works() {
        let input = fs::read_to_string("input_test.txt").unwrap();
        let result = day::solve_b(input);
        assert_eq!(result, day::EXPECTED_B);
    }
}
