use std::{collections::HashMap, env, fs};

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).unwrap();

    println!("{}", puzzle_one(&file));
    println!("{}", puzzle_two(&file));
}

fn puzzle_one(input: &str) -> String {
    let (state_str, ops) = input.split_once("\n\n").unwrap();

    let mut state = parse_state(state_str);

    ops.lines().for_each(|line| {
        let reg = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let caps = reg.captures(line).unwrap();

        let amount = &caps[1].parse::<usize>().unwrap();
        let from_index = &caps[2].parse::<usize>().unwrap() - 1;
        let to_index = &caps[3].parse::<usize>().unwrap() - 1;

        let (moving, from_after) = state.get(&from_index).unwrap().split_at(*amount);
        let to_after = format!(
            "{}{}",
            moving.chars().rev().collect::<String>(),
            state.get(&to_index).unwrap(),
        );

        state.insert(from_index, from_after.into());
        state.insert(to_index, to_after);
    });

    let mut ans = String::new();
    for i in 0..state.len() {
        let value = state.get(&i).unwrap();
        let c = value.chars().next().unwrap();
        ans = format!("{ans}{c}");
    }
    ans
}

fn puzzle_two(input: &str) -> String {
    let (state_str, ops) = input.split_once("\n\n").unwrap();

    let mut state = parse_state(state_str);

    ops.lines().for_each(|line| {
        let reg = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let caps = reg.captures(line).unwrap();

        let amount = &caps[1].parse::<usize>().unwrap();
        let from_index = &caps[2].parse::<usize>().unwrap() - 1;
        let to_index = &caps[3].parse::<usize>().unwrap() - 1;

        let (moving, from_after) = state.get(&from_index).unwrap().split_at(*amount);
        let to_after = format!("{}{}", moving, state.get(&to_index).unwrap(),);

        state.insert(from_index, from_after.into());
        state.insert(to_index, to_after);
    });

    let mut ans = String::new();
    for i in 0..state.len() {
        let value = state.get(&i).unwrap();
        let c = value.chars().next().unwrap();
        ans = format!("{ans}{c}");
    }
    ans
}

fn parse_state(input: &str) -> HashMap<usize, String> {
    let mut map: HashMap<usize, String> = HashMap::new();

    input
        .as_bytes()
        .chunks(4)
        .enumerate()
        .for_each(|(index, letter)| {
            let column = index % 9;

            if letter[1] as char == ' ' || letter[1].is_ascii_digit() {
                return;
            }

            if let Some(s) = map.get(&column) {
                map.insert(column, format!("{}{}", s, letter[1] as char));
            } else {
                map.insert(column, format!("{}", letter[1] as char));
            };
        });

    map
}
