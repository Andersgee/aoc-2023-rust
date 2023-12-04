pub const EXPECTED_A: &str = "13";
pub const EXPECTED_B: &str = "30";

use std::collections::{HashMap, VecDeque};

pub fn solve_a(input: String) -> String {
    let cards = parse_input(input);

    let mut sum = 0;
    for card in cards {
        let mut s = 0;
        for x in card.my_numbers {
            if card.winning_numbers.contains(&x) {
                if s == 0 {
                    s = 1;
                } else {
                    s *= 2;
                }
            }
        }
        sum += s;
    }

    sum.to_string()
}

pub fn solve_b(input: String) -> String {
    let cards = parse_input(input);

    let mut que: VecDeque<&Card> = VecDeque::new();
    let mut count: HashMap<usize, u32> = HashMap::new();
    for original_card in &cards {
        que.push_back(&original_card);
        while !que.is_empty() {
            let card = que.pop_front().unwrap();
            count.entry(card.id).and_modify(|v| *v += 1).or_insert(1);

            let won_cards = card.won_cards(&cards);
            for c in won_cards {
                que.push_back(c);
            }
        }
    }

    let s = count.iter().fold(0, |acc, x| acc + x.1);
    s.to_string()
}

#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    my_numbers: Vec<usize>,
}

impl Card {
    fn won_card_ids(&self) -> Vec<usize> {
        let mut n_matchhing_numbers = 0;
        for x in &self.my_numbers {
            let a = &self.winning_numbers.contains(x);
            if *a {
                n_matchhing_numbers += 1;
            }
        }

        (0..n_matchhing_numbers).map(|i| &self.id + i).collect()
    }

    fn won_cards<'a>(&'a self, cards: &'a Vec<Card>) -> Vec<&Card> {
        let won_card_ids = &self.won_card_ids();

        let b = won_card_ids
            .iter()
            .map(|id| cards.get(*id).unwrap())
            .collect();
        b
    }
}

fn parse_input(input: String) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(": ").unwrap();
            let card_id = left.split_whitespace().collect::<Vec<&str>>()[1];
            let id = card_id.parse::<usize>().unwrap();
            let (list1, list2) = right.split_once(" | ").unwrap();
            let l1 = list1
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let l2 = list2
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            Card {
                id,
                winning_numbers: l1,
                my_numbers: l2,
            }
        })
        .collect()
}
