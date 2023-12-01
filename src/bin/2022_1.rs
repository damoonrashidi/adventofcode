use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).unwrap();

    println!("{}", puzzle_one(&file));
}

fn puzzle_one(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|inventory| {
            inventory
                .lines()
                .map(|line| line.parse::<usize>().unwrap())
                .sum()
        })
        .max()
        .unwrap()
}
