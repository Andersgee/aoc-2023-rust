pub const EXPECTED_A: &str = "6440";
pub const EXPECTED_B: &str = "5905";

use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap, iter::zip};

pub fn solve_a(input: String) -> String {
    let (hands, _) = parse_input(input);

    let sum = hands.iter().sorted().enumerate().fold(0, |acc, (i, hand)| {
        let rank = i + 1;
        let strength = hand.bid * rank;
        acc + strength
    });
    sum.to_string()
}

pub fn solve_b(input: String) -> String {
    let (_, hands) = parse_input(input);

    let sum = hands.iter().sorted().enumerate().fold(0, |acc, (i, hand)| {
        let rank = i + 1;
        let strength = hand.bid * rank;
        acc + strength
    });
    sum.to_string()
}

fn parse_input(input: String) -> (Vec<Hand>, Vec<HandWithJokers>) {
    let hands = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(" ").unwrap();
            let cards = a.chars().collect();
            let bid = b.parse().unwrap();
            let hand_type = parse_hand_type(&cards, false);
            Hand {
                cards,
                bid,
                hand_type,
            }
        })
        .collect::<Vec<Hand>>();
    let hands_with_jokers = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(" ").unwrap();
            let cards = a.chars().collect();
            let bid = b.parse().unwrap();
            let hand_type = parse_hand_type(&cards, true);
            HandWithJokers {
                cards,
                bid,
                hand_type,
            }
        })
        .collect::<Vec<HandWithJokers>>();
    (hands, hands_with_jokers)
}

#[derive(Eq)]
struct Hand {
    cards: Vec<char>,
    bid: usize,
    hand_type: Handtype,
}

#[derive(Eq)]
struct HandWithJokers {
    cards: Vec<char>,
    bid: usize,
    hand_type: Handtype,
}

//implement required trait for .sort() etc
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            for (card, other_card) in zip(&self.cards, &other.cards) {
                let my_index = LETTERS.iter().position(|c| c == card).unwrap();
                let other_index = LETTERS.iter().position(|c| c == other_card).unwrap();

                if my_index > other_index {
                    return Ordering::Greater;
                } else if other_index > my_index {
                    return Ordering::Less;
                }
            }

            Ordering::Equal
        } else if self.hand_type > other.hand_type {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl Ord for HandWithJokers {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            for (card, other_card) in zip(&self.cards, &other.cards) {
                //only for part2: 'J' considered worst., else identical
                if card == &'J' && other_card != &'J' {
                    return Ordering::Less;
                } else if card != &'J' && other_card == &'J' {
                    return Ordering::Greater;
                }

                let my_index = LETTERS.iter().position(|c| c == card).unwrap();
                let other_index = LETTERS.iter().position(|c| c == other_card).unwrap();

                if my_index > other_index {
                    return Ordering::Greater;
                } else if other_index > my_index {
                    return Ordering::Less;
                }
            }

            Ordering::Equal
        } else if self.hand_type > other.hand_type {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
#[repr(u8)]
enum Handtype {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

const LETTERS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

type Counts = HashMap<char, u8>;

fn get_hand_type(counts: Counts) -> Handtype {
    match counts.len() {
        1 => Handtype::FiveOfAKind,
        2 => {
            let has_4_same = counts
                .iter()
                .fold(false, |acc, (_c, n)| if n >= &4 { true } else { acc });
            match has_4_same {
                true => Handtype::FourOfAKind,
                false => Handtype::FullHouse,
            }
        }
        3 => {
            let has_3_same = counts
                .iter()
                .fold(false, |acc, (_c, n)| if n >= &3 { true } else { acc });
            match has_3_same {
                true => Handtype::ThreeOfAKind,
                false => Handtype::TwoPair,
            }
        }
        4 => Handtype::OnePair,
        5 => Handtype::HighCard,
        _ => panic!(),
    }
}

fn parse_hand_type(cards: &Vec<char>, is_part2: bool) -> Handtype {
    let mut counts: Counts = HashMap::new();
    for letter in LETTERS {
        let n = cards
            .iter()
            .fold(0, |acc, c| if c == &letter { acc + 1 } else { acc });
        if n > 0 {
            counts.insert(letter, n);
        }
    }

    match is_part2 {
        false => get_hand_type(counts), //part1
        true => {
            //part2 treats 'J' as jokers aka whatever makes the hands type strongest
            //except when breaking ties; then 'J' is consideres weaker than even '2'
            let n_jokers = counts.get(&'J');
            match n_jokers {
                None => get_hand_type(counts),
                Some(n) => {
                    if n == &5 {
                        Handtype::FiveOfAKind
                    } else {
                        //find best hand type, it will always be best to use all jokers as one type of card
                        //eg for "A7 JJJ" then "A7 AAA" or "A7 777" will always be better than other permutations
                        let mut counts_without_jokers = counts.clone();
                        counts_without_jokers.remove(&'J');

                        let mut possible_hand_types: Vec<Handtype> = vec![];
                        for non_joker_char in counts_without_jokers.keys() {
                            let mut possible_counts = counts_without_jokers.clone();
                            possible_counts
                                .entry(*non_joker_char)
                                .and_modify(|e| *e += n);

                            possible_hand_types.push(get_hand_type(possible_counts));
                        }

                        *possible_hand_types.iter().sorted().last().unwrap()
                    }
                }
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl PartialOrd for HandWithJokers {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HandWithJokers {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
