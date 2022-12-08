use std::{collections::HashSet, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).unwrap();

    println!("{}", puzzle_one(&file));
    println!("{}", puzzle_two(&file));
}

fn puzzle_one(input: &String) -> u32 {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(first, second)| {
            let a: HashSet<u8> = first.bytes().into_iter().collect();
            let b: HashSet<u8> = second.bytes().into_iter().collect();

            a.intersection(&b).map(|item| prio(*item)).sum()
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum()
}

fn puzzle_two(input: &String) -> u32 {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|lines| {
            let a: HashSet<u8> = lines[0].bytes().into_iter().collect();
            let b: HashSet<u8> = lines[1].bytes().into_iter().collect();
            let c: HashSet<u8> = lines[2].bytes().into_iter().collect();

            a.intersection(&b)
                .cloned()
                .collect::<HashSet<u8>>()
                .intersection(&c)
                .map(|item| prio(*item))
                .sum()
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum()
}

fn prio(item: u8) -> u32 {
    match &item {
        b'A'..=b'Z' => item as u32 - b'A' as u32 + 27,
        b'a'..=b'z' => item as u32 - b'a' as u32 + 1,
        _ => unreachable!(),
    }
}
