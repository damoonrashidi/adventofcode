use std::{collections::HashMap, env, fs};

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).unwrap();

    println!("{}", puzzle_one(file));
}

fn puzzle_one(input: String) -> String {
    let (state_str, ops) = input.split_once("\n\n").unwrap();

    let state = parse_state(state_str);

    println!("{:?}", state);

    ops.lines().into_iter().for_each(|line| {
        let reg = Regex::new(r#"move (\d+) from (\d+) to (\d+)"#).unwrap();
        let caps = reg.captures(line).unwrap();
    });

    "".into()
}

fn parse_state(input: &str) -> HashMap<usize, Vec<char>> {
    let mut map: HashMap<usize, Vec<char>> = HashMap::new();

    input
        .as_bytes()
        .chunks(4)
        .into_iter()
        .enumerate()
        .for_each(|(index, letter)| {
            println!("{}{:?}", , letter[1] as char);
            let column = index % 3;

            if map.contains_key(&column) && letter[1] as char != ' ' {
                let v = map.get_mut(&column).unwrap();
                v.insert(index, letter[1] as char);
            }
        });

    println!("{:?}", map);

    map
}
