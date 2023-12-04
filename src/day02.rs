pub const EXPECTED_A: &str = "8";
pub const EXPECTED_B: &str = "2286";

pub fn solve_a(input: String) -> String {
    let games = parse_input(input);

    let mut sum = 0;
    for game in games {
        let mut possible = true;
        for hand in game.hands {
            if hand.red > 12 || hand.green > 13 || hand.blue > 14 {
                possible = false
            }
        }
        if possible {
            sum += game.id;
        }
    }
    sum.to_string()
}

pub fn solve_b(input: String) -> String {
    let games = parse_input(input);

    let mut sum = 0;
    for game in games {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for hand in game.hands {
            r = r.max(hand.red);
            g = g.max(hand.green);
            b = b.max(hand.blue);
        }
        let power = r * g * b;

        sum += power;
    }
    sum.to_string()
}

struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    hands: Vec<Hand>,
}

fn parse_input(input: String) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let (gameid, game) = line.split_once(": ").unwrap();

            let id = gameid.split_once(" ").unwrap().1.parse::<u32>().unwrap();
            let hands = game.split("; ").collect::<Vec<&str>>();

            let mut draws: Vec<Hand> = vec![];
            for hand in hands {
                let list = hand.split(", ").collect::<Vec<&str>>();
                let mut draw = Hand {
                    red: 0,
                    green: 0,
                    blue: 0,
                };
                for item in list {
                    let (qty, color) = item.split_once(" ").unwrap();
                    let n = qty.parse::<u32>().unwrap();

                    if color.eq("red") {
                        draw.red += n;
                    } else if color.eq("green") {
                        draw.green += n;
                    } else if color.eq("blue") {
                        draw.blue += n;
                    }
                }
                draws.push(draw);
            }

            Game { id, hands: draws }
        })
        .collect()
}
