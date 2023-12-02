pub const EXPECTED_A: &str = "8";
pub const EXPECTED_B: &str = "2286";

pub fn solve_a(input: String) -> String {
    let v = parse_input(input);

    let mut sum = 0;
    for (id, draws) in v {
        let mut possible = true;
        for draw in draws {
            if draw.red > 12 || draw.green > 13 || draw.blue > 14 {
                possible = false
            }
        }
        if possible {
            sum += id;
        }
    }
    sum.to_string()
}

pub fn solve_b(input: String) -> String {
    let v = parse_input(input);

    let mut sum = 0;
    for (id, draws) in v {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for draw in draws {
            r = r.max(draw.red);
            g = g.max(draw.green);
            b = b.max(draw.blue);
        }
        let power = r * g * b;

        sum += power;
    }
    sum.to_string()
}

#[derive(Debug)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

fn parse_input(input: String) -> Vec<(u32, Vec<Draw>)> {
    input
        .lines()
        .map(|line| {
            let (gameid, game) = line.split_once(": ").unwrap();

            let id = gameid.split_once(" ").unwrap().1.parse::<u32>().unwrap();
            let hands = game.split("; ").collect::<Vec<&str>>();

            let mut draws: Vec<Draw> = vec![];
            for hand in hands {
                let list = hand.split(", ").collect::<Vec<&str>>();
                let mut draw = Draw {
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
            (id, draws)
        })
        .collect()
}
