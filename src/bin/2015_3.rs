use std::collections::HashSet;

fn main() {
    let input = include_str!("../inputs/2015/3.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    let mut houses: HashSet<String> = HashSet::new();
    houses.insert("0:0".to_string());
    let mut santa = (0, 0);
    for c in input.chars() {
        match c {
            'v' => santa.1 += 1,
            '^' => santa.1 -= 1,
            '<' => santa.0 -= 1,
            '>' => santa.0 += 1,
            _ => unreachable!(),
        };

        houses.insert(format!("{}:{}", santa.0, santa.1));
    }
    houses.len()
}
fn puzzle_two(input: &str) -> usize {
    let mut houses: HashSet<String> = HashSet::new();
    houses.insert("0:0".to_string());

    let mut santa = (0, 0);
    let mut robot = (0, 0);

    for (i, c) in input.chars().enumerate() {
        if i % 2 == 0 {
            match c {
                'v' => santa.1 += 1,
                '^' => santa.1 -= 1,
                '<' => santa.0 -= 1,
                '>' => santa.0 += 1,
                _ => unreachable!(),
            };
            houses.insert(format!("{}:{}", santa.0, santa.1));
        } else {
            match c {
                'v' => robot.1 += 1,
                '^' => robot.1 -= 1,
                '<' => robot.0 -= 1,
                '>' => robot.0 += 1,
                _ => unreachable!(),
            };
            houses.insert(format!("{}:{}", robot.0, robot.1));
        }
    }
    houses.len()
}
