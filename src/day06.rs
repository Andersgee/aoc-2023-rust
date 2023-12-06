pub const EXPECTED_A: &str = "288";
pub const EXPECTED_B: &str = "71503";

use std::iter::zip;

pub fn solve_a(input: String) -> String {
    let (races, _race) = parse_input(input);

    let mut counts = vec![];
    for race in races {
        let mut n_ways_to_beat_record = 0;
        for i in 0..race.lasts_ms {
            let speed = i;
            let remaining_ms = race.lasts_ms - i;
            let dist_ms = speed * remaining_ms;
            if dist_ms > race.record_distance_mm {
                n_ways_to_beat_record += 1
            }
        }
        counts.push(n_ways_to_beat_record)
    }
    counts.iter().product::<u64>().to_string()
}

pub fn solve_b(input: String) -> String {
    let (_races, race) = parse_input(input);

    let mut n_ways_to_beat_record = 0;
    for i in 0..race.lasts_ms {
        let speed = i;
        let remaining_ms = race.lasts_ms - i;
        let dist_ms = speed * remaining_ms;
        if dist_ms > race.record_distance_mm {
            n_ways_to_beat_record += 1
        }
    }

    n_ways_to_beat_record.to_string()
}

struct Race {
    lasts_ms: u64,
    record_distance_mm: u64,
}

fn parse_input(input: String) -> (Vec<Race>, Race) {
    let lines = input.lines().collect::<Vec<&str>>();
    let times = lines[0]
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .collect::<Vec<&str>>();

    let distances = lines[1]
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .collect::<Vec<&str>>();

    let races = zip(&times, &distances)
        .map(|(time, distance)| Race {
            lasts_ms: time.parse::<u64>().unwrap(),
            record_distance_mm: distance.parse::<u64>().unwrap(),
        })
        .collect::<Vec<Race>>();

    let race = Race {
        lasts_ms: times.join("").parse::<u64>().unwrap(),
        record_distance_mm: distances.join("").parse::<u64>().unwrap(),
    };

    (races, race)
}
