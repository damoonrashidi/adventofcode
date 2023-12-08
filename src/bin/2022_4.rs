use regex::Regex;

fn main() {
    let input = include_str!("../../inputs/2022/4.txt");

    println!("{}", puzzle_one(input));
    println!("{}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
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

fn puzzle_two(input: &str) -> usize {
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
