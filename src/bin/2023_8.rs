use std::collections::HashMap;

fn main() {
    let input = include_str!("../../inputs/2023/8.txt").trim();
    let (instructions, map) = parse(input);
    println!("puzzle one: {}", puzzle_one(&instructions, &map, "AAA"));
    println!("puzzle two: {}", puzzle_two(&instructions, &map));
}

fn puzzle_one(instructions: &Vec<char>, map: &HashMap<&str, (&str, &str)>, find: &str) -> usize {
    let (mut steps, mut i) = (0, 0);
    let mut node = find;
    while !node.ends_with('Z') {
        let (l, r) = map.get(node).unwrap();
        node = if instructions[i] == 'L' { l } else { r };
        i = (i + 1) % instructions.len();
        steps += 1;
    }
    steps
}

fn puzzle_two(instructions: &Vec<char>, map: &HashMap<&str, (&str, &str)>) -> usize {
    let places: Vec<&str> = map
        .keys()
        .filter(|place| place.ends_with('A'))
        .copied()
        .collect();

    places
        .into_iter()
        .map(|find| puzzle_one(instructions, map, find))
        .reduce(lcd)
        .unwrap()
}

fn lcd(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn parse(input: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let instructions = input
        .lines()
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .chars()
        .collect();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    input.lines().skip(2).for_each(|line| {
        let (src, rest) = line.split_once(" = ").unwrap();

        let (a, b) = rest
            .strip_prefix('(')
            .and_then(|c| c.strip_suffix(')'))
            .and_then(|c| c.split_once(", "))
            .unwrap();

        map.insert(src, (a, b));
    });

    (instructions, map)
}
