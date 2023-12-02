use regex::Regex;

fn main() {
    let input = include_str!("../inputs/2023_2.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    input.lines().filter_map(id_if_possible).sum()
}

fn puzzle_two(input: &str) -> usize {
    input.lines().map(power_of_min).sum()
}

fn id_if_possible(line: &str) -> Option<usize> {
    let color_reg = Regex::new(r"(?P<amount>[\d]+) (?P<color>blue|green|red)").unwrap();
    let id_reg = Regex::new(r"Game (?P<id>\d+):").unwrap();

    let id: usize = id_reg
        .captures(line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();

    for cap in color_reg.captures_iter(line) {
        let amount: usize = cap.name("amount").unwrap().as_str().parse().unwrap();
        let color = cap.name("color").unwrap().as_str();

        let valid = match color {
            "red" => amount <= 12,
            "green" => amount <= 13,
            "blue" => amount <= 14,
            _ => unreachable!(),
        };

        if !valid {
            return None;
        }
    }

    Some(id)
}

fn power_of_min(line: &str) -> usize {
    let color_reg = Regex::new(r"(?P<amount>[\d]+) (?P<color>blue|green|red)").unwrap();

    let mut max_r = 1;
    let mut max_g = 1;
    let mut max_b = 1;

    for cap in color_reg.captures_iter(line) {
        let amount: usize = cap.name("amount").unwrap().as_str().parse().unwrap();
        let color = cap.name("color").unwrap().as_str();

        match color {
            "red" => {
                if amount > max_r {
                    max_r = amount;
                }
            }
            "green" => {
                if amount > max_g {
                    max_g = amount;
                }
            }
            "blue" => {
                if amount > max_b {
                    max_b = amount;
                }
            }
            _ => unreachable!(),
        };
    }
    max_r * max_g * max_b
}
