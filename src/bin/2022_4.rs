use regex::Regex;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).unwrap();

    println!("{}", puzzle_one(&file));
    println!("{}", puzzle_two(&file));
}

fn puzzle_one(input: &String) -> usize {
    input
        .lines()
        .map(|line| {
            let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
            let caps = re.captures(line).unwrap();
            let first = &caps[1].parse::<u32>().unwrap()..&caps[2].parse().unwrap();
            let second = &caps[3].parse::<u32>().unwrap()..&caps[4].parse().unwrap();

            (first.start <= second.start && first.end >= second.end)
                || second.start <= first.start && second.end >= first.end
        })
        .filter(|l| *l)
        .collect::<Vec<bool>>()
        .len()
}

fn puzzle_two(input: &String) -> usize {
    input
        .lines()
        .map(|line| {
            let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
            let caps = re.captures(line).unwrap();
            let first = &caps[1].parse::<u32>().unwrap()..&caps[2].parse().unwrap();
            let second = &caps[3].parse::<u32>().unwrap()..&caps[4].parse().unwrap();

            (first.start <= second.start && first.end >= second.start)
                || (second.start <= first.start && second.end >= first.start)
        })
        .filter(|l| *l)
        .collect::<Vec<bool>>()
        .len()
}
