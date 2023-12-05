pub const EXPECTED_A: &str = "35";
pub const EXPECTED_B: &str = "46";

use std::ops::Range;

pub fn solve_a(input: String) -> String {
    let (seeds, mappings, _seed_ranges) = parse_input(input);

    let mut lowest_dest_number: Option<u64> = None;
    for seed in seeds {
        let mut current_source_name = "seed";
        let mut current_source_number = seed;

        for mapping in &mappings {
            if mapping.source_name == current_source_name {
                current_source_number = mapping.get_lowest_dest_number(current_source_number);
                current_source_name = &mapping.dest_name;
            }
        }
        match lowest_dest_number {
            Some(x) => lowest_dest_number = Some(x.min(current_source_number)),
            None => lowest_dest_number = Some(current_source_number),
        }
    }
    lowest_dest_number.unwrap().to_string()
}

pub fn solve_b(input: String) -> String {
    let (_seeds, mappings, seed_ranges) = parse_input(input);

    let mut lowest_dest_number: Option<u64> = None;
    for seeds in seed_ranges {
        //I mean this is super slow but just copy pasting solve_a() works fine
        for seed in seeds {
            let mut current_source_name = "seed";
            let mut current_source_number = seed;

            for mapping in &mappings {
                if mapping.source_name == current_source_name {
                    current_source_number = mapping.get_lowest_dest_number(current_source_number);
                    current_source_name = &mapping.dest_name;
                }
            }
            match lowest_dest_number {
                Some(x) => lowest_dest_number = Some(x.min(current_source_number)),
                None => lowest_dest_number = Some(current_source_number),
            }
        }
    }
    lowest_dest_number.unwrap().to_string()
}

struct MappingRange {
    source_range: Range<u64>,
    dest_range: Range<u64>,
}

pub struct Mapping {
    source_name: String,
    dest_name: String,
    ranges: Vec<MappingRange>,
}

impl Mapping {
    fn get_lowest_dest_number(&self, source_number: u64) -> u64 {
        let mut lowest_dest: Option<u64> = None;
        for range in &self.ranges {
            if range.source_range.contains(&source_number) {
                let offset = source_number - range.source_range.start;
                let dest_number = range.dest_range.start + offset;

                match lowest_dest {
                    Some(x) => lowest_dest = Some(x.min(dest_number)),
                    None => lowest_dest = Some(dest_number),
                }
            }
        }
        //"Any source numbers that aren't mapped correspond to the same destination number."
        match lowest_dest {
            Some(x) => x,
            None => source_number,
        }
    }
}

pub fn parse_input(input: String) -> (Vec<u64>, Vec<Mapping>, Vec<Range<u64>>) {
    let (input1, input2) = input.split_once("\n\n").unwrap();
    let seeds = input1
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut mappings: Vec<Mapping> = vec![];
    for str in input2.split("\n\n") {
        let mut mapping = Mapping {
            source_name: String::from(""),
            dest_name: String::from(""),
            ranges: vec![],
        };
        for (i, s) in str.split("\n").enumerate() {
            if i == 0 {
                let (source_name, dest_name) =
                    s.split_once(" ").unwrap().0.split_once("-to-").unwrap();
                mapping.source_name = source_name.to_string();
                mapping.dest_name = dest_name.to_string();
            } else {
                let mut source_range = Range { start: 0, end: 0 };
                let mut dest_range = Range { start: 0, end: 0 };
                for (i, num) in s.split_whitespace().enumerate() {
                    if i == 0 {
                        dest_range.start = num.parse().unwrap();
                    } else if i == 1 {
                        source_range.start = num.parse().unwrap();
                    } else if i == 2 {
                        let d = num.parse::<u64>().unwrap();
                        source_range.end = source_range.start + d;
                        dest_range.end = dest_range.start + d;
                    }
                }
                mapping.ranges.push(MappingRange {
                    source_range,
                    dest_range,
                })
            }
        }

        mappings.push(mapping);
    }

    //for part 2... I mean itertools has .chunks() or .tuples() etc...
    let mut seed_ranges: Vec<Range<u64>> = vec![];
    for (i, x) in seeds.iter().enumerate() {
        if i % 2 == 0 {
            seed_ranges.push(Range { start: *x, end: *x })
        } else {
            seed_ranges[i / 2].end += x;
        }
    }

    (seeds, mappings, seed_ranges)
}
