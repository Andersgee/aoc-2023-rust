pub const EXPECTED_A: &str = "4361";
pub const EXPECTED_B: &str = "467835";

use std::collections::{HashMap, HashSet};

pub fn solve_a(input: String) -> String {
    let map = parse_input(input);
    let numbers = get_numbers(&map);

    let mut sum = 0;
    for number in numbers {
        let mut has_adjacent_symbol = false;
        for point in number.points {
            for d in ADJACENT_DELTA {
                let adjacent_point = Point {
                    x: point.x + d.0,
                    y: point.y + d.1,
                };
                match map.get(&adjacent_point) {
                    None => {}
                    Some(letter) => match letter {
                        Letter::Number(_) => {}
                        Letter::Symbol(_) => {
                            has_adjacent_symbol = true;
                        }
                    },
                }
            }
        }

        if has_adjacent_symbol {
            sum += number.value;
        }
    }

    sum.to_string()
}

pub fn solve_b(input: String) -> String {
    let map = parse_input(input);
    let numbers = get_numbers(&map);

    let stars = &map
        .iter()
        .filter(|(_point, letter)| match letter {
            Letter::Number(_) => false,
            Letter::Symbol(char) => char == &'*',
        })
        .collect::<Vec<(&Point, &Letter)>>();

    let mut sum = 0;
    for star in stars {
        let mut adjacent_numbers: HashSet<&Number> = HashSet::new();

        for d in ADJACENT_DELTA {
            let adjacent_point = Point {
                x: star.0.x + d.0,
                y: star.0.y + d.1,
            };

            for number in &numbers {
                if number.points.contains(&adjacent_point) {
                    adjacent_numbers.insert(number);
                }
            }
        }

        if adjacent_numbers.len() == 2 {
            let gear_ratio: i32 = adjacent_numbers.iter().map(|number| number.value).product();
            sum += gear_ratio;
        }
    }

    sum.to_string()
}

#[derive(Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

enum Letter {
    Number(char),
    Symbol(char),
}

#[derive(Hash, PartialEq, Eq)]
struct Number {
    value: i32,
    points: Vec<Point>,
}

const ADJACENT_DELTA: [(i32, i32); 8] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (-1, 1),
    (-1, -1),
    (1, -1),
];

fn get_numbers(map: &HashMap<Point, Letter>) -> HashSet<Number> {
    let mut numbers: HashSet<Number> = HashSet::new();
    for (point, letter) in map {
        match letter {
            Letter::Symbol(_) => {}
            Letter::Number(_) => {
                let y = point.y;

                //find where enumber starts
                let mut x_start = point.x;
                for x in (0..point.x).rev() {
                    match map.get(&Point { x, y }) {
                        None => break,
                        Some(letter) => match letter {
                            Letter::Symbol(_) => break,
                            Letter::Number(_) => {
                                x_start = x;
                            }
                        },
                    }
                }

                //find where number ends
                let mut x_end = point.x;
                for x in point.x.. {
                    match map.get(&Point { x, y }) {
                        None => break,
                        Some(letter) => match letter {
                            Letter::Symbol(_) => break,
                            Letter::Number(_) => {
                                x_end = x;
                            }
                        },
                    }
                }

                //combine chars and parse it to number
                let value = (x_start..x_end + 1)
                    .map(|x| {
                        let r = map.get(&Point { x, y }).unwrap();
                        match r {
                            Letter::Symbol(_) => panic!(),
                            Letter::Number(c) => c,
                        }
                    })
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();

                //also keep the points this number spans
                let points = (x_start..x_end + 1)
                    .map(|x| Point { x, y })
                    .collect::<Vec<Point>>();

                numbers.insert(Number { value, points });
            }
        }
    }

    numbers
}

fn parse_input(input: String) -> HashMap<Point, Letter> {
    let mut map = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let is_dot = c == '.';
            let is_digit = c.is_ascii_digit();
            let point = Point {
                x: x.try_into().unwrap(),
                y: y.try_into().unwrap(),
            };
            if is_dot {
                continue;
            } else if is_digit {
                map.insert(point, Letter::Number(c));
            } else {
                map.insert(point, Letter::Symbol(c));
            };
        }
    }
    map
}
